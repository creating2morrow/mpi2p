use clap::Parser;

// cmd line args
#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// set release environment
    #[arg(
        short,
        long,
        help = "Set release environment (dev, prod)",
        default_value = "dev"
    )]
    pub release_env: String,
    /// Log Level
    #[arg(
        short,
        long,
        help = "Comma separated log level e.g. <WARN,INFO...>",
        default_value = "ERROR,INFO"
    )]
    pub log_level: String,
    /// Monero RPC HOST
    #[arg(
        short,
        long,
        help = "Monero RPC host.",
        default_value = "http://localhost:38083"
    )]
    pub monero_rpc_host: String,
    /// Monero RPC HOST
    #[arg(
        short,
        long,
        help = "Postgres db url",
        default_value = "postgres://postgres:postgres@127.0.0.1:5432/postgres"
    )]
    pub postgres_db_url: String,
    /// Token expiration in minutes
    #[arg(
        short,
        long,
        help = "Set the token expiration limit in minutes.",
        default_value = "60"
    )]
    pub token_timeout: i64,
}
