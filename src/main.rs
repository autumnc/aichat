mod ai;
mod ai_models;
mod app;
mod config;
mod events;
mod i18n;
mod simple;
mod ui;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    env,
    io,
    time::{Duration, Instant},
};

use ai_models::AIModel;
use i18n::Language;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 && args[1] == "--simple" {
        let model = if args.len() >= 3 {
            AIModel::from_str(&args[2]).unwrap_or_else(|| {
                eprintln!("Unknown model '{}', using deepseek", args[2]);
                AIModel::DeepSeek
            })
        } else {
            AIModel::DeepSeek
        };
        return simple::run_simple_mode(model, Language::English).await;
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    res
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    let mut last_blink_update = Instant::now();
    loop {
        if last_blink_update.elapsed() >= Duration::from_millis(500) {
            app.update_cursor_blink();
            last_blink_update = Instant::now();
        }
        terminal.draw(|f| ui::render(app, f))?;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if events::handle_key_event(key, app) {
                    return Ok(());
                }
            }
        }
    }
}
