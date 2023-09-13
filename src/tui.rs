use crate::{db, feed, podcast};
use crossterm::{event, execute, terminal};
use ratatui::{prelude::*, widgets};
use std::io;
use std::thread;
use std::time::Duration;

pub fn start() -> Result<(), io::Error> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(main_widget)?;
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if let event::KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }
    }

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn main_widget(f: &mut Frame<CrosstermBackend<io::Stdout>>) {
    let size = f.size();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(1), Constraint::Min(0)].as_ref())
        .split(size);
    let title = widgets::Block::default()
        .title("dipper")
        .title_alignment(Alignment::Center)
        .title_style(Style::default().yellow())
        .borders(widgets::Borders::TOP)
        .border_type(widgets::BorderType::Double);
    f.render_widget(title, layout[0]);

    let conn = db::init_db(&"test.db".to_string()).unwrap();
    let pods = db::fetch_all_podcasts(&conn).unwrap();
    let mut items = Vec::new();
    for pod in pods {
        items.push(widgets::ListItem::new(pod.title));
    }
    let pods = widgets::List::new(items).block(
        widgets::Block::default()
            .borders(widgets::Borders::TOP)
            .title("Podcasts"),
    );

    f.render_widget(pods, layout[1]);
}
