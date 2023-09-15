use crate::{db, podcast};
use crossterm::{event, execute, terminal};
use ratatui::{prelude::*, widgets};
use std::io;

trait Page {
    fn render(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect);
}

struct App {
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

    let app = App::new("test.db".to_string());
    app.run(&mut terminal);

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

impl App {
    fn new(db_name: String) -> App {
        let conn = db::init_db(&db_name).unwrap();
        let pods = std::rc::Rc::new(db::fetch_all_podcasts(&conn).unwrap());
        App {
            podcasts: pods.clone(),
            podcast_page: PodcastsPage::new(pods),
            layout: Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Min(0)].as_ref()),
        }
    }

    fn run(&self, term: &mut Terminal<CrosstermBackend<io::Stdout>>) {
        loop {
            term.draw(|f| self.render(f)).unwrap();
            if !self.handle_input() {
                break;
            }
        }
    }

    fn render(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>) {
        let size = f.size();
        let rects = self.layout.split(size);
        self.render_tab_widget(f, rects[0]);
        self.podcast_page.render(f, rects[1]);
    }

    fn handle_input(&self) -> bool {
        if let event::Event::Key(key) = event::read().unwrap() {
            if let event::KeyCode::Char('q') = key.code {
                return false;
            }
        }
        true
    }

    fn render_tab_widget(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect) {
        let tabs = widgets::Tabs::new(vec!["Podcasts", "Episodes"])
            .block(
                widgets::Block::default()
                    .borders(widgets::Borders::ALL)
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
    state: widgets::ListState,
}

impl PodcastsPage {
    fn new(pods: std::rc::Rc<Vec<podcast::Podcast>>) -> PodcastsPage {
        PodcastsPage {
            pods,
            state: widgets::ListState::default(),
        }
    }
}

impl Page for PodcastsPage {
    fn render(&self, f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect) {
        let mut items = Vec::new();
        for pod in self.pods.iter() {
            items.push(widgets::ListItem::new(pod.title.clone()));
        }
        let pods = widgets::List::new(items).block(
            widgets::Block::default()
                .borders(widgets::Borders::TOP)
                .title("Podcasts"),
        );
        f.render_widget(pods, rect);
    }
}
