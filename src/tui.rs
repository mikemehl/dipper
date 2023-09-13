use crate::{db, podcast};
use crossterm::{event, execute, terminal};
use ratatui::{prelude::*, widgets};
use std::io;

enum Page {
    Main { pods: Vec<podcast::Podcast> },
    Episodes { pod: podcast::Podcast },
    DetailedEpisode { ep: podcast::Episode },
}

pub fn start() -> Result<(), io::Error> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(render_main_page)?;
        if let event::Event::Key(key) = event::read().unwrap() {
            if let event::KeyCode::Char('q') = key.code {
                break;
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn render(_f: &mut Frame<CrosstermBackend<io::Stdout>>, _page: Page) {
    todo!("Match on the Page enum and use that to render the current state.");
}

fn render_title_widget(f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect) {
    let title = widgets::Block::default()
        .title("dipper")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().yellow())
        .borders(widgets::Borders::TOP)
        .border_type(widgets::BorderType::Double);

    f.render_widget(title, rect);
}

fn render_podcasts_widget(
    f: &mut Frame<CrosstermBackend<io::Stdout>>,
    rect: Rect,
    pods: &Vec<podcast::Podcast>,
) {
    let mut items = Vec::new();
    for pod in pods {
        items.push(widgets::ListItem::new(pod.title.clone()));
    }
    let pods = widgets::List::new(items)
        .bg(Color::Black)
        .fg(Color::Cyan)
        .block(
            widgets::Block::default()
                .borders(widgets::Borders::TOP)
                .title("Podcasts"),
        );
    f.render_widget(pods, rect);
}

fn render_main_page(f: &mut Frame<CrosstermBackend<io::Stdout>>) {
    let size = f.size();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(1), Constraint::Min(0)].as_ref())
        .split(size);
    render_title_widget(f, layout[0]);

    let conn = db::init_db(&"test.db".to_string()).unwrap();
    let pods = db::fetch_all_podcasts(&conn).unwrap();
    render_podcasts_widget(f, layout[1], &pods);
}
