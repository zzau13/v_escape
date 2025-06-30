#!/bin/bash

# Release script for v_escape monorepo
# Usage: ./scripts/release.sh <package> <version-type> [--dry-run]

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
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

# Check if required tools are installed
check_dependencies() {
    print_info "Checking dependencies..."
    
    if ! command -v cargo &> /dev/null; then
        print_error "cargo is not installed"
        exit 1
    fi
    
    if ! command -v git &> /dev/null; then
        print_error "git is not installed"
        exit 1
    fi
    
    if ! cargo release --version &> /dev/null; then
        print_warning "cargo-release is not installed. Installing..."
        cargo install cargo-release --locked
    fi
    
    if ! git-cliff --version &> /dev/null; then
        print_warning "git-cliff is not installed. Please install it manually:"
        print_info "Visit: https://github.com/orhun/git-cliff#installation"
        exit 1
    fi
    
    print_success "All dependencies are available"
}

# Validate package name - using actual workspace package names
validate_package() {
    local package=$1
    local valid_packages=("v_escape-base" "v_escape-codegen-base" "v_escape-codegen" "v_escape-proc-macro" "v_escape" "v_htmlescape" "v_jsonescape" "v_latexescape")
    
    for valid_pkg in "${valid_packages[@]}"; do
        if [[ "$package" == "$valid_pkg" ]]; then
            return 0
        fi
    done
    
    print_error "Invalid package: $package"
    print_info "Valid packages: ${valid_packages[*]}"
    exit 1
}

# Validate version type
validate_version() {
    local version=$1
    local valid_versions=("patch" "minor" "major")
    
    for valid_ver in "${valid_versions[@]}"; do
        if [[ "$version" == "$valid_ver" ]]; then
            return 0
        fi
    done
    
    print_error "Invalid version type: $version"
    print_info "Valid version types: ${valid_versions[*]}"
    exit 1
}

# Check git status
check_git_status() {
    print_info "Checking git status..."
    
    if [[ -n $(git status --porcelain) ]]; then
        print_error "Working directory is not clean. Please commit or stash your changes."
        git status --short
        exit 1
    fi
    
    if [[ -z $(git branch --show-current) ]]; then
        print_error "Not on a branch. Please checkout a branch."
        exit 1
    fi
    
    print_success "Git status is clean"
}

# Main release function
release() {
    local package=$1
    local version_type=$2
    local dry_run=${3:-false}
    
    print_info "Starting release process for package: $package"
    print_info "Version bump type: $version_type"
    print_info "Dry run: $dry_run"
    
    # Check dependencies
    check_dependencies
    
    # Validate inputs
    validate_package "$package"
    validate_version "$version_type"
    
    # Check git status
    check_git_status
    
    # Determine cargo-release command
    local cargo_release_cmd="cargo release $version_type --package $package"
    
    if [[ "$dry_run" == "true" ]]; then
        cargo_release_cmd="$cargo_release_cmd --dry-run"
        print_info "Running in dry-run mode..."
    else
        cargo_release_cmd="$cargo_release_cmd --execute"
        print_info "Running actual release..."
    fi
    
    # Execute cargo-release
    print_info "Executing: $cargo_release_cmd"
    eval "$cargo_release_cmd"
    
    if [[ "$dry_run" == "true" ]]; then
        print_success "Dry run completed successfully"
        print_info "To perform the actual release, run: $0 $package $version_type"
    else
        print_success "Release completed successfully!"
        print_info "The package has been published to crates.io"
        print_info "A GitHub release has been created"
    fi
}

# Show usage
usage() {
    cat << EOF
Usage: $0 <package> <version-type> [--dry-run]

Arguments:
  package       Package to release (v_escape-base, v_escape-codegen-base, v_escape-codegen, v_escape-proc-macro, v_escape, v_htmlescape, v_jsonescape, v_latexescape)
  version-type  Version bump type (patch, minor, major)
  --dry-run     Run in dry-run mode (default: false)

Examples:
  $0 v_htmlescape patch --dry-run
  $0 v_jsonescape minor
  $0 v_escape-base major

Dependencies:
  - cargo
  - git
  - cargo-release (will be installed automatically)
  - git-cliff (must be installed manually)

EOF
}

# Main script logic
main() {
    if [[ $# -lt 2 ]]; then
        usage
        exit 1
    fi
    
    local package=$1
    local version_type=$2
    local dry_run=false
    
    # Parse optional arguments
    shift 2
    while [[ $# -gt 0 ]]; do
        case $1 in
            --dry-run)
                dry_run=true
                shift
                ;;
            *)
                print_error "Unknown option: $1"
                usage
                exit 1
                ;;
        esac
    done
    
    release "$package" "$version_type" "$dry_run"
}

# Run main function with all arguments
main "$@" 