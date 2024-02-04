use crate::podcast;
use crate::tui::Page;
use anyhow::Result;
use libmpv::MpvHandler;
use ratatui::widgets::ListItem;
use ratatui::{prelude::*, widgets};
use std::io;
use std::rc::Rc;
use std::vec::Vec;

pub struct PlayerPage {
    queue: Vec<podcast::Episode>,
    mpv: MpvHandler,
}

impl Page for PlayerPage {
    fn render(&mut self, f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect) {
        let mut titles = Vec::new();
        for ep in self.queue.iter() {
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
        f.render_widget(list, rect);
    }
}

impl PlayerPage {
    pub fn new() -> Result<PlayerPage> {
        let mpv = MpvHandler::new()?;
        PlayerPage {
            queue: Vec::new(),
            mpv,
        }
    }
}
