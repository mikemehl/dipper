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

pub fn fetch_all_podcasts(
    conn: &rusqlite::Connection,
) -> Result<Vec<podcast::Podcast>, rusqlite::Error> {
    let mut ret = Vec::new();
    let mut pod_stmt = conn.prepare(
        "SELECT id, title, description, rss_url, link, language, pub_date, last_build_date
        FROM podcasts",
    )?;
    let mut pods = pod_stmt.query_map(rusqlite::params![], |row| {
        Ok(podcast::Podcast {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            rss_url: row.get(3)?,
            link: row.get(4)?,
            language: row.get(5)?,
            pub_date: row.get(6)?,
            last_build_date: row.get(7)?,
            episodes: Vec::new(),
        })
    })?;
    for pod in pods {
        ret.push(pod?);
    }
    Ok(ret)
}

pub fn fetch_episodes(conn: &rusqlite::Connection, id: i64) -> Result<Vec<podcast::Episode>, rusqlite::Error>{
    let mut ret = Vec::new();
    let mut ep_stmt = conn.prepare(
        "SELECT id, title, guid, description, pub_date, link, enclosure_url, enclosure_length, enclosure_mime_type
        FROM episodes
        WHERE podcast_id = ?1",
    )?;
    let eps = ep_stmt.query_map(rusqlite::params![id], |row| {
        Ok(podcast::Episode {
            id: row.get(0)?,
            title: row.get(1)?,
            guid: row.get(2)?,
            description: row.get(3)?,
            pub_date: row.get(4)?,
            link: row.get(5)?,
            enclosure: match row.get(6)? {
                Some(url) => Some(podcast::Enclosure {
                    url,
                    length: row.get(7)?,
                    mime_type: row.get(8)?,
                }),
                None => None,
            },
        })
    })?;
    for ep in eps {
        ret.push(ep?);
    }
    Ok(ret)
}

pub fn fetch_episode(
    conn: &rusqlite::Connection,
    id: i64,
) -> Result<podcast::Episode, rusqlite::Error> {
    let mut ep_stmt = conn.prepare(
        "SELECT id, title, guid, description, pub_date, link, enclosure_url, enclosure_length, enclosure_mime_type
        FROM episodes
        WHERE id = ?1",
    )?;
    let ep = ep_stmt.query_row(rusqlite::params![id], |row| {
        Ok(podcast::Episode {
            id: row.get(0)?,
            title: row.get(1)?,
            guid: row.get(2)?,
            description: row.get(3)?,
            pub_date: row.get(4)?,
            link: row.get(5)?,
            enclosure: match row.get(6)? {
                Some(url) => Some(podcast::Enclosure {
                    url,
                    length: row.get(7)?,
                    mime_type: row.get(8)?,
                }),
                None => None,
            },
        })
    })?;
    Ok(ep)
}

pub fn fetch_podcast(
    conn: &rusqlite::Connection,
    id: i64,
) -> Result<podcast::Podcast, rusqlite::Error> {
    let mut pod_stmt = conn.prepare(
        "SELECT id, title, description, rss_url, link, language, pub_date, last_build_date
        FROM podcasts
        WHERE id = ?1",
    )?;
    let pod = pod_stmt.query_row(rusqlite::params![id], |row| {
        Ok(podcast::Podcast {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            rss_url: row.get(3)?,
            link: row.get(4)?,
            language: row.get(5)?,
            pub_date: row.get(6)?,
            last_build_date: row.get(7)?,
            episodes: Vec::new(),
        })
    })?;
    Ok(pod)
}

pub fn fetch_podcast_and_episodes(
    conn: &rusqlite::Connection,
    id: i64,
) -> Result<podcast::Podcast, rusqlite::Error> {
    let mut pod = fetch_podcast(conn, id)?;
    let mut ep_stmt = conn.prepare(
        "SELECT id, title, guid, description, pub_date, link, enclosure_url, enclosure_length, enclosure_mime_type
        FROM episodes
        WHERE podcast_id = ?1")?;
    ep_stmt
        .query_map(rusqlite::params![id], |row| {
            Ok(podcast::Episode {
                id: row.get(0)?,
                title: row.get(1)?,
                guid: row.get(2)?,
                description: row.get(3)?,
                pub_date: row.get(4)?,
                link: row.get(5)?,
                enclosure: match row.get(6)? {
                    Some(url) => Some(podcast::Enclosure {
                        url,
                        length: row.get(7)?,
                        mime_type: row.get(8)?,
                    }),
                    None => None,
                },
            })
        })?
        .for_each(|ep| {
            pod.episodes.push(ep.unwrap());
        });
    Ok(pod)
}

pub fn insert_episode(
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

pub fn remove_podcast(conn: &rusqlite::Connection, id: i64) -> Result<(), rusqlite::Error> {
    conn.execute(
        "DELETE FROM podcasts
        WHERE id = ?1",
        rusqlite::params![id],
    )?;
    conn.execute(
        "DELETE FROM episodes
        WHERE podcast_id = ?1",
        rusqlite::params![id],
    )?;
    Ok(())
}

pub fn search_podcasts(conn: &rusqlite::Connection, term: String) -> Result<Vec<podcast::Podcast>, rusqlite::Error> {
    let mut search_query = conn.prepare(
        "SELECT id, title, description, rss_url, link, language, pub_date, last_build_date
        FROM podcasts
        WHERE title LIKE ?1 OR description LIKE ?1")?;
    let mut ret = Vec::new();
    let pods = search_query.query_map(rusqlite::params!["%".to_string() + &term + "%"], |row| {
        Ok(podcast::Podcast {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            rss_url: row.get(3)?,
            link: row.get(4)?,
            language: row.get(5)?,
            pub_date: row.get(6)?,
            last_build_date: row.get(7)?,
            episodes: Vec::new(),
        })
    })?;
    for pod in pods {
        ret.push(pod?);
    }
    Ok(ret)
}

pub  fn search_episodes(conn: &rusqlite::Connection, term: String, arg: i64) -> Result<Vec<podcast::Episode>, rusqlite::Error> {
    let query_string = match arg {
        0 => "SELECT id, title, guid, description, pub_date, link, enclosure_url, enclosure_length, enclosure_mime_type
        FROM episodes
        WHERE title LIKE ?1 OR description LIKE ?1",
        _ => "SELECT id, title, guid, description, pub_date, link, enclosure_url, enclosure_length, enclosure_mime_type
        FROM episodes
        WHERE title LIKE ?1 OR description LIKE ?1 OR podcast_id = ?2",
    };
    let params = rusqlite::params!["%".to_string() + &term + "%", arg];
    let mut search_query = conn.prepare(query_string)?;
    let mut ret = Vec::new();
    let eps = search_query.query_map(params, |row| {
        Ok(podcast::Episode {
            id: row.get(0)?,
            title: row.get(1)?,
            guid: row.get(2)?,
            description: row.get(3)?,
            pub_date: row.get(4)?,
            link: row.get(5)?,
            enclosure: match row.get(6)? {
                Some(url) => Some(podcast::Enclosure {
                    url,
                    length: row.get(7)?,
                    mime_type: row.get(8)?,
                }),
                None => None,
            },
        })
    })?;
    for ep in eps {
        ret.push(ep?);
    }
    Ok(ret)
}
