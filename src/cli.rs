// use clap::{Args, Parser, Subcommand};
//
// #[derive(Parser)]
// #[command(name = "dipper", version = "0.1.0", about = "A podcast client")]
// struct Args {
//     #[arg(short, long, default_value = "dipper.db")]
//     dbfname: String,
//
//     #[clap(subcommand)]
//     cmd: Command,
// }
//
// #[derive(Debug, clap::subcommand::Subcommand)]
// enum Command {
//     // Add a podcast to the database
//     Add {
//         // Url to add to the database
//         #[clap(short, long, value_name = "URL")]
//         url: String,
//     },
//     // List podcasts in the database
//     List {
//         // Podcast id to list
//         #[clap(short, long)]
//         id: Option<i64>,
//
//         // Number of podcasts to list
//         #[clap(short, long)]
//         count: Option<i64>,
//
//         // Show detailed information
//         #[clap(short, long)]
//         detailed: bool,
//     },
//     // Update podcasts in the database
//     Update {
//         // Podcast id to update
//         #[clap(short, long)]
//         id: Option<i64>,
//     },
//     // Remove a podcast from the database
//     Remove {
//         // Podcast id to remove
//         #[clap(short, long)]
//         id: i64,
//     },
//
//     // View and manipulate episodes
//     Episode {
//         #[clap(subcommand)]
//         cmd: EpisodeCommand,
//     },
// }
//
// #[derive(Debug, clap::subcommand::Subcommand)]
// enum EpisodeCommand {
//     // List episodes in the database
//     List {
//         // Podcast id to list
//         #[clap(short, long)]
//         id: i64,
//
//         // Number of episodes to list
//         #[clap(short, long)]
//         count: Option<i64>,
//
//         // Show detailed information
//         #[clap(short, long)]
//         detailed: bool,
//     },
//
//     // Download episodes
//     Download {
//         // Episode id to download
//         #[clap(short, long)]
//         id: i64,
//     },
// }
