use crate::db;
use crate::feed;
use slug;
use clap::{Parser, Subcommand};

const DEFAULT_DB_NAME: &str = "test.db";

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
    // List podcasts.
    List {
        // The id of the podcast to show.
        #[arg(short, long)]
        id: Option<i64>,

        // Detailed output.
        #[arg(short, long)]
        detailed: bool,

        // Number of podcasts to show.
        #[arg(short, long)]
        limit: Option<i64>,
    },
    // Add a podcast.
    Add {
        // The url of the podcast to add.
        url: String,
    },
    // List episodes.
    Episodes {
        // The id of the podcast to show episodes for.
        #[arg(short, long)]
        id: i64,

        // Detailed output.
        #[arg(short, long)]
        detailed: bool,

        // Number of episodes to show.
        #[arg(short, long)]
        limit: Option<i64>,
    },
    // Update podcasts.
    Update {
        // The id of the podcast to update.
        #[arg(short, long)]
        id: Option<i64>,
    },
    // Remove a podcast.
    Remove {
        // The id of the podcast to remove.
        id: i64,
    },
    Download {
        // The id of the episode to download.
        id: i64,
    },
    Search {
        // Detailed output.
        #[arg(short, long)]
        detailed: bool,

        // Search episodes.
        #[arg(short, long)]
        episodes: bool,

        // Podcast id to search within.
        #[arg(short, long)]
        id: Option<i64>,
        
        // The search term.
        term: String,
    },
}

pub fn parse_args() {
    let cli = Cli::parse();
    let db_name = cli.db.unwrap_or(DEFAULT_DB_NAME.to_string());
    match cli.command {
        Commands::List {
            id,
            detailed,
            limit,
        } => do_list(db_name, id, detailed, limit),
        Commands::Add { url } => do_add(db_name, url),
        Commands::Episodes {
            id,
            detailed,
            limit,
        } => do_episodes(db_name, id, detailed, limit),
        Commands::Update { id } => do_update(db_name, id),
        Commands::Remove { id } => do_remove(db_name, id),
        Commands::Download { id } => do_download(db_name, id),
        Commands::Search { term, detailed, episodes, id } => do_search(db_name, term, detailed, episodes, id),
    }
}

fn do_list(db_name: String, id: Option<i64>, detailed: bool, limit: Option<i64>) {
    let conn = db::init_db(db_name).unwrap();
    if let Some(id) = id {
        let pod = db::fetch_podcast_and_episodes(&conn, id).unwrap();
        pod.print(detailed);
    } else {
        let pods = db::fetch_all_podcasts(&conn).unwrap();
        match limit {
            Some(limit) => {
                for pod in pods.iter().take(limit as usize) {
                    pod.print(detailed);
                }
            }
            None => {
                for pod in pods {
                    pod.print(detailed);
                }
            }
        }
    }
}

fn do_add(db_name: String, url: String) {
    let conn = db::init_db(db_name).unwrap();
    let rss = feed::fetch_rss(&url).unwrap();
    let mut pod = feed::parse_rss(&url, &rss).unwrap();
    db::insert_podcast(&conn, &mut pod).unwrap();
    println!("Added podcast {}.", pod.title);
}

fn do_episodes(db_name: String, id: i64, detailed: bool, limit: Option<i64>) {
    let conn = db::init_db(db_name).unwrap();
    let pod = db::fetch_podcast_and_episodes(&conn, id).unwrap();
    match limit {
        Some(limit) => {
            for ep in pod.episodes.iter().take(limit as usize) {
                ep.print(detailed);
            }
        }
        None => {
            for ep in pod.episodes {
                ep.print(detailed);
            }
        }
    }
}

fn do_update(db_name: String, id: Option<i64>) {
    let conn = db::init_db(db_name).unwrap();
    match id {
        Some(id) => {
            let pod = db::fetch_podcast(&conn, id).unwrap();
            let rss = feed::fetch_rss(&pod.rss_url).unwrap();
            let pod = feed::parse_rss(&pod.rss_url, &rss).unwrap();
            for ep in pod.episodes {
                db::insert_episode(&conn, &ep, id).unwrap();
            }
            println!("Updated {}.", pod.title);
        }
        None => {
            let pods = db::fetch_all_podcasts(&conn).unwrap();
            for mut pod in pods {
                let rss = feed::fetch_rss(&pod.rss_url).unwrap();
                pod = feed::parse_rss(&pod.rss_url, &rss).unwrap();
                for ep in pod.episodes {
                    db::insert_episode(&conn, &ep, pod.id).unwrap();
                }
                println!("Updated podcast {}.", pod.title);
            }
        }
    }
}

fn do_remove(db_name: String, id: i64) {
    let conn = db::init_db(db_name).unwrap();
    match db::fetch_podcast(&conn, id) {
        Ok(pod) => {
            db::remove_podcast(&conn, id).unwrap();
            println!("Removed {}.", pod.title);
        }
        Err(_) => {
            println!("No podcast with id {}.", id);
        }
    }
}

fn do_download(db_name: String, id: i64) {
    let conn = db::init_db(db_name).unwrap();
    let ep = db::fetch_episode(&conn, id).unwrap();
    let enclosure = ep.enclosure.unwrap();
    let data = feed::fetch_enclosure(&enclosure).unwrap();
    let fname = slug::slugify(ep.title) + ".mp3";
    std::fs::write(fname, data).unwrap();
}

fn do_search(db_name: String, term: String, detailed: bool, episodes: bool, id: Option<i64>) {
    let conn = db::init_db(db_name).unwrap();
    match (episodes, id) {
        (true, Some(id)) => {
            let eps = db::search_episodes(&conn, term, id).unwrap();
            for ep in eps {
                ep.print(detailed);
            }
        }
        (true, None) => {
            let eps = db::search_episodes(&conn, term, 0).unwrap();
            for ep in eps {
                ep.print(detailed);
            }
        }
        _ => {
            let pods = db::search_podcasts(&conn, term).unwrap();
            for pod in pods {
                pod.print(detailed);
            }
        }
    }
}
