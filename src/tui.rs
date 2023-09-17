mod episodes_page;
mod pods_page;

use crate::tui::episodes_page::EpisodesPage;
use crate::tui::pods_page::PodcastsPage;
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
    selected_tab: usize,
    podcast_page: PodcastsPage,
    episodes_page: EpisodesPage,
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
                .border_type(widgets::BorderType::Thick)
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
        let pods = std::rc::Rc::new(App::load_podcasts(db_name));
        App {
            podcasts: pods.clone(),
            podcast_page: PodcastsPage::new(pods.clone()),
            episodes_page: EpisodesPage::new(pods.clone()),
            layout: Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(2), Constraint::Min(0)].as_ref()),
            selected_tab: 0,
        }
    }

    fn load_podcasts(db_name: String) -> Vec<podcast::Podcast> {
        let conn = db::init_db(&db_name).unwrap();
        let mut pods = db::fetch_all_podcasts_and_episodes(&conn).unwrap();
        pods.sort_by(|a, b| a.title.cmp(&b.title));
        pods
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
        match self.selected_tab {
            0 => {
                self.podcast_page.render(f, rects[1]);
            }
            1 => {
                self.episodes_page.render(f, rects[1]);
            }
            _ => (),
        }
    }

    fn handle_input(&mut self) -> bool {
        if let event::Event::Key(key) = event::read().unwrap() {
            match key.code {
                event::KeyCode::Char('q') => return false,
                event::KeyCode::Char('j') => match self.selected_tab {
                    0 => self.podcast_page.select_next(),
                    1 => self.episodes_page.select_next(),
                    _ => (),
                },
                event::KeyCode::Char('k') => match self.selected_tab {
                    0 => self.podcast_page.select_previous(),
                    1 => self.episodes_page.select_previous(),
                    _ => (),
                },
                event::KeyCode::Char('h') => {
                    self.podcast_page.focus_pod_list();
                }
                event::KeyCode::Char('l') => {
                    self.podcast_page.focus_ep_list();
                }
                event::KeyCode::Tab => {
                    self.selected_tab = (self.selected_tab + 1) % 2;
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
            .highlight_style(Style::default().fg(Color::Yellow).bg(Color::DarkGray))
            .select(self.selected_tab);
        f.render_widget(tabs, rect)
    }
}
