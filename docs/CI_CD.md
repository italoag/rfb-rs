# CI/CD Pipeline Documentation

This document describes the automated CI/CD pipeline for rfb-rs.

## Overview

The CI/CD pipeline automatically:
- Runs comprehensive tests on multiple platforms
- Performs security audits
- Builds release binaries for multiple platforms
- Creates Docker images
- Generates semantic versioned releases
- Publishes to crates.io

## Workflows

### Main CI/CD Pipeline (`.github/workflows/ci-cd.yml`)

Triggered on:
- Push to `main` or `develop` branches
- Pull requests to `main`
- Manual workflow dispatch

#### Jobs:

1. **Test Suite**
   - Runs on: Ubuntu, macOS, Windows
   - Rust versions: stable, beta
   - Steps:
     - Format checking (`cargo fmt`)
     - Linting (`cargo clippy`)
     - Unit tests
     - Integration tests
     - Doc tests

2. **Code Coverage**
   - Generates code coverage report using tarpaulin
   - Uploads to Codecov

3. **Security Audit**
   - Runs `cargo audit` to check for vulnerabilities
   - Uses rustsec database

4. **Build Binaries** (main branch only)
   - Platforms:
     - Linux (GNU and MUSL)
     - macOS (Intel and ARM)
     - Windows
   - Uploads artifacts for release

5. **Build Docker** (main branch only)
   - Multi-architecture builds
   - Pushes to GitHub Container Registry (ghcr.io)
   - Tags: branch name, semver, SHA

6. **Semantic Release** (main branch only)
   - Analyzes commits using conventional commits
   - Determines version bump (major/minor/patch)
   - Generates changelog
   - Creates GitHub release with binaries
   - Updates Cargo.toml version

7. **Publish to crates.io** (main branch only)
   - Publishes new version to Rust package registry

## Semantic Versioning

The project uses [Conventional Commits](https://www.conventionalcommits.org/) for automatic versioning.

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types and Version Bumps

| Type | Description | Version Bump |
|------|-------------|--------------|
| `feat` | New feature | MINOR |
| `fix` | Bug fix | PATCH |
| `perf` | Performance improvement | PATCH |
| `docs` | Documentation only | PATCH |
| `refactor` | Code refactoring | PATCH |
| `build` | Build system changes | PATCH |
| `test` | Adding tests | None |
| `ci` | CI/CD changes | None |
| `chore` | Other changes | None |
| `BREAKING CHANGE` | Breaking changes | MAJOR |

### Examples

```bash
# Minor version bump (0.1.0 -> 0.2.0)
feat: add parallel download support

# Patch version bump (0.1.0 -> 0.1.1)
fix: correct CNPJ validation regex

# Major version bump (0.1.0 -> 1.0.0)
feat!: redesign API endpoints

BREAKING CHANGE: API endpoints now use /v2/ prefix
```

## Release Process

### Automatic (Recommended)

1. Commit changes using conventional commits format
2. Push to `main` branch (or merge PR)
3. Pipeline automatically:
   - Runs all tests
   - Builds binaries
   - Creates release
   - Publishes to crates.io

### Manual

If needed, you can trigger a release manually:

```bash
# Tag with version
git tag v1.0.0
git push origin v1.0.0

# Or use GitHub UI to create release
```

## Docker Images

### Building Locally

```bash
# Build image
docker build -t rfb-rs .

# Run container
docker run -p 8080:8080 \
  -e DATABASE_URL=postgres://user:pass@host/db \
  rfb-rs
```

### Using Docker Compose

```bash
# Start all services (includes PostgreSQL)
docker-compose up -d

# Initialize database
docker-compose run rfb-init

# View logs
docker-compose logs -f rfb-api
```

### Pulling from Registry

```bash
# Pull latest
docker pull ghcr.io/italoag/rfb-rs:latest

# Pull specific version
docker pull ghcr.io/italoag/rfb-rs:v1.0.0
```

## Testing

### Local Testing

```bash
# All tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Specific test file
cargo test --test download_integration_test

# With ignored tests (requires database)
cargo test -- --ignored
```

### CI Testing

Tests run automatically on:
- Every push
- Every pull request
- Before release

### Coverage

```bash
# Generate coverage report
cargo tarpaulin --out html

# View report
open tarpaulin-report.html
```

## Secrets Required

The following GitHub secrets must be configured:

| Secret | Description | Required For |
|--------|-------------|--------------|
| `GITHUB_TOKEN` | Auto-provided by GitHub | All workflows |
| `CARGO_TOKEN` | crates.io API token | Publishing to crates.io |
| `CODECOV_TOKEN` | Codecov upload token | Code coverage (optional) |

## Configuration Files

- `.releaserc.json` - Semantic release configuration
- `.github/workflows/ci-cd.yml` - Main CI/CD workflow
- `Dockerfile` - Container image build
- `docker-compose.yml` - Local development setup
- `.dockerignore` - Docker build exclusions

## Monitoring

- **Build Status**: Check GitHub Actions tab
- **Coverage**: View Codecov dashboard
- **Security**: Check Dependabot alerts
- **Releases**: View GitHub Releases page

## Troubleshooting

### Build Fails

1. Check GitHub Actions logs
2. Reproduce locally: `cargo build --release`
3. Check for dependency conflicts

### Tests Fail

1. Run locally: `cargo test --verbose`
2. Check for platform-specific issues
3. Verify database connectivity (for ignored tests)

### Release Not Created

1. Verify commit message format
2. Check semantic-release logs
3. Ensure CARGO_TOKEN is valid

### Docker Build Fails

1. Test locally: `docker build .`
2. Check for missing dependencies
3. Verify base image availability

## Best Practices

1. **Always use conventional commits** for automatic versioning
2. **Run tests locally** before pushing
3. **Keep secrets secure** and rotate regularly
4. **Monitor CI/CD runs** for failures
5. **Review generated changelogs** before major releases
6. **Tag releases** for easy rollback
7. **Document breaking changes** thoroughly

## Support

For issues with CI/CD:
1. Check GitHub Actions logs
2. Review this documentation
3. Open an issue with CI/CD tag
