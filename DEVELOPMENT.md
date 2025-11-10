# Development Guide

## Overview

This document provides guidance for developers working on rfb-rs.

## Project Structure

```
rfb-rs/
├── src/
│   ├── download/          # Download module
│   │   ├── mod.rs         # Module definition and error types
│   │   ├── federal_revenue.rs  # Federal Revenue URLs
│   │   ├── downloader.rs  # Main download logic
│   │   └── check.rs       # ZIP integrity checking
│   ├── transform/         # Transform module
│   │   ├── mod.rs         # Module definition and error types
│   │   ├── company.rs     # Company data structures
│   │   ├── partner.rs     # Partner data structures
│   │   ├── cnae.rs        # CNAE data structures
│   │   ├── tax_regime.rs  # Tax regime data structures
│   │   ├── lookups.rs     # Lookup tables
│   │   └── transformer.rs # Main transform logic
│   ├── db/               # Database module
│   │   ├── mod.rs        # Module definition and traits
│   │   ├── postgres.rs   # PostgreSQL implementation
│   │   └── schema.rs     # Database schema
│   ├── api/              # API module
│   │   ├── mod.rs        # Module definition
│   │   ├── server.rs     # API server
│   │   └── handlers.rs   # Request handlers
│   ├── lib.rs            # Library root
│   └── main.rs           # Binary entry point
├── Cargo.toml            # Dependencies and metadata
└── README.md             # User documentation
```

## Development Workflow

### Setup

1. Clone the repository
2. Install Rust (1.70+): https://rustup.rs/
3. Install PostgreSQL (optional for testing)
4. Copy `.env.example` to `.env` and configure

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Check without building
cargo check
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests with specific filter
cargo test download::
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for outdated dependencies
cargo outdated

# Audit dependencies for security vulnerabilities
cargo audit
```

## Implementation Status

### ✅ Completed

- [x] Project structure and module organization
- [x] CLI command framework with clap
- [x] Download module structure
- [x] Transform module data structures
- [x] Database module structure
- [x] API module structure
- [x] Basic tests for all modules
- [x] Documentation and README

### 🚧 In Progress / TODO

#### Download Module
- [ ] Implement chunked HTTP downloads with reqwest
- [ ] Add retry logic with exponential backoff
- [ ] Add progress bars with indicatif
- [ ] Implement resume capability for interrupted downloads
- [ ] Add checksum verification

#### Transform Module
- [ ] Implement CSV parsing with Polars
- [ ] Load and parse lookup tables
- [ ] Enrich company data with lookups
- [ ] Handle data type conversions
- [ ] Implement privacy mode data masking
- [ ] Add validation for CNPJ format
- [ ] Process all file types (Estabelecimentos, Empresas, Socios, Simples)

#### Database Module
- [ ] Implement Diesel integration
- [ ] Add migrations for schema management
- [ ] Implement bulk insert operations
- [ ] Add connection pooling with r2d2
- [ ] Create indexes for performance
- [ ] Add transaction support
- [ ] Implement data loading from transformed files

#### API Module
- [ ] Implement HTTP server with actix-web
- [ ] Add CNPJ lookup endpoint
- [ ] Add pagination support
- [ ] Implement search endpoints
- [ ] Add Prometheus metrics
- [ ] Add CORS support
- [ ] Implement rate limiting
- [ ] Add API documentation with OpenAPI/Swagger

#### Additional Features
- [ ] Add integration tests
- [ ] Add performance benchmarks
- [ ] Add Docker support
- [ ] Add GitHub Actions CI/CD
- [ ] Add example queries and use cases
- [ ] Add monitoring and observability

## Key Design Decisions

### Why Polars?

Polars is chosen over other data processing libraries because:
- High performance (written in Rust, uses Apache Arrow)
- Lazy evaluation for memory efficiency
- Similar API to pandas but faster
- Native Rust integration
- Excellent CSV parsing capabilities

### Why Diesel?

Diesel provides:
- Type-safe SQL queries at compile time
- Migration management
- Connection pooling
- Works well with PostgreSQL

### Why Actix-web?

Actix-web offers:
- High performance
- Async/await support
- Mature ecosystem
- Good middleware support

## Testing Strategy

### Unit Tests

Each module should have unit tests for:
- Data structure creation and validation
- Parsing logic
- Error handling
- Edge cases

### Integration Tests

Integration tests should cover:
- End-to-end data pipeline
- Database operations
- API endpoints
- CLI commands

### Performance Tests

Benchmark critical operations:
- CSV parsing with large files
- Database bulk inserts
- API response times

## Contributing Guidelines

1. Create a feature branch from `main`
2. Write tests for new functionality
3. Ensure all tests pass
4. Format code with `cargo fmt`
5. Run `cargo clippy` and fix warnings
6. Update documentation as needed
7. Submit a pull request

## Common Tasks

### Adding a New Field to Company

1. Update `Company` struct in `src/transform/company.rs`
2. Update database schema in `src/db/schema.rs`
3. Update transformation logic
4. Update tests
5. Run migrations

### Adding a New Endpoint

1. Define handler in `src/api/handlers.rs`
2. Register route in `src/api/server.rs`
3. Add tests
4. Update API documentation

### Adding a New CLI Command

1. Add command enum variant in `src/main.rs`
2. Implement command handler
3. Add help text
4. Update README

## Performance Considerations

- Use streaming for large file processing
- Batch database operations
- Use connection pooling
- Implement caching where appropriate
- Profile before optimizing

## Security Considerations

- Validate all user inputs
- Use parameterized queries
- Sanitize file paths
- Implement rate limiting
- Review dependencies regularly
- Follow OWASP guidelines

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Polars Documentation](https://pola-rs.github.io/polars-book/)
- [Diesel Documentation](https://diesel.rs/)
- [Actix-web Documentation](https://actix.rs/)
- [minha-receita](https://github.com/cuducos/minha-receita) - Reference implementation
