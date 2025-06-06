use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use std::io;
use std::time::Duration;

use crate::app::{Pomodoro};

pub async fn handle_input(
    app_state: Arc<Mutex<Pomodoro>>,
    shutdown_tx: mpsc::Sender<()>,
) -> io::Result<()> {
    loop {
        // Poll for events with a timeout
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // Only handle key press events (not releases)
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') => {
                        if let Err(e) = shutdown_tx.send(()).await {
                            eprintln!("Failed to send shutdown signal: {}", e);
                        }
                        break;
                    }
                    KeyCode::Char('s') => {
                        let mut app = app_state.lock().map_err(|_| {
                            io::Error::new(io::ErrorKind::Other, "Failed to acquire lock")
                        })?;
                        app.toggle();
                    }
                    KeyCode::Char('r') => {
                        let mut app = app_state.lock().map_err(|_| {
                            io::Error::new(io::ErrorKind::Other, "Failed to acquire lock")
                        })?;
                        app.reset(25);
                    }
                    _ => {}
                }
            }
        }
        
        // Small sleep to prevent busy waiting
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    Ok(())
}
