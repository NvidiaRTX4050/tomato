use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    Running,
    Paused,
    Stopped,
}

#[derive(Debug)]
pub struct Pomodoro {
    pub remaining: Duration,
    pub state: TimerState,
}

impl Pomodoro {
    pub fn new(minutes: u64) -> Self {
        Self {
            remaining: Duration::from_secs(minutes * 60),
            state: TimerState::Stopped,
        }
    }

    pub fn reset(&mut self, minutes: u64) {
        self.remaining = Duration::from_secs(minutes * 60);
        self.state = TimerState::Stopped;
    }

    pub fn tick(&mut self) {
        if self.state == TimerState::Running && self.remaining.as_secs() > 0 {
            self.remaining -= Duration::from_secs(1);
        }
    }

    pub fn toggle(&mut self) {
        self.state = match self.state {
            TimerState::Running => TimerState::Paused,
            TimerState::Paused => TimerState::Running,
            TimerState::Stopped => TimerState::Running,
        };
    }
}
