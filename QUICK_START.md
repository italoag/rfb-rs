# Quick Start Guide - CI/CD and Testing

## Running Tests Locally

### All Tests
```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test file
cargo test --test download_integration_test
```

### Unit Tests Only
```bash
cargo test --lib
```

### Integration Tests Only
```bash
cargo test --test '*'
```

### Tests Requiring External Services
```bash
# Set up PostgreSQL
docker run -d -p 5432:5432 \
  -e POSTGRES_DB=rfb_test \
  -e POSTGRES_USER=rfb \
  -e POSTGRES_PASSWORD=rfb \
  postgres:15-alpine

# Set environment variable
export DATABASE_URL="postgres://rfb:rfb@localhost:5432/rfb_test"

# Run ignored tests
cargo test -- --ignored
```

## Making a Release

### Using Semantic Versioning

1. **Make changes and commit using conventional format:**

```bash
# New feature (minor version bump)
git commit -m "feat: add support for custom download URLs"

# Bug fix (patch version bump)
git commit -m "fix: correct timezone handling in date parsing"

# Breaking change (major version bump)
git commit -m "feat!: redesign API response format

BREAKING CHANGE: API now returns data in v2 format"
```

2. **Push to main (or merge PR):**

```bash
git push origin main
```

3. **Automatic pipeline runs:**
   - ✅ Tests on Ubuntu, macOS, Windows
   - ✅ Security audit
   - ✅ Build binaries for 5 platforms
   - ✅ Build Docker images
   - ✅ Analyze commits and determine version
   - ✅ Generate CHANGELOG
   - ✅ Create GitHub release
   - ✅ Publish to crates.io

## Docker Usage

### Local Development

```bash
# Build image
docker build -t rfb-rs .

# Run container
docker run -p 8080:8080 \
  -e DATABASE_URL=postgres://user:pass@host/db \
  rfb-rs

# Using docker-compose (recommended)
docker-compose up -d

# Initialize database
docker-compose run rfb-init

# View logs
docker-compose logs -f rfb-api
```

### Using Published Images

```bash
# Pull latest
docker pull ghcr.io/italoag/rfb-rs:latest

# Pull specific version
docker pull ghcr.io/italoag/rfb-rs:v1.0.0

# Run
docker run -p 8080:8080 \
  -e DATABASE_URL=postgres://user:pass@host/db \
  ghcr.io/italoag/rfb-rs:latest
```

## Commit Message Examples

### Feature Addition
```bash
git commit -m "feat(download): add resume capability for interrupted downloads

- Implements partial download tracking
- Adds retry logic with exponential backoff
- Updates documentation"
```

### Bug Fix
```bash
git commit -m "fix(api): correct CNPJ validation for special cases

Fixes #123"
```

### Documentation
```bash
git commit -m "docs: add Docker deployment guide

- Add Dockerfile
- Add docker-compose.yml
- Update README with Docker instructions"
```

### Breaking Change
```bash
git commit -m "feat(api)!: change API endpoint structure

BREAKING CHANGE: API endpoints now use /v2/ prefix
All clients must update their base URL

Closes #456"
```

### Performance
```bash
git commit -m "perf(transform): optimize Polars DataFrame operations

- Use lazy evaluation
- Reduce memory allocations
- 2x faster processing"
```

## Viewing Results

### GitHub Actions
- Go to repository → Actions tab
- View workflow runs
- Check build status
- Download artifacts

### Releases
- Go to repository → Releases
- View generated changelog
- Download binaries for your platform
- Check release notes

### Coverage
- Check Codecov dashboard (if configured)
- View coverage trends
- See detailed file coverage

## Troubleshooting

### Tests Fail Locally

1. **Check dependencies:**
```bash
cargo clean
cargo build
```

2. **Update dependencies:**
```bash
cargo update
```

3. **Check specific test:**
```bash
cargo test test_name -- --nocapture
```

### CI/CD Pipeline Fails

1. **Check commit message format**
   - Must follow conventional commits
   - Examples: `feat:`, `fix:`, `docs:`

2. **Check GitHub Actions logs**
   - Click on failed workflow
   - Review error messages
   - Fix issues locally first

3. **Verify secrets are set**
   - CARGO_TOKEN for crates.io
   - CODECOV_TOKEN for coverage (optional)

### Docker Build Fails

1. **Test locally:**
```bash
docker build -t test .
```

2. **Check Dockerfile syntax**

3. **Verify base images are available**

## Best Practices

1. **Always test locally before pushing**
```bash
cargo test && cargo clippy && cargo fmt
```

2. **Use meaningful commit messages**
   - Describe what and why
   - Reference issues
   - Follow conventional commits

3. **Keep PRs focused**
   - One feature/fix per PR
   - Clear description
   - Add tests

4. **Review generated CHANGELOG**
   - Check version bump is correct
   - Verify changes are documented

5. **Monitor CI/CD runs**
   - Don't merge failing PRs
   - Fix issues promptly

## Quick Reference

| Command | Description |
|---------|-------------|
| `cargo test` | Run all tests |
| `cargo test --lib` | Unit tests only |
| `cargo test --test '*'` | Integration tests |
| `cargo clippy` | Lint code |
| `cargo fmt` | Format code |
| `cargo build --release` | Build release binary |
| `docker-compose up` | Start all services |

## Example Workflow

```bash
# 1. Create feature branch
git checkout -b feat/new-feature

# 2. Make changes and test
cargo test
cargo clippy
cargo fmt

# 3. Commit with conventional format
git add .
git commit -m "feat: add new feature

- Implements feature X
- Adds tests
- Updates documentation"

# 4. Push and create PR
git push origin feat/new-feature

# 5. After review, merge to main
# Pipeline runs automatically!

# 6. Check releases page for new version
```

## Support

For help:
- Check `CI_CD.md` for detailed documentation
- Review GitHub Actions logs
- Open issue with `ci` or `test` label
