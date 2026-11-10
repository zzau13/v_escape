#!/usr/bin/env sh

set -xe

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repo_root"

for i in v_latexescape v_jsonescape v_htmlescape; do
  cargo run --manifest-path packages/codegen/Cargo.toml -- -i "packages/$i"
done

cargo fmt --all
cargo test --workspace --all-features
