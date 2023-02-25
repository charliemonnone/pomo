use std::fmt::Display;

use crate::command::Command;

pub enum State {
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
            Self::PausedTimer => write!(f, "Paused "), // extra space is hack to ensure state format size is consistent
        }
    }
}

impl State {
    /// Derive next state from command.
    pub fn next(&self, command: Command) -> Self {
        match self {
            State::Quitting => State::Quitting,
            State::StoppedTimer => match command {
                Command::Start => State::RunningTimer,
                Command::Quit => State::Quitting,
                _ => State::StoppedTimer,
            },
            State::RunningTimer => match command {
                Command::Pause => State::PausedTimer,
                Command::Quit => State::Quitting,
                _ => State::RunningTimer,
            },
            State::PausedTimer => match command {
                Command::Resume => State::RunningTimer,
                Command::Quit => State::Quitting,
                _ => State::PausedTimer,
            },
        }
    }
}
