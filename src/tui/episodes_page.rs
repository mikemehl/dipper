use crate::podcast;
use crate::tui::Page;
use ratatui::widgets::ListItem;
use ratatui::{prelude::*, widgets};
use std::io;
use std::rc::Rc;
use std::vec::Vec;

pub struct EpisodesPage {
    #[allow(dead_code)]
    eps: Vec<Rc<podcast::Episode>>,
    ep_list_state: widgets::ListState,
}

impl Page for EpisodesPage {
    fn render(&mut self, f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect) {
        let mut titles = Vec::new();
        for ep in self.eps.iter() {
            titles.push(ListItem::new(ep.title.clone()));
        }
        let list = widgets::List::new(titles)
            .highlight_style(Style::default().fg(Color::Yellow))
            .highlight_symbol(">> ")
            .block(
                widgets::Block::default()
                    .title("Episodes")
                    .borders(widgets::Borders::ALL),
            );
        f.render_stateful_widget(list, rect, &mut self.ep_list_state);
    }
}

impl EpisodesPage {
    pub fn new(pods: Rc<Vec<podcast::Podcast>>) -> EpisodesPage {
        let mut eps = Vec::new();
        for pod in pods.iter() {
            for ep in pod.episodes.iter() {
                eps.push(Rc::new(ep.clone()));
            }
        }
        eps.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));
        let ep_list_state = widgets::ListState::default().with_selected(Some(0));
        EpisodesPage { eps, ep_list_state }
    }

    pub fn select_next(&mut self) {
        if let Some(i) = self.ep_list_state.selected() {
            if i + 1 < self.eps.len() {
                self.ep_list_state.select(Some(i + 1));
            } else {
                self.ep_list_state.select(Some(0));
            }
        }
    }

    pub fn page_up(&mut self) {
        if let Some(i) = self.ep_list_state.selected() {
            if i > 10 {
                self.ep_list_state.select(Some(i - 10));
            } else {
                self.ep_list_state.select(Some(0));
            }
        }
    }

    pub fn select_previous(&mut self) {
        if let Some(i) = self.ep_list_state.selected() {
            if i > 0 {
                self.ep_list_state.select(Some(i - 1));
            } else {
                self.ep_list_state.select(Some(self.eps.len() - 1));
            }
        }
    }

    pub fn page_down(&mut self) {
        if let Some(i) = self.ep_list_state.selected() {
            if i + 10 < self.eps.len() {
                self.ep_list_state.select(Some(i + 10));
            } else {
                self.ep_list_state.select(Some(self.eps.len() - 1));
            }
        }
    }
}
