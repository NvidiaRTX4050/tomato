use std::sync::{Arc, Mutex};
use tokio::sync::watch;
use tokio::time::{interval, Duration};
use std::io;

use crate::app::{Pomodoro, TimerState};

pub async fn run_timer(
    app_state: Arc<Mutex<Pomodoro>>,
    mut shutdown: watch::Receiver<bool>,
) -> io::Result<()> {
    let mut ticker = interval(Duration::from_secs(1));

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                let mut pomodoro = app_state.lock().map_err(|_| {
                    io::Error::new(io::ErrorKind::Other, "Failed to acquire lock")
                })?;
                if pomodoro.state == TimerState::Running {
                    pomodoro.tick();
                }
            }
            _ = shutdown.changed() => {
                if *shutdown.borrow() {
                    break;
                }
            }
            else => {
                break;
            }
        }
    }
    Ok(())
}
