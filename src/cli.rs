use crate::db;
use crate::feed;
use crate::tui;
use clap::{Parser, Subcommand};
use opml::OPML;

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
    Import {
        file: String,
    },
    Export {
        file: String,
    },
    Tui,
    Play {
        id: i64,
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
        Commands::Search {
            term,
            detailed,
            episodes,
            id,
        } => do_search(db_name, term, detailed, episodes, id),
        Commands::Import { file } => do_import(db_name, file),
        Commands::Export { file } => do_export(db_name, file),
        Commands::Tui => tui::start().unwrap(),
        Commands::Play { id } => do_play(db_name, id),
    }
}

fn do_list(db_name: String, id: Option<i64>, detailed: bool, limit: Option<i64>) {
    let conn = db::init_db(&db_name).unwrap();
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
    let conn = db::init_db(&db_name).unwrap();
    let rss = feed::fetch_rss(&url);
    if rss.is_err() {
        println!("Error fetching RSS feed: {}.", url);
        return;
    }
    let rss = rss.unwrap();
    let pod = feed::parse_rss(&url, &rss);
    if let Ok(mut pod) = pod {
        db::insert_podcast(&conn, &mut pod).unwrap();
        println!("Added {}.", pod.title);
    } else {
        println!("Error parsing RSS feed: {}.", url);
    }
}

fn do_episodes(db_name: String, id: i64, detailed: bool, limit: Option<i64>) {
    let conn = db::init_db(&db_name).unwrap();
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
    let conn = db::init_db(&db_name).unwrap();
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
    let conn = db::init_db(&db_name).unwrap();
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
    let conn = db::init_db(&db_name).unwrap();
    let ep = db::fetch_episode(&conn, id).unwrap();
    let enclosure = ep.enclosure.unwrap();
    let data = feed::fetch_enclosure(&enclosure).unwrap();
    let fname = slug::slugify(ep.title) + ".mp3";
    std::fs::write(fname, data).unwrap();
}

fn do_search(db_name: String, term: String, detailed: bool, episodes: bool, id: Option<i64>) {
    let conn = db::init_db(&db_name).unwrap();
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

fn do_import(db_name: String, file: String) {
    let contents = std::fs::read_to_string(file).unwrap();
    let opml = OPML::from_str(&contents).unwrap();
    opml.body.outlines.iter().for_each(|o| {
        if let Some(ref url) = o.xml_url {
            do_add(db_name.clone(), url.clone())
        }
    });
}

fn do_export(db_name: String, file: String) {
    let conn = db::init_db(&db_name).unwrap();
    let pods = db::fetch_all_podcasts(&conn).unwrap();
    let opml = OPML {
        version: "2.0".to_string(),
        head: None,
        body: opml::Body {
            outlines: pods
                .iter()
                .map(|p| opml::Outline {
                    text: p.title.clone(),
                    r#type: Some("rss".to_string()),
                    title: Some(p.title.clone()),
                    html_url: p.link.clone(),
                    xml_url: Some(p.rss_url.clone()),
                    ..Default::default()
                })
                .collect(),
        },
    };
    std::fs::write(file, opml.to_string().unwrap()).unwrap();
}

fn do_play(db_name: String, id: i64) {
    let conn = db::init_db(&db_name).unwrap();
    let ep = db::fetch_episode(&conn, id).unwrap();
    let enclosure = ep.enclosure.unwrap();
    call_mpv(enclosure);
}

fn call_mpv(enclosure: crate::podcast::Enclosure) {
    std::process::Command::new("mpv")
        .arg(enclosure.url)
        .status()
        .unwrap();
}
