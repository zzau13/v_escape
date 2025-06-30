#!/bin/bash

# Script to check current versions of all packages in the monorepo

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Check if jq is available
if ! command -v jq &> /dev/null; then
    print_warning "jq is not installed. Installing..."
    if command -v pacman &> /dev/null; then
        sudo pacman -S jq
    elif command -v apt &> /dev/null; then
        sudo apt install jq
    elif command -v brew &> /dev/null; then
        brew install jq
    else
        print_warning "Please install jq manually to get better output formatting"
        print_info "You can install it from: https://jqlang.github.io/jq/download/"
    fi
fi

print_info "Checking package versions..."

# Get package metadata
if command -v jq &> /dev/null; then
    # Use jq for formatted output
    cargo metadata --format-version 1 | jq -r '.packages[] | "\(.name): \(.version)"' | sort
else
    # Fallback without jq
    cargo metadata --format-version 1 | grep -E '"name"|"version"' | paste -d: - - | sed 's/.*"name":"\([^"]*\)".*"version":"\([^"]*\)".*/\1: \2/' | sort
fi

print_success "Version check completed!"

# Show workspace packages
print_info "Workspace packages:"
echo "  - base"
echo "  - codegen-base"
echo "  - codegen"
echo "  - proc-macro"
echo "  - v_escape"
echo "  - v_htmlescape"
echo "  - v_jsonescape"
echo "  - v_latexescape"

print_info "To release a package, use: ./scripts/release.sh <package> <version-type> [--dry-run]" 