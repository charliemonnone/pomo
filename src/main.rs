use std::fmt::Display;
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
use crate::io_utils::*;
use crate::ring::RingBuffer;
use crate::state::State;
use crate::term_utils::*;
use crate::timer::{Timer, MINUTE, SECOND};

const POMODORO_LENGTH: u64 = 25; // seconds for testing, should be minutes
const SHORT_BREAK: u64 = 5;

// NOTE: need to include rounds as well
// TODO: dont wait until minute has passed to sleep, just wait to decrement timer until minute has passed
// try to hit 60 fps
// TODO: need a way to notify user of invalid input now that FromStr is gone for command

// TODO: status line printing state/timer name, remaining time, indicator animation, current round
// TODO: now that line re-writing is switch to blocking input when no timer is running, and spawn
// a new input thread when timer is running


struct Status<'a> {
    indicator: &'a str,
    state: State,
    timer: &'a mut Timer, // formatting state should also include the duration in the status line
    rounds: u32
}

impl Display for Status<'_> {


    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let indicator = self.indicator;
        let rounds = self.rounds.to_string();
        let state = self.state.to_string();
        let timer_name = self.timer.name();
        let duration = format_duration(self.timer.remaining());

        // 10 is the number of static chars below between the │ chars
        let char_budget = WIDTH - indicator.len() - rounds.len() - state.len() - timer_name.len() - duration.len() - 14;  
        let back_chars = "\x1b[18E";
        write!(
            f, 
            "{LINE_CLEAR}\r│ {} Round: {} | {} {} {} |{}│\x1b[{char_budget}D", 
            indicator, rounds, state, timer_name, duration, " ".repeat(char_budget)
            
        )
    }
}

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

    let mut indicator = RingBuffer::new(["◤", "◥", "◢", "◣"]);
    let mut timers = RingBuffer::new([
        Timer::new("pomodoro", POMODORO_LENGTH),
        Timer::new("break", SHORT_BREAK),
    ]);

    let input_channel = spawn_input_channel();
    let mut state = State::StoppedTimer;

    // let mut current_timer = timers.next_mut();
    // let mut status = String::new();

    let mut status = Status {
        indicator: "◈",
        state: State::StoppedTimer,
        timer: timers.next_mut(),
        rounds: 0
    };

    loop {
        let start = Instant::now();

        let command = match get_input(&input_channel) {
            Some(input) => Command::from(input.as_str()),
            None => Command::from(""),
        };

        status.state = status.state.next(command);

        match status.state {
            State::Quitting => {
                break;
            },
            State::StoppedTimer | State::PausedTimer => {
                status.indicator = "◈";
                // status.push_str(&current_timer.to_string());
                // write(format_status("◈", &state, &status));
            },
            State::RunningTimer => {
                if status.timer.remaining() == &Duration::ZERO {
                    print!("\x07");
                    status.indicator = "◈";
                    status.timer.reset();
                    // status.timer = 
                    status.state = State::StoppedTimer;
                } else {
                    status.indicator = indicator.next();
                    status.timer.advance();
                }
            },
        }
        write(status.to_string());
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
