# Automated Release Setup

This document explains how to set up the automated release system for the v_escape monorepo.

## What's Been Set Up

### 1. Configuration Files

- **`release.toml`**: Cargo Release configuration for monorepo
- **`cliff.toml`**: Git Cliff configuration for changelog generation
- **`.github/workflows/release.yml`**: Automated release workflow
- **`.github/workflows/prepare-release.yml`**: Manual release preparation workflow

### 2. Scripts

- **`scripts/release.sh`**: Main release script with validation and error handling
- **`scripts/check-versions.sh`**: Utility to check current package versions

### 3. Documentation

- **`RELEASE.md`**: Comprehensive release guide
- **`SETUP.md`**: This setup guide

## Quick Start

### 1. Install Dependencies

```bash
# Install Git Cliff
sudo pacman -S git-cliff  # Arch Linux
# or
brew install git-cliff     # macOS
# or
sudo apt install git-cliff # Ubuntu/Debian

# Cargo Release will be installed automatically by the script
```

### 2. Set Up GitHub Secrets

In your GitHub repository settings:

1. Go to Settings → Secrets and variables → Actions
2. Add the following secrets:
   - **`CARGO_REGISTRY_TOKEN`**: Your crates.io API token

### 3. Test the Setup

```bash
# Check current versions
./scripts/check-versions.sh

# Test a dry run release
./scripts/release.sh v_htmlescape patch --dry-run
```

## How It Works

### Release Process Flow

1. **Manual Release** (using `scripts/release.sh`):

   - Validates package name and version type
   - Checks git status (clean working directory)
   - Runs cargo-release to bump version and create tag
   - Triggers GitHub Actions release workflow

2. **GitHub Actions Release Workflow**:

   - Triggered by git tags matching `*-v*`
   - Extracts package name and version from tag
   - Generates changelog using Git Cliff
   - Publishes to crates.io
   - Creates GitHub release

3. **Manual GitHub Actions** (using prepare-release workflow):
   - Web interface for triggering releases
   - Supports dry-run mode
   - Creates pull requests for review

### Tag Format

Tags follow the pattern: `{package-name}-v{version}`

Examples:

- `v_htmlescape-v0.15.9`
- `v_jsonescape-v0.7.9`
- `base-v0.1.0`

### Changelog Generation

Git Cliff automatically generates changelogs from conventional commits:

- `feat:` → Features
- `fix:` → Bug Fixes
- `docs:` → Documentation
- `perf:` → Performance
- `refactor:` → Refactor
- `test:` → Testing
- `chore:` → Miscellaneous Tasks

## Configuration Details

### release.toml

```toml
# Allowed branches for releases
allow-branch = ["main", "master"]

# Push changes to remote
push = true

# Tag the release
tag = true

# Tag name format - usando package name con version (formato: package-vX.Y.Z)
tag-name = "{{package}}-v{{version}}"

# Tag message
tag-message = "Release {{package}} {{version}}"

# Pre-release commit message
pre-release-commit-message = "chore: prepare for {{package}} {{version}}"

# Pre-release hook para generar changelog
pre-release-hook = ["git-cliff", "--tag", "{{package}}-v{{version}}", "--output", "CHANGELOG.md"]
```

### cliff.toml

```toml
# Tag pattern for monorepo
tag_pattern = "*-v[0-9]*"

# Conventional commits parsing
conventional_commits = true

# Commit parsers for different types
commit_parsers = [
    { message = "^feat", group = "Features" },
    { message = "^fix", group = "Bug Fixes" },
    # ... more parsers
]
```

## Usage Examples

### Release a Single Package

```bash
# Patch release
./scripts/release.sh v_htmlescape patch

# Minor release
./scripts/release.sh v_jsonescape minor

# Major release
./scripts/release.sh base major
```

### Release Multiple Packages

```bash
# Release base first (no dependencies)
./scripts/release.sh base patch

# Then release dependent packages
./scripts/release.sh v_htmlescape patch
./scripts/release.sh v_jsonescape patch
./scripts/release.sh v_latexescape patch
```

### GitHub Actions

1. **Automated Release**: Push a tag to trigger automatic release
2. **Manual Preparation**: Use the "Prepare Release" workflow in GitHub Actions

## Troubleshooting

### Common Issues

1. **"git-cliff not found"**

   - Install Git Cliff manually from the official repository

2. **"Working directory is not clean"**

   - Commit or stash your changes before releasing

3. **"Invalid package"**

   - Check the package name spelling
   - Use one of the valid package names

4. **Publishing fails**
   - Check your `CARGO_REGISTRY_TOKEN` is set correctly
   - Ensure you have publish permissions

### Debug Commands

```bash
# Check versions
./scripts/check-versions.sh

# Dry run with verbose output
cargo release patch --package v_htmlescape --dry-run --verbose

# Check git status
git status

# Check current branch
git branch --show-current
```

## Best Practices

1. **Always do a dry run first** to see what changes will be made
2. **Follow the release order** for dependent packages
3. **Use conventional commits** for automatic changelog generation
4. **Test before releasing** to ensure everything works
5. **Review the generated changelog** before publishing
6. **Keep dependencies up to date** in the workspace

## Security Considerations

1. **API Tokens**: Store `CARGO_REGISTRY_TOKEN` as a GitHub secret
2. **Permissions**: GitHub Actions workflows have minimal required permissions
3. **Branch Protection**: Consider protecting main/master branch
4. **Code Review**: Use pull requests for important changes

## Support

- **Cargo Release**: [https://github.com/crate-ci/cargo-release](https://github.com/crate-ci/cargo-release)
- **Git Cliff**: [https://github.com/orhun/git-cliff](https://github.com/orhun/git-cliff)
- **Conventional Commits**: [https://www.conventionalcommits.org/](https://www.conventionalcommits.org/)

## Next Steps

1. Install Git Cliff on your system
2. Set up the `CARGO_REGISTRY_TOKEN` secret in GitHub
3. Test with a dry run: `./scripts/release.sh v_htmlescape patch --dry-run`
4. Make your first release: `./scripts/release.sh v_htmlescape patch`

The system is now ready for automated releases! 🚀
