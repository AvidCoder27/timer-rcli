mod quit_checker;
use std::{
    io::{self, Write, stdout},
    process::Command,
    thread,
    time::{Duration, Instant},
};

use termion::{self, raw::IntoRawMode};

fn main() {
    let total_time = read_duration_from_user();
    let start_time = Instant::now();

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}Input 'q' anytime to exit.{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::style::Reset,
        termion::cursor::Goto(1, 2)
    )
    .unwrap();

    stdout.flush().unwrap();
    let quit_checker = quit_checker::QuitChecker::new();
    let frame_duration = Duration::from_millis(250);

    loop {
        if quit_checker.should_quit() {
            break;
        }
        let elapsed = start_time.elapsed();
        let remaining = match total_time.checked_sub(elapsed) {
            Some(dur) => dur,
            None => {
                write!(
                    stdout,
                    "{}Time's up!                {}",
                    termion::cursor::Goto(1, 2),
                    termion::cursor::Goto(1, 3)
                )
                .unwrap();
                stdout.flush().unwrap();
                Command::new("espeak")
                    .arg(format!("\"Time's up!\""))
                    .spawn()
                    .expect("Failed to speak");
                break;
            }
        };
        let remaining_seconds = remaining.as_secs() + 1;
        let remaining_minutes = remaining_seconds / 60;
        let remaining_hours = remaining_minutes / 60;
        write!(
            stdout,
            "{}{:02}:{:02}:{:02}",
            termion::cursor::Goto(1, 2),
            remaining_hours,
            remaining_minutes % 60,
            remaining_seconds % 60,
        )
        .unwrap();
        stdout.flush().unwrap();
        thread::sleep(frame_duration);
    }
}

pub fn read_duration_from_user() -> Duration {
    fn read_number(prompt: &str) -> u64 {
        loop {
            print!("{}", prompt);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_ok() {
                let input = input.trim();
                if input.is_empty() {
                    return 0;
                } else if let Ok(value) = input.parse::<u64>() {
                    return value;
                } else {
                    println!("Please enter a valid number.");
                }
            }
        }
    }

    let hours = read_number("Enter hours: ");
    let minutes = read_number("Enter minutes: ");
    let seconds = read_number("Enter seconds: ");

    Duration::from_secs(hours * 3600 + minutes * 60 + seconds)
}
