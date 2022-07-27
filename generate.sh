#!/usr/bin/env sh

set -xe

cargo install --path codegen
v_escape_codegen -i v_latexescape
v_escape_codegen -i v_jsonescape
v_escape_codegen -i v_htmlescape

cargo fmt
cargo test --all-features
