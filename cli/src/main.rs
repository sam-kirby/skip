use clap::{Parser, Subcommand};

/// Password and OTP generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate a new password
    Generate { name: String },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    Ok(())
}
