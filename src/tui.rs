use crate::{db, podcast};
use crossterm::{event, execute, terminal};
use ratatui::{prelude::*, widgets};
use std::io;

trait Page {
    fn render(&mut self, f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect);
}

struct App {
    #[allow(dead_code)]
    podcasts: std::rc::Rc<Vec<podcast::Podcast>>,
    layout: Layout,
    podcast_page: PodcastsPage,
}

pub fn start() -> Result<(), io::Error> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let size = f.size();
        let loading = widgets::Paragraph::new("Loading...").block(
            widgets::Block::default()
                .title("dipper")
                .borders(widgets::Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title_style(Style::default().fg(Color::Yellow)),
        );
        f.render_widget(loading, size);
    })?;

    let mut app = App::new("test.db".to_string());
    app.run(&mut terminal);

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

impl App {
    fn new(db_name: String) -> App {
        let conn = db::init_db(&db_name).unwrap();
        let pods = std::rc::Rc::new(db::fetch_all_podcasts_and_episodes(&conn).unwrap());
        App {
            podcasts: pods.clone(),
            podcast_page: PodcastsPage::new(pods.clone()),
            layout: Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Min(0)].as_ref()),
        }
    }

    fn run(&mut self, term: &mut Terminal<CrosstermBackend<io::Stdout>>) {
        loop {
            term.draw(|f| self.render(f)).unwrap();
            if !self.handle_input() {
                break;
            }
        }
    }

    fn render(&mut self, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
        let size = f.size();
        let rects = self.layout.split(size);
        self.render_tab_widget(f, rects[0]);
        self.podcast_page.render(f, rects[1]);
    }

    fn handle_input(&mut self) -> bool {
        if let event::Event::Key(key) = event::read().unwrap() {
            match key.code {
                event::KeyCode::Char('q') => return false,
                event::KeyCode::Char('j') => {
                    self.podcast_page.select_next();
                }
                event::KeyCode::Char('k') => {
                    self.podcast_page.select_previous();
                }
                event::KeyCode::Char('h') => {
                    self.podcast_page.focus_pod_list();
                }
                event::KeyCode::Char('l') => {
                    self.podcast_page.focus_ep_list();
                }
                _ => (),
            }
        }
        true
    }

    fn render_tab_widget(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect) {
        let tabs = widgets::Tabs::new(vec!["Podcasts", "Episodes"])
            .block(
                widgets::Block::default()
                    .borders(widgets::Borders::TOP)
                    .blue()
                    .title("dipper")
                    .title_style(Style::default().fg(Color::Yellow)),
            )
            .divider("|")
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(0);
        f.render_widget(tabs, rect)
    }
}

struct PodcastsPage {
    pods: std::rc::Rc<Vec<podcast::Podcast>>,
    pod_list_state: widgets::ListState,
    ep_list_state: Vec<widgets::ListState>,
    pod_list_focused: bool,
    vsplit: Layout,
    hsplit: Layout,
}

impl PodcastsPage {
    fn new(pods: std::rc::Rc<Vec<podcast::Podcast>>) -> PodcastsPage {
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

    fn select_next(&mut self) {
        if self.pod_list_focused {
            self.select_next_podcast();
        } else {
            self.select_next_episode();
        }
    }

    fn select_previous(&mut self) {
        if self.pod_list_focused {
            self.select_previous_podcast();
        } else {
            self.select_previous_episode();
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

    fn select_previous_podcast(&mut self) {
        if let Some(i) = self.pod_list_state.selected() {
            if i > 0 {
                self.pod_list_state.select(Some(i - 1));
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

    fn focus_pod_list(&mut self) {
        self.pod_list_focused = true;
    }

    fn focus_ep_list(&mut self) {
        self.pod_list_focused = false;
    }

    fn style_if_focus(&self, invert: bool) -> Style {
        if self.pod_list_focused ^ invert {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        }
    }

    fn render_podcasts_widget(&mut self, f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect) {
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
                    .borders(widgets::Borders::TOP)
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
                    .borders(widgets::Borders::TOP)
                    .border_style(self.style_if_focus(true))
                    .title("Episodes")
                    .title_style(self.style_if_focus(true)),
            );
        f.render_stateful_widget(ep_list, rect, &mut self.ep_list_state[selected])
    }
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
