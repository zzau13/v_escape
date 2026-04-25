#![doc = include_str!("../README.md")]
use std::{
    collections::BTreeMap,
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
    str,
};

use proc_macro2::{Ident, Span, TokenStream};
use serde::Serialize;
use tests::build_tests;
use toml::Value;

use clap::Parser;

use v_escape_codegen_base::generate as generate_base;
mod tests;

fn ident(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}

/// V_escape codegen - A tool for generating escape functions
#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    about = "Generate escape functions from template files",
    long_about = "A tool for generating SIMD-optimized escape functions from template files. 
    
Creates a new crate with escape_fmt and escape_string functions based on character mappings defined in src/_lib.rs.

Example usage:
  mkdir my_escape
  cd my_escape
  cargo init --lib
  cat <<EOF > src/_lib.rs
  new!(
      '&' -> \"&amp;\",
      '/' -> \"&#x2f;\",
      '<' -> \"&lt;\",
      '>' -> \"&gt;\",
      '\"' -> \"&quot;\",
      '\\'' -> \"&#x27;\"
  );
  EOF
  v_escape-codegen -i .",
    after_help = "For more information, see: https://github.com/zzau13/v_escape"
)]
struct Args {
    /// Input directory containing the crate to generate
    #[clap(short, long, default_value = "./", value_name = "DIR")]
    pub input_dir: PathBuf,
}

#[derive(Serialize)]
struct Dep {
    workspace: bool,
}

fn generate(dir: impl AsRef<Path>) -> anyhow::Result<()> {
    _generate(dir.as_ref())
}

fn read_cargo(p: &Path) -> anyhow::Result<(Value, String)> {
    let cargo_src =
        fs::read_to_string(p).map_err(|e| anyhow::anyhow!("Failed to read Cargo.toml: {}", e))?;
    let mut cargo_value: Value = toml::from_str(&cargo_src)
        .map_err(|e| anyhow::anyhow!("Failed to parse Cargo.toml: {}", e))?;

    let doc: Value = toml::from_str(
        r"
    [docs.rs]
    all-features = true
    ",
    )
    .map_err(|e| anyhow::anyhow!("Failed to parse TOML: {}", e))?;

    let cargo_mut = cargo_value
        .as_table_mut()
        .ok_or_else(|| anyhow::anyhow!("Expected a table for cargo_value"))?;

    cargo_mut
        .get_mut("package")
        .ok_or_else(|| anyhow::anyhow!("Expected a package section in Cargo.toml"))?
        .as_table_mut()
        .ok_or_else(|| anyhow::anyhow!("Expected a table for package"))?
        .insert("metadata".to_string(), doc);

    let mut features = BTreeMap::new();
    features.insert("default", vec!["std", "string", "fmt", "bytes"]);
    features.insert("std", vec!["v_escape-base/std", "alloc"]);
    features.insert("alloc", vec!["v_escape-base/alloc"]);
    features.insert("string", vec!["v_escape-base/string"]);
    features.insert("fmt", vec!["v_escape-base/fmt"]);
    features.insert("bytes", vec!["v_escape-base/bytes"]);

    cargo_mut.insert("features".into(), Value::from(features));

    if !cargo_mut.contains_key("dependencies") {
        cargo_mut.insert(
            "dependencies".into(),
            Value::from(BTreeMap::<String, Value>::new()),
        );
    }
    let dependencies = cargo_mut
        .get_mut("dependencies")
        .ok_or_else(|| anyhow::anyhow!("Expected a table for dependencies"))?;
    dependencies
        .as_table_mut()
        .ok_or_else(|| anyhow::anyhow!("Expected a table for dependencies"))?
        .insert(
            "v_escape-base".into(),
            Value::try_from(Dep { workspace: true })?,
        );

    let package_name = cargo_value
        .as_table()
        .ok_or_else(|| anyhow::anyhow!("Expected a table for cargo_value"))?
        .get("package")
        .ok_or_else(|| anyhow::anyhow!("Expected a package section in Cargo.toml"))?
        .as_table()
        .ok_or_else(|| anyhow::anyhow!("Expected a table for package"))?
        .get("name")
        .ok_or_else(|| anyhow::anyhow!("Expected a name in package section"))?
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Expected a name as str"))?;

    let package_name = package_name.to_string();
    Ok((cargo_value, package_name))
}

