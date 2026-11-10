#!/bin/bash

set -euo pipefail
dir="$(cd "$(dirname "$0")" && pwd)"/../packages

for i in $(find $dir -mindepth 1 -maxdepth 1 -type d -printf '%f\n') 
do
    awk -F '"' '/^name =/ {print $2; exit}' "$dir/$i/Cargo.toml"
done

