use std::sync::{Arc, Mutex};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction, Alignment},
    style::{Style, Color},
    text::{Line, Span},
    Frame,
};
use std::io::{self, Stdout};

use crate::app::Pomodoro;

const DIGIT_HEIGHT: usize = 5;

fn get_big_digit(n: u8) -> [&'static str; DIGIT_HEIGHT] {
    match n {
        0 => [
            "█████",
            "█   █",
            "█   █",
            "█   █",
            "█████",
        ],
        1 => [
            "  █  ",
            " ██  ",
            "  █  ",
            "  █  ",
            "█████",
        ],
        2 => [
            "█████",
            "    █",
            "█████",
            "█    ",
            "█████",
        ],
        3 => [
            "█████",
            "    █",
            "█████",
            "    █",
            "█████",
        ],
        4 => [
            "█   █",
            "█   █",
            "█████",
            "    █",
            "    █",
        ],
        5 => [
            "█████",
            "█    ",
            "█████",
            "    █",
            "█████",
        ],
        6 => [
            "█████",
            "█    ",
            "█████",
            "█   █",
            "█████",
        ],
        7 => [
            "█████",
            "    █",
            "   █ ",
            "  █  ",
            " █   ",
        ],
        8 => [
            "█████",
            "█   █",
            "█████",
            "█   █",
            "█████",
        ],
        9 => [
            "█████",
            "█   █",
            "█████",
            "    █",
            "█████",
        ],
        _ => [
            "     ",
            "     ",
            "     ",
            "     ",
            "     ",
        ],
    }
}

fn render_big_time(minutes: u64, seconds: u64) -> Vec<Line<'static>> {
    let min_tens = ((minutes / 10) % 10) as u8;
    let min_ones = (minutes % 10) as u8;
    let sec_tens = ((seconds / 10) % 10) as u8;
    let sec_ones = (seconds % 10) as u8;

    let digits = [
        get_big_digit(min_tens),
        get_big_digit(min_ones),
        ["     ", "  █  ", "     ", "  █  ", "     "], // colon
        get_big_digit(sec_tens),
        get_big_digit(sec_ones),
    ];

    let mut lines = Vec::with_capacity(DIGIT_HEIGHT);
    for row in 0..DIGIT_HEIGHT {
        let mut line = String::new();
        for digit in &digits {
            line.push_str(digit[row]);
            line.push_str("  "); // Add spacing between digits
        }
        lines.push(Line::from(Span::styled(
            line,
            Style::default().fg(Color::Yellow)
        )));
    }
    lines
}

fn render(f: &mut Frame, pomodoro: &Pomodoro) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(8),  // Timer
            Constraint::Length(3),  // Status
            Constraint::Length(3),  // Help
            Constraint::Min(0),
        ].as_ref())
        .split(size);

    let minutes = pomodoro.remaining.as_secs() / 60;
    let seconds = pomodoro.remaining.as_secs() % 60;
    
    // Render big digits
    let time_display = render_big_time(minutes, seconds);
    let timer = Paragraph::new(time_display)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

    let state_label = match pomodoro.state {
        crate::app::TimerState::Running => "Running",
        crate::app::TimerState::Paused => "Paused",
        crate::app::TimerState::Stopped => "Stopped",
    };

    let status = Paragraph::new(state_label)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Status"));

    let help = Paragraph::new(Line::from("s: Start/Pause | r: Reset | q: Quit"))
        .alignment(Alignment::Center);

    f.render_widget(timer, chunks[0]);
    f.render_widget(status, chunks[1]);
    f.render_widget(help, chunks[2]);
}

/// Render the main UI layout
pub fn draw_ui(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app_state: Arc<Mutex<Pomodoro>>,
) -> io::Result<()> {
    let pomodoro = app_state.lock().map_err(|_| {
        io::Error::new(io::ErrorKind::Other, "Failed to acquire lock")
    })?;

    terminal.draw(|f| render(f, &pomodoro))?;

    Ok(())
}
