use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // TODO: Support different types,
    // for the time being, generate all
    /// Generate lookup for speedy evaluation
    Generate {}
}

fn main() {
    let cli = Cli::parse();

    dbg!(cli);
}
