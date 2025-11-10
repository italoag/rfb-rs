# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure with modular architecture
- Download module skeleton with Federal Revenue data source
  - File URL generation for all required files
  - ZIP integrity checking functionality
  - Download configuration structure
- Transform module skeleton with data structures
  - Company data structure with all fields
  - Partner (QSA) data structure
  - CNAE (economic activity) data structure
  - Tax regime data structure
  - Lookup tables for reference data
  - ZIP extraction capability
- Database module skeleton
  - PostgreSQL support structure
  - Database trait for abstraction
  - Schema definition with indexes
- API module skeleton
  - Server structure
  - Handler functions (health, metrics, company lookup)
- Comprehensive CLI with subcommands
  - `download` - Download data from Federal Revenue
  - `transform` - Transform downloaded data
  - `db create` - Create database tables
  - `db drop` - Drop database tables
  - `api` - Start API server
  - `check` - Verify ZIP file integrity
- Test suite with 16+ passing tests
- Comprehensive documentation
  - README with usage examples
  - DEVELOPMENT guide for contributors
  - Environment configuration examples

### Changed
- Replaced simple greeting CLI with full-featured ETL and API tool
- Updated dependencies to include Polars, Tokio, and other required libraries

### Technical Details
- Rust edition: 2021
- Minimum Rust version: 1.70
- Key dependencies:
  - Polars 0.25 for data processing
  - Clap 4.0 for CLI
  - Diesel 2.0 for database
  - Tokio 1.0 for async runtime
  - Reqwest 0.11 for HTTP
  - Tracing 0.1 for logging

## [0.1.0] - 2024-XX-XX

### Added
- Initial release
- Core module structure
- Basic CLI framework

[Unreleased]: https://github.com/italoag/rfb-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/italoag/rfb-rs/releases/tag/v0.1.0
