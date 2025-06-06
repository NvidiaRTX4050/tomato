mod app;
mod timer;
mod input;
mod ui;

use std::{io, sync::{Arc, Mutex}};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use app::Pomodoro;
use ui::draw_ui;
use timer::run_timer;
use input::handle_input;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Initialize shared application state
    let app_state = Arc::new(Mutex::new(Pomodoro::new(25)));

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Graceful shutdown signals
    let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
    let (input_shutdown_tx, mut input_shutdown_rx) = mpsc::channel(1);

    let timer_state = Arc::clone(&app_state);
    let input_state = Arc::clone(&app_state);
    let ui_state = Arc::clone(&app_state);

    // Spawn timer task
    let timer_handle = tokio::spawn(run_timer(timer_state, shutdown_rx));

    // Spawn input handler
    let input_handle = tokio::spawn(handle_input(input_state, input_shutdown_tx));

    // Create UI refresh interval
    let mut refresh_interval = interval(Duration::from_millis(100));

    // Main UI loop
    loop {
        refresh_interval.tick().await;

        if let Err(e) = draw_ui(&mut terminal, Arc::clone(&ui_state)) {
            eprintln!("UI error: {}", e);
            break;
        }

        match input_shutdown_rx.try_recv() {
            Ok(_) => {
                if let Err(e) = shutdown_tx.send(true) {
                    eprintln!("Failed to send shutdown signal: {}", e);
                }
                break;
            }
            Err(mpsc::error::TryRecvError::Empty) => {}
            Err(mpsc::error::TryRecvError::Disconnected) => break,
        }
    }

    // Wait for tasks to complete
    if let Err(e) = timer_handle.await {
        eprintln!("Timer task error: {}", e);
    }
    if let Err(e) = input_handle.await {
        eprintln!("Input handler error: {}", e);
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
