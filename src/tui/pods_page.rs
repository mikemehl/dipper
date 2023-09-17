use crate::podcast;
use crate::tui::Page;
use ratatui::{prelude::*, widgets};
use std::io;

pub struct PodcastsPage {
    pods: std::rc::Rc<Vec<podcast::Podcast>>,
    pod_list_state: widgets::ListState,
    ep_list_state: Vec<widgets::ListState>,
    pod_list_focused: bool,
    vsplit: Layout,
    hsplit: Layout,
}

impl Page for PodcastsPage {
    fn render(&mut self, f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect) {
        let vrects = self.vsplit.split(rect);
        self.render_podcasts_widget(f, vrects[0]);
        let hrects = self.hsplit.split(vrects[1]);
        self.render_desc_widget(f, hrects[0]);
        self.render_episodes_widget(f, hrects[1]);
    }
}

impl PodcastsPage {
    pub fn new(pods: std::rc::Rc<Vec<podcast::Podcast>>) -> PodcastsPage {
        let pod_list_state = widgets::ListState::default().with_selected(Some(0));
        let mut ep_list_state = Vec::<widgets::ListState>::new();
        for _ in 0..pods.len() {
            ep_list_state.push(widgets::ListState::default().with_selected(Some(0)));
        }
        PodcastsPage {
            pods,
            pod_list_state,
            vsplit: Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(33), Constraint::Min(0)].as_ref()),
            hsplit: Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(33), Constraint::Min(0)].as_ref()),
            ep_list_state,
            pod_list_focused: true,
        }
    }

    pub fn select_next(&mut self) {
        if self.pod_list_focused {
            self.select_next_podcast();
        } else {
            self.select_next_episode();
        }
    }

    pub fn page_up(&mut self) {
        if self.pod_list_focused {
            self.select_page_up_podcast();
        } else {
            self.select_page_up_episode();
        }
    }

    pub fn select_previous(&mut self) {
        if self.pod_list_focused {
            self.select_previous_podcast();
        } else {
            self.select_previous_episode();
        }
    }

    pub fn page_down(&mut self) {
        if self.pod_list_focused {
            self.select_page_down_podcast();
        } else {
            self.select_page_down_episode();
        }
    }

    fn select_next_podcast(&mut self) {
        if let Some(i) = self.pod_list_state.selected() {
            if i + 1 < self.pods.len() {
                self.pod_list_state.select(Some(i + 1));
            } else {
                self.pod_list_state.select(Some(0));
            }
        }
    }

    fn select_page_up_podcast(&mut self) {
        if let Some(i) = self.pod_list_state.selected() {
            if i > 10 {
                self.pod_list_state.select(Some(i - 10));
            } else {
                self.pod_list_state.select(Some(0));
            }
        }
    }

    fn select_previous_podcast(&mut self) {
        if let Some(i) = self.pod_list_state.selected() {
            if i > 0 {
                self.pod_list_state.select(Some(i - 1));
            } else {
                self.pod_list_state.select(Some(self.pods.len() - 1));
            }
        }
    }

    fn select_page_down_podcast(&mut self) {
        if let Some(i) = self.pod_list_state.selected() {
            if i + 10 < self.pods.len() {
                self.pod_list_state.select(Some(i + 10));
            } else {
                self.pod_list_state.select(Some(self.pods.len() - 1));
            }
        }
    }

    fn select_next_episode(&mut self) {
        if let Some(i) = self.pod_list_state.selected() {
            if let Some(j) = self.ep_list_state[i].selected() {
                if j + 1 < self.pods[i].episodes.len() {
                    self.ep_list_state[i].select(Some(j + 1));
                } else {
                    self.ep_list_state[i].select(Some(0));
                }
            }
        }
    }

    fn select_page_up_episode(&mut self) {
        if let Some(i) = self.pod_list_state.selected() {
            if let Some(j) = self.ep_list_state[i].selected() {
                if j > 10 {
                    self.ep_list_state[i].select(Some(j - 10));
                } else {
                    self.ep_list_state[i].select(Some(0));
                }
            }
        }
    }

    fn select_previous_episode(&mut self) {
        if let Some(i) = self.pod_list_state.selected() {
            if let Some(j) = self.ep_list_state[i].selected() {
                if j > 0 {
                    self.ep_list_state[i].select(Some(j - 1));
                } else {
                    self.ep_list_state[i].select(Some(self.pods[i].episodes.len() - 1));
                }
            }
        }
    }

    fn select_page_down_episode(&mut self) {
        if let Some(i) = self.pod_list_state.selected() {
            if let Some(j) = self.ep_list_state[i].selected() {
                if j + 10 < self.pods[i].episodes.len() {
                    self.ep_list_state[i].select(Some(j + 10));
                } else {
                    self.ep_list_state[i].select(Some(self.pods[i].episodes.len() - 1));
                }
            }
        }
    }

    pub fn focus_pod_list(&mut self) {
        self.pod_list_focused = true;
    }

    pub fn focus_ep_list(&mut self) {
        self.pod_list_focused = false;
    }

    pub fn style_if_focus(&self, invert: bool) -> Style {
        if self.pod_list_focused ^ invert {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        }
    }

    pub fn render_podcasts_widget(
        &mut self,
        f: &mut Frame<CrosstermBackend<io::Stdout>>,
        rect: Rect,
    ) {
        let mut items = Vec::new();
        for pod in self.pods.iter() {
            items.push(widgets::ListItem::new(pod.title.clone()));
        }
        let pod_list = widgets::List::new(items)
            .highlight_style(self.style_if_focus(false))
            .highlight_symbol(">> ")
            .block(
                widgets::Block::default()
                    .borders(widgets::Borders::TOP)
                    .border_style(self.style_if_focus(false))
                    .title("Podcasts")
                    .title_style(self.style_if_focus(false)),
            );
        f.render_stateful_widget(pod_list, rect, &mut self.pod_list_state);
    }

    fn render_desc_widget(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect) {
        let selected = self.pod_list_state.selected().unwrap();
        let desc = widgets::Paragraph::new(self.pods[selected].description.clone())
            .wrap(widgets::Wrap { trim: false })
            .block(
                widgets::Block::default()
                    .borders(widgets::Borders::TOP | widgets::Borders::LEFT)
                    .title("Description"),
            );
        f.render_widget(desc, rect);
    }

    fn render_episodes_widget(&mut self, f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect) {
        let selected = self.pod_list_state.selected().unwrap();
        let mut items = Vec::new();
        for ep in self.pods[selected].episodes.iter() {
            items.push(widgets::ListItem::new(ep.title.clone()));
        }
        let ep_list = widgets::List::new(items)
            .highlight_style(self.style_if_focus(true))
            .highlight_symbol(">> ")
            .block(
                widgets::Block::default()
                    .borders(widgets::Borders::TOP | widgets::Borders::LEFT)
                    .border_style(self.style_if_focus(true))
                    .title("Episodes")
                    .title_style(self.style_if_focus(true)),
            );
        f.render_stateful_widget(ep_list, rect, &mut self.ep_list_state[selected])
    }
}
