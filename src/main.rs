// #![allow(dead_code)]
use std::thread::sleep;
use std::time::{Duration, Instant};

mod command;
mod io_utils;
mod ring;
mod state;
mod term_utils;
mod timer;

use crate::command::Command;
use crate::io_utils::write;
use crate::ring::RingBuffer;
use crate::state::State;
use crate::term_utils::*;
use crate::timer::{Timer, MINUTE, SECOND};

use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread;

const POMODORO_LENGTH: u64 = 25; // seconds for testing, should be minutes
const SHORT_BREAK: u64 = 5;

// NOTE: need to include rounds as well
// TODO: cycle through some unicode chars while timer is running
// paused is some kind of dot char instead
// TODO: dont wait until minute has passed to sleep, just wait to decrement timer until minute has passed
// try to hit 60 fps
// TODO: need a way to notify user of invalid input now that FromStr is gone for command

// TODO: status line printing state/timer name, remaining time, indicator animation, current round
// TODO: now that line re-writing is switch to blocking input when no timer is running, and spawn
// a new input thread when timer is running

fn main() {
    write(SCREEN_CLEAR);
    write(main_header("POMO"));
    write(sub_header("Standard Pomodoro Timer"));
    write(line("pomodoro : 25:00 minutes"));
    write(line("break    : 05:00 minutes"));
    write(sub_header("Controls"));
    write(line("s - start | q - quit"));
    write(line("p - pause | r - resume"));
    write(sub_header("Status"));
    write(line(" "));

    let mut rounds = 0;
    let mut indicator = RingBuffer::new(["◤", "◥", "◢", "◣"]);
    let mut timers = RingBuffer::new([
        Timer::new("pomodoro", POMODORO_LENGTH),
        Timer::new("break", SHORT_BREAK),
    ]);

    let input_channel = spawn_input_channel();
    let mut state = State::StoppedTimer;

    let mut current_timer = timers.next_mut();
    let mut status = String::new();

    loop {
        let start = Instant::now();
        status.clear();

        let command = match get_input(&input_channel) {
            Some(input) => Command::from(input.as_str()),
            None => Command::from(""),
        };

        state = state.next(command);
        match state {
            State::Quitting => {
                break;
            }
            State::StoppedTimer => {
                status.push_str(&current_timer.to_string());
                write(format_status("◈", &state, &status));
            }
            State::RunningTimer => {
                status.push_str(&format_duration(current_timer.remaining()));
                write(format_status(indicator.next(), &state, &status));

                if current_timer.remaining() == &Duration::ZERO {
                    print!("\x07");
                    current_timer.reset();
                    current_timer = timers.next_mut();
                    state = State::StoppedTimer;
                } else {
                    current_timer.advance();
                }
            }
            State::PausedTimer => {
                status.push_str(&format_duration(current_timer.remaining()));
                write(format_status("◈", &state, &status))
            }
        }
        sleep(SECOND.saturating_sub(start.elapsed()));
    }

    write(SCREEN_CLEAR);
}

fn main_header(title: &str) -> String {
    let red = fg_color(250, 100, 100);
    format!(
        "╭{}╮\n│{}{red}{title}{RESET}{}│\n├{}┤",
        BOX_CHARS[4].repeat(WIDTH),
        " ".repeat(24),
        " ".repeat(24),
        BOX_CHARS[4].repeat(WIDTH)
    )
}

fn sub_header(contents: &str) -> String {
    let char_budget = if contents.len() > WIDTH {
        0
    } else {
        WIDTH - contents.len()
    };

    let left_space = char_budget / 2;
    let right_space = (char_budget / 2) + (contents.len() % 2);

    format!(
        "\n│{}│\n│{}{contents}{}│\n│ {} │",
        " ".repeat(WIDTH),
        " ".repeat(left_space),
        " ".repeat(right_space),
        BOX_CHARS[4].repeat(WIDTH - 2)
    )
}

fn line(contents: &str) -> String {
    let char_budget = if contents.len() > WIDTH {
        0
    } else {
        WIDTH - contents.len() - 1
    };

    let space = char_budget + (contents.len() % 2);

    format!("\n│ {contents}{}│", " ".repeat(space),)
}

fn get_input(input_channel: &Receiver<String>) -> Option<String> {
    match input_channel.try_recv() {
        Ok(input) => {
            write(BACK_ONE_LINE);
            Some(input)
        }
        Err(TryRecvError::Empty) => None,
        Err(TryRecvError::Disconnected) => panic!("input channel disconnected, shutting down."),
    }
}

fn spawn_input_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer.retain(|c| c != '\n' && c != '\r');
        tx.send(buffer).unwrap();
    });

    rx
}

fn format_status(indicator: &str, state: &State, status: impl Into<String>) -> String {
    format!(
        "{LINE_CLEAR}\r{VERTICAL_BAR} {indicator} {} {} ",
        state,
        status.into()
    )
}

fn format_duration(duration: &Duration) -> String {
    let seconds = duration.as_secs() % MINUTE;
    let minutes = duration.as_secs() / MINUTE;
    let seconds_zero_char = if (seconds as f64) / 10.0 < 1.0 {
        "0"
    } else {
        ""
    };

    let zero_char = if minutes == 0 { "0" } else { "" };

    format!("{zero_char}{minutes}:{seconds_zero_char}{seconds}")
}
