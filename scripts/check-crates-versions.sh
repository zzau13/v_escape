#!/bin/bash

# Script to check latest versions on crates.io and compare with local versions

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
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

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if required tools are available
check_dependencies() {
    if ! command -v curl &> /dev/null; then
        print_error "curl is not installed"
        exit 1
    fi
    
    if ! command -v jq &> /dev/null; then
        print_warning "jq is not installed. Installing..."
        if command -v pacman &> /dev/null; then
            sudo pacman -S jq
        elif command -v apt &> /dev/null; then
            sudo apt install jq
        elif command -v brew &> /dev/null; then
            brew install jq
        else
            print_error "Please install jq manually"
            exit 1
        fi
    fi
}

# Get local version of a package
get_local_version() {
    local package=$1
    cargo metadata --format-version 1 | jq -r ".packages[] | select(.name == \"$package\") | .version"
}

# Get latest version from crates.io
get_crates_version() {
    local package=$1
    
    # Try to get version from crates.io API
    local response
    response=$(curl -s "https://crates.io/api/v1/crates/$package" 2>/dev/null || echo "")
    
    if [[ -n "$response" ]]; then
        local version
        version=$(echo "$response" | jq -r '.crate.max_version // "not_found"' 2>/dev/null)
        if [[ "$version" == "null" ]]; then
            echo "not_found"
        else
            echo "$version"
        fi
    else
        echo "not_found"
    fi
}

# Compare versions
compare_versions() {
    local local_ver=$1
    local crates_ver=$2
    local package=$3
    
    if [[ "$crates_ver" == "not_found" ]]; then
        echo -e "${YELLOW}$package: $local_ver (local) - ${RED}not published${NC}"
        return
    fi
    
    if [[ "$local_ver" == "$crates_ver" ]]; then
        echo -e "${GREEN}$package: $local_ver (local) = $crates_ver (crates.io)${NC}"
    elif [[ "$local_ver" > "$crates_ver" ]]; then
        echo -e "${BLUE}$package: $local_ver (local) > $crates_ver (crates.io)${NC}"
    else
        echo -e "${RED}$package: $local_ver (local) < $crates_ver (crates.io)${NC}"
    fi
}

# Main function
main() {
    print_info "Checking package versions on crates.io..."
    
    # Check dependencies
    check_dependencies
    
    # List of packages to check
    local packages=(
        "v_escape-base"
        "v_escape-codegen-base"
        "v_escape-codegen"
        "v_escape-proc-macro"
        "v_escape"
        "v_htmlescape"
        "v_jsonescape"
        "v_latexescape"
    )
    
    echo
    echo "Package Version Comparison (local vs crates.io):"
    echo "================================================"
    
    for package in "${packages[@]}"; do
        local local_version
        local crates_version
        
        local_version=$(get_local_version "$package")
        crates_version=$(get_crates_version "$package")
        
        compare_versions "$local_version" "$crates_version" "$package"
    done
    
    echo
    print_info "Legend:"
    echo -e "  ${GREEN}Green${NC}: Local version matches crates.io"
    echo -e "  ${BLUE}Blue${NC}: Local version is newer (ready to publish)"
    echo -e "  ${RED}Red${NC}: Local version is older (needs update)"
    echo -e "  ${YELLOW}Yellow${NC}: Package not published on crates.io"
    
    echo
    print_info "To publish a package, use: ./scripts/release.sh <package> <version-type>"
}

# Run main function
main "$@" 