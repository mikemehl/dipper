use std::option::Option;

const NO_ID: i64 = -1;

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
    pub id: i64,
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
            id: NO_ID,
        }
    }

    pub fn print(&self, detailed: bool) {
        if detailed {
            self.print_detailed();
        } else {
            self.print_summary();
        }
    }

    pub fn str(&self, detailed: bool) -> String {
        if !detailed {
            format!("{} => {}", self.id, self.title)
        } else {
            format!("{:?}", self)
        }
    }

    fn print_summary(&self) {
        println!("{} => {}", self.id, self.title);
    }

    fn print_detailed(&self) {
        println!("{:?}", self);
    }
}

impl std::fmt::Display for Podcast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{} => {}\nDescription: {}\n",
            self.id, self.title, self.description
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Episode {
    pub title: String,
    pub guid: String,
    pub description: String,
    pub pub_date: Option<String>,
    pub link: Option<String>,
    pub enclosure: Option<Enclosure>,
    pub id: i64,
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
            id: NO_ID,
        }
    }

    pub fn print(&self, detailed: bool) {
        if detailed {
            self.print_detailed();
        } else {
            self.print_summary();
        }
    }

    pub fn str(&self, detailed: bool) -> String {
        if !detailed {
            format!("{} => {}", self.id, self.title)
        } else {
            format!("{:?}", self)
        }
    }

    fn print_summary(&self) {
        println!("{} => {}", self.id, self.title);
    }

    fn print_detailed(&self) {
        println!("{:?}", self);
    }
}

impl std::fmt::Display for Episode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{} => {}\nDescription: {}\n",
            self.id, self.title, self.description
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Enclosure {
    pub url: String,
    pub length: Option<String>,
    pub mime_type: Option<String>,
}