fn _generate(dir: &Path) -> anyhow::Result<()> {
    if !dir.is_dir() {
        anyhow::bail!("input_dir should be a directory");
    }

    // Modify Cargo.toml
    // TODO: should use a pretty toml
    let cargo = dir.join("Cargo.toml");
    let (cargo_value, name) = read_cargo(&cargo)?;

    // Check directories
    let src = dir.join("src");
    let test = dir.join("tests");
    if !test.exists() {
        fs::create_dir(&test)?;
    }
    // Read template
    let template = src.join("_lib.rs");
    let template_src = fs::read_to_string(&template)?;

    // Generate code
    let (code, mappings) = generate_base(
        template_src
            .parse::<TokenStream>()
            .map_err(|e| anyhow::anyhow!("Failed to parse template source: {}", e))?,
        "v_escape_base",
    )?;

    // Prettify code
    let code_pretty = prettyplease::unparse(
        &syn::parse2(code)
            .map_err(|e| anyhow::anyhow!("Failed to parse code to TokenStream: {}", e))?,
    );

    // Build header and module-level documentation describing the generated module.
    let escapes_bytes: Vec<u8> = mappings.iter().map(|(c, _)| *c).collect();
    let escapes = String::from_utf8(escapes_bytes.clone())
        .map_err(|e| anyhow::anyhow!("escape characters must be valid UTF-8: {}", e))?;
    let escaped: String = mappings.iter().map(|(_, q)| q.as_str()).collect();
    let module_doc = render_module_doc(&name, &mappings);
    let head = format!(
        "//! autogenerated by v_escape_codegen@{version}\n{module_doc}",
        version = env!("CARGO_PKG_VERSION"),
    );

    // Generate tests
    let code_test = build_tests(&ident(&name), &escapes, &escaped);
    let code_test_pretty = prettyplease::unparse(
        &syn::parse2(code_test)
            .map_err(|e| anyhow::anyhow!("Failed to parse code to TokenStream: {}", e))?,
    );

    // Write files
    fs::write(&cargo, toml::to_string(&cargo_value)?)?;
    fs::write(src.join("lib.rs"), head + &code_pretty)?;
    fs::write(
        test.join("lib.rs"),
        format!(
            "//! autogenerated by v_escape_codegen@{}\n{}",
            env!("CARGO_PKG_VERSION"),
            code_test_pretty
        ),
    )?;

    Ok(())
}

/// Render an inner-doc (`//!`) block describing the generated escape module.
///
/// The output is meant to be prepended to the `lib.rs` file produced by codegen so
/// that `cargo doc` / docs.rs renders an explanation of what the crate does and
/// which characters get rewritten.
fn render_module_doc(crate_name: &str, mappings: &[(u8, String)]) -> String {
    let mut out = String::new();
    out.push_str("//!\n");
    out.push_str(&format!("//! # `{crate_name}`\n"));
    out.push_str("//!\n");
    out.push_str(
        "//! Autogenerated escape crate produced by\n\
         //! [`v_escape_codegen`](https://crates.io/crates/v_escape_codegen) on top of the\n\
         //! [`v_escape-base`](https://crates.io/crates/v_escape-base) runtime.\n",
    );
    out.push_str("//!\n");
    out.push_str("//! ## Behavior\n");
    out.push_str("//!\n");
    out.push_str(
        "//! Each call rewrites the characters listed in the table below into their\n\
         //! replacement string; every other byte of the input is forwarded verbatim.\n\
         //! All public entry points take a `&str` (UTF-8 guaranteed at the type level),\n\
         //! so they cannot be used to construct invalid UTF-8.\n",
    );
    out.push_str("//!\n");
    out.push_str("//! ## Escape table\n");
    out.push_str("//!\n");
    out.push_str("//! | Byte (hex) | Source | Replacement |\n");
    out.push_str("//! | ---------- | ------ | ----------- |\n");
    for (b, q) in mappings {
        out.push_str(&format!(
            "//! | `0x{:02X}` | {} | `{}` |\n",
            b,
            render_source_byte(*b),
            escape_md_inline_code(q),
        ));
    }
    out.push_str("//!\n");
    out.push_str("//! ## Public API\n");
    out.push_str("//!\n");
    out.push_str(
        "//! The following functions are emitted, gated by their respective Cargo\n\
         //! features (all enabled by default):\n",
    );
    out.push_str("//!\n");
    out.push_str(
        "//! | Function | Feature | Signature |\n\
         //! | -------- | ------- | --------- |\n\
         //! | `escape_string` | `string` | `fn(&str, &mut String)` |\n\
         //! | `escape_bytes`  | `bytes`  | `fn(&str, &mut Vec<u8>)` |\n\
         //! | `escape_fmt`    | `fmt`    | `fn(&str) -> impl Display + '_` |\n",
    );
    out.push_str("//!\n");
    out.push_str(
        "//! At runtime the implementation dispatches to the best SIMD backend\n\
         //! available on the current CPU (AVX2/SSE2 on x86_64, NEON on aarch64,\n\
         //! `simd128` on wasm32) and falls back to a scalar loop otherwise.\n",
    );
    out.push_str("//!\n");
    out
}

/// Render the byte that triggers an escape in a doc-friendly form.
fn render_source_byte(b: u8) -> String {
    match b {
        b'`' => "`` ` ``".to_string(),
        b'|' => "`\\|`".to_string(),
        0x20..=0x7E => format!("`{}`", b as char),
        _ => format!("`<0x{:02X}>`", b),
    }
}

/// Escape backticks and pipes in a replacement so it can sit inside a Markdown
/// table cell wrapped in inline code.
fn escape_md_inline_code(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '`' => out.push('\''),
            '|' => out.push_str("\\|"),
            _ => out.push(ch),
        }
    }
    out
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let dir: PathBuf = args.input_dir;

    generate(dir)
}
