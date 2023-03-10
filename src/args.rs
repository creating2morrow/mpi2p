use clap::Parser;

/// cmd line args
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
    /// Monero RPC HOST
    #[arg(
        long,
        help = "Monero RPC host.",
        default_value = "http://localhost:38083"
    )]
    pub monero_rpc_host: String,
    /// Monero RPC password (--password arg)
    #[arg(
        long,
        help = "Monero RPC password.",
        default_value = "password",
    )]
    pub monero_rpc_password: String,
    /// Monero RPC daemon
    #[arg(
        long,
        help = "Monero RPC daemon.",
        default_value = "https://stagenet.xmr-tw.org:38081"
    )]
    pub monero_rpc_daemon: String,
    /// Monero RPC Username
    #[arg(
        long,
        help = "Monero RPC username.",
        default_value = "user",
    )]
    pub monero_rpc_username: String,
    /// Monero RPC credential
    #[arg(
        long,
        help = "Monero RPC credential.",
        default_value = "pass",
    )]
    pub monero_rpc_cred: String,
    /// Token expiration in minutes
    #[arg(
        short,
        long,
        help = "Set the token expiration limit in minutes.",
        default_value = "60"
    )]
    pub token_timeout: i64,
    /// JWT Secret Key
    #[arg(
        short,
        long,
        help = "Set a secret for signing JWTs",
        default_value = "some-secret",
    )]
    pub jwt_secret_key: String,
}
