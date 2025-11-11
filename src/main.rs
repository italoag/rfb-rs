use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "rfb")]
#[command(about = "RFB-RS - ETL and API for Brazilian Federal Revenue data", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Download data files from Federal Revenue
    Download {
        /// Directory to save downloaded files
        #[arg(short, long, default_value = "data")]
        directory: String,

        /// Skip already downloaded files
        #[arg(short, long)]
        skip_existing: bool,

        /// Maximum parallel downloads
        #[arg(short, long, default_value_t = 4)]
        parallel: usize,

        /// Restart downloads from beginning
        #[arg(short, long)]
        restart: bool,
    },

    /// Transform downloaded data
    Transform {
        /// Directory with downloaded files
        #[arg(short, long, default_value = "data")]
        directory: String,

        /// Output directory
        #[arg(short, long, default_value = "output")]
        output: String,

        /// Enable privacy mode (mask sensitive data)
        #[arg(short, long)]
        privacy: bool,
    },

    /// Database commands
    Db {
        #[command(subcommand)]
        command: DbCommands,
    },

    /// Start API server
    Api {
        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// Port to bind to
        #[arg(short, long, default_value_t = 8080)]
        port: u16,

        /// Database URL
        #[arg(short, long)]
        database_url: Option<String>,
    },

    /// Check integrity of downloaded ZIP files
    Check {
        /// Directory with downloaded files
        #[arg(short, long, default_value = "data")]
        directory: String,

        /// Delete corrupted files
        #[arg(short = 'x', long)]
        delete: bool,
    },
}

#[derive(Subcommand, Debug)]
enum DbCommands {
    /// Create database tables
    Create {
        /// Database URL
        #[arg(short, long)]
        database_url: Option<String>,

        /// PostgreSQL schema
        #[arg(short, long, default_value = "public")]
        schema: String,
    },

    /// Drop database tables
    Drop {
        /// Database URL
        #[arg(short, long)]
        database_url: Option<String>,

        /// PostgreSQL schema
        #[arg(short, long, default_value = "public")]
        schema: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Download {
            directory,
            skip_existing,
            parallel,
            restart,
        } => {
            // Validate parallel parameter
            if parallel == 0 {
                eprintln!("Error: parallel downloads must be at least 1");
                std::process::exit(1);
            }

            let config = rfb_rs::download::DownloadConfig {
                data_dir: directory,
                skip_existing,
                max_parallel: parallel,
                restart,
                ..Default::default()
            };
            let downloader = rfb_rs::Downloader::new(config);
            downloader.download().await?;
        }

        Commands::Transform {
            directory,
            output,
            privacy,
        } => {
            let config = rfb_rs::transform::TransformConfig {
                data_dir: directory,
                output_dir: output,
                privacy_mode: privacy,
            };
            let mut transformer = rfb_rs::Transformer::new(config);
            transformer.load_lookups()?;
            transformer.transform().await?;
        }

        Commands::Db { command } => match command {
            DbCommands::Create {
                database_url,
                schema,
            } => {
                use rfb_rs::db::{Database, PostgresDatabase};
                let db_url = database_url.unwrap_or_else(|| {
                    std::env::var("DATABASE_URL")
                        .expect("DATABASE_URL must be set or provided via --database-url")
                });
                let db = PostgresDatabase::new(db_url, schema);
                db.create()?;
                println!("Database tables created successfully");
            }

            DbCommands::Drop {
                database_url,
                schema,
            } => {
                use rfb_rs::db::{Database, PostgresDatabase};
                let db_url = database_url.unwrap_or_else(|| {
                    std::env::var("DATABASE_URL")
                        .expect("DATABASE_URL must be set or provided via --database-url")
                });
                let db = PostgresDatabase::new(db_url, schema);
                db.drop()?;
                println!("Database tables dropped successfully");
            }
        },

        Commands::Api {
            host,
            port,
            database_url,
        } => {
            let config = rfb_rs::api::ApiConfig {
                host,
                port,
                database_url: database_url.unwrap_or_else(|| {
                    std::env::var("DATABASE_URL")
                        .unwrap_or_else(|_| "postgres://localhost/rfb".to_string())
                }),
            };
            let server = rfb_rs::ApiServer::new(config);
            server.start().await?;
        }

        Commands::Check { directory, delete } => {
            use std::path::Path;
            use walkdir::WalkDir;

            let data_dir = Path::new(&directory);
            if !data_dir.exists() {
                eprintln!("Directory does not exist: {}", directory);
                std::process::exit(1);
            }

            let mut checked = 0;
            let mut errors = 0;

            for entry in WalkDir::new(data_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("zip"))
            {
                checked += 1;
                let path = entry.path();
                print!("Checking {}... ", path.display());

                match rfb_rs::download::check_zip_integrity(path) {
                    Ok(_) => {
                        println!("OK");
                    }
                    Err(e) => {
                        println!("ERROR: {}", e);
                        errors += 1;
                        if delete {
                            match std::fs::remove_file(path) {
                                Ok(_) => println!("  Deleted corrupted file"),
                                Err(e) => eprintln!("  Failed to delete: {}", e),
                            }
                        }
                    }
                }
            }

            println!("\nChecked {} files, {} errors", checked, errors);
        }
    }

    Ok(())
}
