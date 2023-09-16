use crate::podcast;
use crate::tui::Page;
use ratatui::{prelude::*, widgets};
use std::io;
use std::rc::Rc;
use std::vec::Vec;

pub struct EpisodesPage {
    #[allow(dead_code)]
    eps: Vec<Rc<podcast::Episode>>,
}

impl Page for EpisodesPage {
    fn render(&mut self, f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect) {}
}

impl EpisodesPage {
    pub fn new(pods: Rc<Vec<podcast::Podcast>>) -> EpisodesPage {
        let mut eps = Vec::new();
        for pod in pods.iter() {
            for ep in pod.episodes.iter() {
                eps.push(Rc::new(ep.clone()));
            }
        }
        eps.sort_by(|a, b| a.pub_date.cmp(&b.pub_date));
        EpisodesPage { eps }
    }
}
