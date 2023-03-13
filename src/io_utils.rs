use std::{
    fmt::Display,
    io::{self, Write}, sync::mpsc::{Receiver, TryRecvError, self}, thread,
};

use crate::term_utils::BACK_ONE_LINE;

// pub fn write<'a, T>(msg: T)
// where
// 	T: Into<&'a str> + Display,
// {
// 	write!(io::stdout(), "{msg}").unwrap()
// }

// pub fn write<'a, T>(msg: T) -> io::Result<()>
// where
//  T: Into<&'a str> + Display,
// {
// 	write!(io::stdout(), "{msg}").unwrap()
// }

pub fn write(msg: impl Into<String>) {
    write!(io::stdout(), "{}", msg.into()).unwrap();
    match std::io::stdout().flush() {
        Ok(()) => {}
        Err(e) => {
            println!("{:?}", e)
        }
    }
}

pub fn get_input(input_channel: &Receiver<String>) -> Option<String> {
    match input_channel.try_recv() {
        Ok(input) => {
            write(BACK_ONE_LINE);
            Some(input)
        }
        Err(TryRecvError::Empty) => None,
        Err(TryRecvError::Disconnected) => panic!("input channel disconnected, shutting down."),
    }
}

pub fn spawn_input_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        buffer.retain(|c| c != '\n' && c != '\r');
        tx.send(buffer).unwrap();
    });

    rx
}
