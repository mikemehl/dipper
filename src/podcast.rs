use std::option::Option;

#[derive(Debug)]
pub struct Podcast {
    pub title: String,
    pub description: String,
    pub rss_url: String,
    pub link: Option<String>,
    pub language: Option<String>,
    pub pub_date: Option<String>,
    pub last_build_date: Option<String>,
    pub episodes: Vec<Episode>,
}

impl Podcast {
    pub fn new(title: String, description: String, rss_url: String) -> Podcast {
        Podcast {
            title,
            description,
            rss_url,
            link: None,
            language: None,
            pub_date: None,
            last_build_date: None,
            episodes: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Episode {
    pub title: String,
    pub guid: String,
    pub description: String,
    pub pub_date: Option<String>,
    pub link: Option<String>,
    pub enclosure: Option<Enclosure>,
}

impl Episode {
    pub fn new(title: String, guid: String, description: String) -> Episode {
        Episode {
            title,
            guid,
            description,
            pub_date: None,
            link: None,
            enclosure: None,
        }
    }
}

#[derive(Debug)]
pub struct Enclosure {
    pub url: String,
    pub length: Option<String>,
    pub mime_type: Option<String>,
}

impl Enclosure {
    pub fn new(url: String) -> Enclosure {
        Enclosure {
            url,
            length: None,
            mime_type: None,
        }
    }
}
