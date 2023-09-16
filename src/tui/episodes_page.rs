use crate::podcast;
use crate::tui::Page;
use ratatui::{prelude::*, widgets};
use std::io;
use std::vec::Vec;

pub struct EpisodesPage {
    #[allow(dead_code)]
    eps: Vec<podcast::Episode>,
}

impl Page for EpisodesPage {
    fn render(&mut self, f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect) {}
}

impl EpisodesPage {
    pub fn new(pods: std::rc::Rc<Vec<podcast::Podcast>>) -> EpisodesPage {
        let eps = Vec::new();
        EpisodesPage { eps }
    }
}
