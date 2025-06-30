# Release Guide

This document explains how to use the automated release system for the v_escape monorepo.

## Overview

The release system uses:

- **Git Cliff**: For generating changelogs from conventional commits
- **Cargo Release**: For version bumping, publishing to crates.io, and creating git tags
- **GitHub Actions**: For automated releases triggered by git tags

## Prerequisites

### Required Tools

1. **Git Cliff**: Install from [https://github.com/orhun/git-cliff#installation](https://github.com/orhun/git-cliff#installation)

   ```bash
   # On Arch Linux
   sudo pacman -S git-cliff

   # On macOS
   brew install git-cliff

   # On Ubuntu/Debian
   sudo apt install git-cliff
   ```

2. **Cargo Release**: Will be installed automatically by the release script
   ```bash
   cargo install cargo-release --locked
   ```

### Required Secrets

Set up the following secrets in your GitHub repository:

1. **`CARGO_REGISTRY_TOKEN`**: Your crates.io API token

   - Get it from [https://crates.io/settings/tokens](https://crates.io/settings/tokens)
   - This is used to publish packages to crates.io

2. **`GITHUB_TOKEN`**: Automatically provided by GitHub Actions

## Release Process

### 1. Manual Release (Recommended)

Use the provided release script:

```bash
# Dry run first (recommended)
./scripts/release.sh v_htmlescape patch --dry-run

# Actual release
./scripts/release.sh v_htmlescape patch
```

**Available packages:**

- `base` - Core escaping functionality
- `codegen-base` - Code generation base
- `codegen` - Code generation tools
- `proc-macro` - Procedural macros
- `v_escape` - Main escape crate
- `v_htmlescape` - HTML escaping
- `v_jsonescape` - JSON escaping
- `v_latexescape` - LaTeX escaping

**Version types:**

- `patch` - Bug fixes (0.1.0 → 0.1.1)
- `minor` - New features (0.1.0 → 0.2.0)
- `major` - Breaking changes (0.1.0 → 1.0.0)

### 2. GitHub Actions Workflow

You can also trigger releases via GitHub Actions:

1. Go to your repository's Actions tab
2. Select "Prepare Release" workflow
3. Click "Run workflow"
4. Choose the package, version type, and whether to do a dry run

### 3. Direct Cargo Release

For advanced users, you can use cargo-release directly:

```bash
# Install cargo-release
cargo install cargo-release --locked

# Dry run
cargo release patch --package v_htmlescape --dry-run

# Actual release
cargo release patch --package v_htmlescape --execute
```

## Release Order

When releasing multiple packages, follow this order to handle dependencies correctly:

1. `base` (no dependencies)
2. `codegen-base` (depends on base)
3. `codegen` (depends on codegen-base)
4. `proc-macro` (depends on base)
5. `v_escape` (depends on base)
6. `v_htmlescape` (depends on base)
7. `v_jsonescape` (depends on base)
8. `v_latexescape` (depends on base)

## Configuration Files

### `release.toml`

Configuration for cargo-release:

- Workspace mode enabled
- Git integration configured
- Changelog generation with Git Cliff
- Publishing to crates.io

### `cliff.toml`

Configuration for Git Cliff:

- Conventional commits parsing
- Changelog template
- Tag pattern for monorepo (`*-v[0-9]*`)

## Commit Convention

Follow [Conventional Commits](https://www.conventionalcommits.org/) for automatic changelog generation:

```
feat: add new HTML escaping function
fix: resolve memory leak in JSON escaping
docs: update README with usage examples
perf: optimize SIMD operations
refactor: restructure code generation
test: add comprehensive test suite
chore: update dependencies
```

## Automated Workflows

### Release Workflow (`.github/workflows/release.yml`)

Triggered by git tags matching `*-v*`:

1. Extracts package name and version from tag
2. Generates changelog using Git Cliff
3. Publishes to crates.io
4. Creates GitHub release

### Debug Mode

Run cargo-release with verbose output:

```bash
cargo release patch --package v_htmlescape --dry-run --verbose
```

### Check Current Versions

```bash
# List all package versions
cargo metadata --format-version 1 | jq '.packages[] | {name: .name, version: .version}'
```

## Support

If you encounter issues:

1. Check the troubleshooting section above
2. Review the GitHub Actions logs
3. Check the cargo-release documentation: [https://github.com/crate-ci/cargo-release](https://github.com/crate-ci/cargo-release)
4. Check the Git Cliff documentation: [https://github.com/orhun/git-cliff](https://github.com/orhun/git-cliff)
