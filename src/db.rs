use crate::podcast;
use rusqlite;

pub fn init_db(dbfname: String) -> Result<rusqlite::Connection, rusqlite::Error> {
    let conn = rusqlite::Connection::open(dbfname)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS podcasts (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            rss_url TEXT NOT NULL UNIQUE,
            link TEXT,
            language TEXT,
            pub_date TEXT,
            last_build_date TEXT
        )",
        (),
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS episodes (
            id INTEGER PRIMARY KEY,
            podcast_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            guid TEXT NOT NULL UNIQUE,
            description TEXT NOT NULL,
            pub_date TEXT,
            link TEXT,
            enclosure_url TEXT NOT NULL UNIQUE,
            enclosure_length TEXT,
            enclosure_mime_type TEXT,
            FOREIGN KEY (podcast_id) REFERENCES podcasts(id)
        )",
        (),
    )?;
    Ok(conn)
}

pub fn insert_podcast(
    conn: &rusqlite::Connection,
    podcast: &mut podcast::Podcast,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT OR IGNORE INTO podcasts (title, description, rss_url, link, language, pub_date, last_build_date)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        rusqlite::params![
            podcast.title,
            podcast.description,
            podcast.rss_url,
            podcast.link,
            podcast.language,
            podcast.pub_date,
            podcast.last_build_date,
        ],
    )?;
    podcast.id = conn.last_insert_rowid();
    for episode in &podcast.episodes {
        insert_episode(conn, episode, podcast.id)?;
    }
    Ok(())
}

fn insert_episode(
    conn: &rusqlite::Connection,
    episode: &podcast::Episode,
    podcast_id: i64,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT OR IGNORE INTO episodes (podcast_id, title, guid, description, pub_date, link, enclosure_url, enclosure_length, enclosure_mime_type)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            podcast_id,
            episode.title,
            episode.guid,
            episode.description,
            episode.pub_date,
            episode.link,
            episode.enclosure.as_ref().map(|e| &e.url),
            episode.enclosure.as_ref().map(|e| &e.length),
            episode.enclosure.as_ref().map(|e| &e.mime_type),
        ],
    )?;
    Ok(())
}
