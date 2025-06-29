# v_escape-codegen

A tool for generating escape functions.

## Installation

```bash
cargo install v_escape-codegen
```

## Usage

```bash
# Create a new crate
mkdir <crate_name>
cd <crate_name>
cargo init --lib

# Write a template to src/_lib.rs
cat <<EOF > src/_lib.rs
new!(
    '<' -> "&lt;",
    '>' -> "&gt;",
    '&' -> "&amp;",
    '"' -> "&quot;",
    '\'' -> "&#x27;",
    '/' -> "&#x2f;"
);
EOF

# Generate the code from the template
v_escape-codegen -i .
```

## Generated crate

Generate a new crate with `escape_fmt` and `escape_string` functions.

### Features in the generated crate

- `alloc`: Enables the `alloc` library features.
- `fmt`: Enables the `escape_fmt` function.
- `string`: Enables the `escape_string` function.
- `std`: Enables the `std` library features.
