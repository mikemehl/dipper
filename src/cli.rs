use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, about, version, long_about = None)]
struct Cli {
    // The database to operate on.
    db: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List{
        // The id of the podcast to show.
        #[arg(short, long)]
        id: Option<i64>,

        // Detailed output.
        #[arg(short, long)]
        detailed: bool,
    }
}

pub fn parse_args() {
    let cli = Cli::parse();
    match cli.command {
        Commands::List { id, detailed } => {
            println!("list: id: {:?}, detailed: {}", id, detailed);
        }
    }
}
