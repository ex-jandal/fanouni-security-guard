use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Fanouni Security Gateway", long_about = None)]
pub struct Args {
    /// Generate a new Ed25519 + HMAC Private Key
    /// for the Copyright Notary and Sha256 hash check
    #[arg(long)]
    pub generate_keys: bool,

    /// Set the port for the gateway (Default: 4000)
    #[arg(short, long, default_value_t = 4000)]
    pub port: u16,

    /// Run in Debug Mode
    #[arg(long)]
    pub debug: bool,
}
