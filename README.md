# rfb-rs

**RFB-RS** - ETL and API for Brazilian Federal Revenue (Receita Federal Brasileira) data in Rust 🦀

A Rust implementation inspired by [minha-receita](https://github.com/cuducos/minha-receita), providing tools to download, transform, and serve Brazilian company data from the Federal Revenue.

## 🎯 Features

- **Download**: Efficiently download large ZIP files from Federal Revenue using chunked HTTP requests
- **Transform**: Process and transform CSV data using Polars DataFrames
- **Database**: Load data into PostgreSQL with proper indexing
- **API**: REST API to query company information by CNPJ
- **Privacy**: Optional privacy mode to mask sensitive personal information

## 📋 Requirements

- Rust 1.70 or higher
- PostgreSQL 12+ (optional, for database functionality)
- ~200GB disk space for complete dataset

## 🚀 Installation

### From Source

```bash
git clone https://github.com/italoag/rfb-rs
cd rfb-rs
cargo build --release
```

The binary will be available at `target/release/rfb`

## 📖 Usage

### Download Data

Download all required files from Federal Revenue:

```bash
rfb download --directory data --parallel 4
```

Options:
- `-d, --directory`: Directory to save files (default: `data`)
- `-s, --skip-existing`: Skip already downloaded files
- `-p, --parallel`: Number of parallel downloads (default: 4)
- `-r, --restart`: Restart all downloads from beginning

### Check File Integrity

Verify downloaded ZIP files:

```bash
rfb check --directory data
```

Options:
- `-d, --directory`: Directory with downloaded files
- `-x, --delete`: Delete corrupted files

### Transform Data

Extract and process the downloaded data:

```bash
rfb transform --directory data --output output
```

Options:
- `-d, --directory`: Directory with downloaded files
- `-o, --output`: Output directory for processed data
- `-p, --privacy`: Enable privacy mode (masks CPF and personal data)

### Database Operations

Create database tables:

```bash
export DATABASE_URL="postgres://user:password@localhost/rfb"
rfb db create
```

Drop database tables:

```bash
rfb db drop
```

Options:
- `-d, --database-url`: PostgreSQL connection URL (or use DATABASE_URL env var)
- `-s, --schema`: PostgreSQL schema name (default: `public`)

### API Server

Start the REST API server:

```bash
export DATABASE_URL="postgres://user:password@localhost/rfb"
rfb api --port 8080
```

Options:
- `--host`: Host to bind to (default: `127.0.0.1`)
- `-p, --port`: Port to bind to (default: 8080)
- `-d, --database-url`: Database connection URL

API Endpoints:
- `GET /cnpj/{cnpj}` - Get company information by CNPJ
- `GET /health` - Health check
- `GET /metrics` - Prometheus metrics

## 📊 Data Structure

The system processes several types of data:

### Companies (Estabelecimentos)
Main company data including:
- CNPJ (company ID)
- Trade name and legal name
- Address and contact information
- Registration status
- Economic activity (CNAE)
- Tax regime (Simples Nacional, MEI)

### Partners (Sócios/QSA)
Company partners and stakeholders:
- Partner name and identification
- Entry date
- Qualification
- Legal representatives

### Lookup Tables
Reference data for codes:
- Countries (Países)
- Cities (Municípios)
- Legal natures (Naturezas Jurídicas)
- Qualifications (Qualificações)
- Economic activities (CNAEs)
- Registration status motives (Motivos)

## 🏗️ Architecture

```
rfb-rs/
├── src/
│   ├── download/      # Download module
│   │   ├── federal_revenue.rs
│   │   ├── downloader.rs
│   │   └── check.rs
│   ├── transform/     # Transform module
│   │   ├── company.rs
│   │   ├── partner.rs
│   │   ├── cnae.rs
│   │   ├── tax_regime.rs
│   │   ├── lookups.rs
│   │   └── transformer.rs
│   ├── db/           # Database module
│   │   ├── postgres.rs
│   │   └── schema.rs
│   ├── api/          # API module
│   │   ├── server.rs
│   │   └── handlers.rs
│   ├── lib.rs
│   └── main.rs
└── Cargo.toml
```

## 🛠️ Technology Stack

- **Clap**: Command-line argument parsing
- **Polars**: High-performance DataFrames for data transformation
- **Diesel**: Type-safe PostgreSQL ORM
- **Reqwest**: HTTP client for downloads
- **Tokio**: Async runtime
- **Actix-web**: HTTP server (planned)
- **Tracing**: Structured logging

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## 📄 License

This project is licensed under the MIT License - see the LICENSE.md file for details.

## 🙏 Acknowledgments

- [minha-receita](https://github.com/cuducos/minha-receita) by [@cuducos](https://github.com/cuducos) - The original Go implementation that inspired this project
- [Receita Federal do Brasil](http://www.receita.fazenda.gov.br/) - For providing open data

## 📚 Related Projects

- [minha-receita](https://github.com/cuducos/minha-receita) - Original Go implementation
- [brasil.io](https://brasil.io/dataset/socios-brasil/) - Similar data access service

## ⚠️ Disclaimer

This project is not officially affiliated with the Brazilian Federal Revenue. The data is publicly available and provided by the government.