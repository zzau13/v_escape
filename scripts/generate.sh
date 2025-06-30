#!/usr/bin/env sh

set -xe

cargo install --path codegen
for i in v_latexescape v_jsonescape v_htmlescape; do
  ~/.cargo/bin/v_escape-codegen -i $i
done

cargo fmt
cargo test --all-features
