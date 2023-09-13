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
    terminal.draw(|f| {
        let size = f.size();
        let block = widgets::Block::default()
            .title("Block")
            .borders(widgets::Borders::ALL)
            .border_type(widgets::BorderType::Rounded);
        f.render_widget(block, size);
    })?;

    thread::spawn(|| loop {
        let _ = event::read();
    });

    thread::sleep(Duration::from_secs(5));

    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
