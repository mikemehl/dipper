use rusqlite;

pub fn init_db(dbfname: String) -> Result<rusqlite::Connection, rusqlite::Error> {
    let conn = rusqlite::Connection::open(dbfname)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS podcasts (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            rss_url TEXT NOT NULL,
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
            guid TEXT NOT NULL,
            description TEXT NOT NULL,
            pub_date TEXT,
            link TEXT,
            enclosure_url TEXT,
            enclosure_length TEXT,
            enclosure_mime_type TEXT,
            FOREIGN KEY (podcast_id) REFERENCES podcasts(id)
        )",
        (),
    )?;
    Ok(conn)
}
