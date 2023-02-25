// #![allow(dead_code)]
use std::fmt::Display;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod command;
mod io_utils;
mod ring;
mod term_utils;
mod timer;

use crate::command::Command;
use crate::io_utils::write;
use crate::ring::RingBuffer;
use crate::term_utils::*;
use crate::timer::{Timer, SECOND};

use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::thread;

const POMODORO_LENGTH: u64 = 25; // seconds for testing, should be minutes
const SHORT_BREAK: u64 = 5;
const MINUTE: u64 = 60;

// NOTE: need to include rounds as well
// TODO: cycle through some unicode chars while timer is running
// paused is some kind of dot char instead
// TODO: dont wait until minute has passed to sleep, just wait to decrement timer until minute has passed
// try to hit 60 fps
// TODO: need a way to notify user of invalid input now that FromStr is gone for command

// TODO: status line printing state/timer name, remaining time, indicator animation, current round
// TODO: now that line re-writing is switch to blocking input when no timer is running, and spawn
// a new input thread when timer is running 

#[derive(Debug)]
enum State {
    Quitting,
    StoppedTimer,
    RunningTimer,
    PausedTimer,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Quitting => write!(f, "Quitting"),
            Self::StoppedTimer => write!(f, "Stopped"),
            Self::RunningTimer => write!(f, "Running"),
            Self::PausedTimer => write!(f, "Paused"),
        }

        
    }
}

fn main() {
    write(SCREEN_CLEAR);
    write(start_message());
    write(standard_timer());
    write(controls());

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
            Some(input) => {
                Command::from(input.as_str())
            },
            None => Command::from("")
        };

        state = get_next_state(state, command);
        match state {
            State::Quitting => {
                break;
            }
            State::StoppedTimer => {
                status.push_str(&format_timer(current_timer));
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

fn controls() -> String {
    format!(
        "
{VERTICAL_BAR} Controls
\u{251c}{}
{VERTICAL_BAR} s - Start | n - no | q - quit | p - pause | r - resume
{VERTICAL_BAR}
        ",
        divider()
    )
}

fn standard_timer() -> String {
    format!(
        "
{VERTICAL_BAR} Standard Pomodoro Timer
\u{251c}{}
{VERTICAL_BAR} pomodoro    : {POMODORO_LENGTH}:00 minutes
{VERTICAL_BAR} break :  {SHORT_BREAK}:00 minutes
{VERTICAL_BAR}
        ",
        divider()
    )
}

fn start_message() -> String {
    let red = fg_color(250, 100, 100);

    format!(
        "
{VERTICAL_BAR} {red}POMO{RESET}
\u{251c}{}
{VERTICAL_BAR} A tiny pomodoro timer.
{VERTICAL_BAR} Type <command> + Enter to control an in progress timer.
{VERTICAL_BAR}
        ", 
        divider()
    )
}

fn get_next_state(state: State, command: Command) -> State {
    match state {
        State::Quitting => {
            state // this state shouldn't technically be possible, but covering case anyway
        }
        State::StoppedTimer => match command {
            Command::Start => State::RunningTimer,
            Command::Quit => State::Quitting,
            _ => state,
        },
        State::RunningTimer => match command {
            Command::Pause => State::PausedTimer,
            Command::Quit => State::Quitting,
            _ => state,
        },
        State::PausedTimer => match command {
            Command::Resume => State::RunningTimer,
            Command::Quit => State::Quitting,
            _ => state,
        },
    }
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
        "{LINE_CLEAR}\r{VERTICAL_BAR} {indicator} {} {}", state, status.into()
    )
}

fn format_timer(timer: &Timer) -> String {
    let seconds = timer.remaining().as_secs() % MINUTE;
    let minutes = timer.remaining().as_secs() / MINUTE;
    let seconds_zero_char = if (seconds as f64) / 10.0 < 1.0 {
        "0"
    } else {
        ""
    };

    let zero_char = if minutes == 0 { "0" } else { "" };
    format!(
        "{} {zero_char}{minutes}:{seconds_zero_char}{seconds} ",
        timer.name()
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

// fn start_delta_timer(length: u64) {
//     let mut start = Instant::now();
//     let end = Duration::new(0,0);

//     let mut time_remaining = Duration::new(length, start.elapsed().as_nanos() as u32);

//     let debug_timer = Instant::now();

//     while time_remaining.as_secs() > end.as_secs() {
//         let elapsed = start.elapsed();
//         if elapsed > SECOND {

//             time_remaining = time_remaining.saturating_sub(SECOND);
//             print_time_remaining(time_remaining);
//             std::io::stdout().flush();
//             start = Instant::now();

//         }
//         else {
//             sleep(SECOND.saturating_sub(elapsed));
//         }
//     }

//     println!();
//     println!("{}", debug_timer.elapsed().as_millis());
// }
