use std::path::PathBuf;

use clap::Parser;

mod generator;
mod macros;
mod ranges;
mod scalar;
mod tests;
mod utils;

/// V_escape codegen
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input directory file
    #[clap(short, long, default_value = "./")]
    pub input_dir: PathBuf,
}

fn main() {
    let args = Args::parse();
    let dir: PathBuf = args.input_dir;
    generator::generate(dir);
}
