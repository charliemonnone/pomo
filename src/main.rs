use std::thread::sleep;
use std::time::{Duration, Instant};

mod command;
mod io_utils;
mod state;
mod term_utils;
mod timer;

use crate::command::Command;
use crate::io_utils::*;
use crate::state::State;
use crate::term_utils::*;
use crate::timer::{format_duration, Timer, MINUTE, SECOND};

const POMODORO_LENGTH: u64 = 25 * MINUTE;
const SHORT_BREAK: u64 = 5 * MINUTE;

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

    let mut timers = [
        Timer::new("pomodoro", POMODORO_LENGTH),
        Timer::new("break", SHORT_BREAK),
    ];

    let input_channel = spawn_input_channel();

    let mut current_timer = 0;
    let mut rounds = 0;
    let mut state = State::StoppedTimer;

    loop {
        let start = Instant::now();

        let command = match get_input(&input_channel) {
            Some(input) => Command::from(input.as_str()),
            None => Command::from(""),
        };

        state = state.next(command);

        match state {
            State::Quitting => {
                break;
            }
            State::StoppedTimer => {}
            State::PausedTimer => {}
            State::RunningTimer => {
                if timers[current_timer].remaining() == &Duration::ZERO {
                    alert();
                    timers[current_timer].reset();
                    if timers[current_timer].name() == "break" {
                        rounds += 1;
                    }
                    current_timer = (current_timer + 1) % timers.len(); // wrap timer index  
                    state = State::StoppedTimer;
                } else {
                    timers[current_timer].advance();
                }
            }
        }

        write(LINE_CLEAR);
        write(format_status(&state, &timers[current_timer], rounds));
        write(bottom_line());
        sleep(SECOND.saturating_sub(start.elapsed()));
    }

    write(SCREEN_CLEAR);
}

fn main_header(title: &str) -> String {
    let red = fg_color(250, 100, 100);
    format!(
        "╭{}╮\n│{}{red}{title}{RESET}{}│\n├{}┤",
        "─".repeat(WIDTH),
        " ".repeat(24),
        " ".repeat(24),
        "─".repeat(WIDTH)
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
        "─".repeat(WIDTH - 2)
    )
}

fn line(contents: &str) -> String {
    let mut char_budget = if contents.len() > WIDTH {
        0
    } else {
        WIDTH - contents.len()
    };

    char_budget += contents.len() % 2; // account for even/odd len contents
    char_budget -= 1; // for the space

    format!("\n│ {contents}{}│", " ".repeat(char_budget))
}

fn bottom_line() -> String {
    format!("\n\r╰{}╯ \x1b[1A \x1b[9D", "─".repeat(WIDTH))
}

fn format_status(state: &State, timer: &Timer, rounds: u32) -> String {
    let rounds = rounds.to_string();
    let state = state.to_string();
    let timer_name = timer.name();
    let duration = format_duration(timer.remaining());

    let char_budget = WIDTH - rounds.len() - state.len() - timer_name.len() - duration.len() - 25; // number of static chars below between outer │

    format!(
        "{LINE_CLEAR}\r│ Round: {} │ {} │ {} {} │ input: {}│",
        rounds,
        state,
        timer_name,
        duration,
        " ".repeat(char_budget)
    )
}
