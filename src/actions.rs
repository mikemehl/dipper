use crate::db;
use crate::feed;
use anyhow::Result;
use opml::OPML;
use std::sync::mpsc;

pub trait Action {
    fn execute(&self, tx: mpsc::Sender<String>, conn: rusqlite::Connection) -> Result<()>;
}

pub struct List {
    pub id: Option<i64>,
    pub detailed: bool,
    pub limit: Option<i64>,
}

impl Action for List {
    fn execute(self: &List, tx: mpsc::Sender<String>, conn: rusqlite::Connection) -> Result<()> {
        let limit = if let Some(l) = self.limit { l } else { 0 };
        if let Some(id) = self.id {
            let pod = db::fetch_podcast(&conn, id)?;
            tx.send(pod.str(self.detailed))?;
            for ep in pod.episodes.iter().take(if limit > 0 {
                limit as usize
            } else {
                pod.episodes.len()
            }) {
                tx.send(ep.str(self.detailed))?;
            }
        } else {
            let pods = db::fetch_all_podcasts(&conn)?;
            for pod in pods.iter().take(if limit > 0 {
                limit as usize
            } else {
                pods.len()
            }) {
                tx.send(pod.str(self.detailed))?;
            }
        }
        Ok(())
    }
}

pub struct Add {
    pub url: String,
}

impl Action for Add {
    fn execute(self: &Add, tx: mpsc::Sender<String>, conn: rusqlite::Connection) -> Result<()> {
        tx.send(format!("Fetching {}...", self.url))?;
        let rss = feed::fetch_rss(&self.url)?;
        let mut pod = feed::parse_rss(&self.url, &rss)?;
        db::insert_podcast(&conn, &mut pod)?;
        tx.send(format!("Added {}.", pod.title))?;
        Ok(())
    }
}

pub struct Episodes {
    pub id: i64,
    pub detailed: bool,
    pub limit: Option<i64>,
}

impl Action for Episodes {
    fn execute(
        self: &Episodes,
        tx: mpsc::Sender<String>,
        conn: rusqlite::Connection,
    ) -> Result<()> {
        let pod = db::fetch_podcast_and_episodes(&conn, self.id)?;
        if let Some(limit) = self.limit {
            for ep in pod.episodes.iter().take(limit as usize) {
                tx.send(ep.str(self.detailed))?;
            }
        } else {
            for ep in pod.episodes {
                tx.send(ep.str(self.detailed))?;
            }
        }
        Ok(())
    }
}

pub struct Update {
    pub id: Option<i64>,
}

impl Action for Update {
    fn execute(&self, tx: mpsc::Sender<String>, conn: rusqlite::Connection) -> Result<()> {
        if let Some(id) = self.id {
            let pod = db::fetch_podcast(&conn, id)?;
            tx.send(format!("Updating {}...", pod.title))?;
            let rss = feed::fetch_rss(&pod.rss_url)?;
            let pod = feed::parse_rss(&pod.rss_url, &rss)?;
            for ep in pod.episodes {
                db::insert_episode(&conn, &ep, id)?;
            }
            tx.send(format!("Updated {}.", pod.title))?;
        } else {
            let pods = db::fetch_all_podcasts(&conn)?;
            for pod in pods {
                tx.send(format!("Updating {}...", pod.title))?;
                let rss = feed::fetch_rss(&pod.rss_url);
                if let Ok(rss) = rss {
                    if let Ok(pod) = feed::parse_rss(&pod.rss_url, &rss) {
                        for ep in pod.episodes {
                            db::insert_episode(&conn, &ep, pod.id)?;
                        }
                        tx.send(format!("Updated podcast {}.", pod.title))?;
                    } else {
                        tx.send(format!("Failed to update podcast {}.", pod.title))?;
                    }
                } else {
                    tx.send(format!("Failed to update podcast {}.", pod.title))?;
                }
            }
        }
        Ok(())
    }
}

pub struct Remove {
    pub id: i64,
}

// impl Action for Remove {
//     fn execute(&self, conn: rusqlite::Connection) -> Result<String> {
//         todo!()
//     }
// }
//
// pub struct Download {
//     pub id: i64,
// }
//
// impl Action for Download {
//     fn execute(&self, conn: rusqlite::Connection) -> Result<String> {
//         todo!()
//     }
// }
//
// pub struct Search {
//     pub term: String,
//     pub detailed: bool,
// }
//
// impl Action for Search {
//     fn execute(&self, conn: rusqlite::Connection) -> Result<String> {
//         todo!()
//     }
// }
//
// pub struct Import {
//     pub file: String,
// }
//
// impl Action for Import {
//     fn execute(&self, conn: rusqlite::Connection) -> Result<String> {
//         todo!()
//     }
// }
//
// pub struct Export {
//     pub file: String,
// }
//
// impl Action for Export {
//     fn execute(&self, conn: rusqlite::Connection) -> Result<String> {
//         todo!()
//     }
// }
//
// pub struct Play {
//     pub id: i64,
// }
//
// impl Action for Play {
//     fn execute(&self, conn: rusqlite::Connection) -> Result<String> {
//         todo!()
//     }
// }
//
// // pub struct Pause {}
// // pub struct FastForward {}
// // pub struct Rewind {}
// // pub struct SkipNext {}
// // pub struct SkipPrev {}
