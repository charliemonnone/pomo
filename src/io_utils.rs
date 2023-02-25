use std::{
    io::{self, Write},
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
};

use crate::term_utils::BACK_ONE_LINE;

/// Write to stdout and flush it.  
pub fn write(msg: impl Into<String>) {
    print!("{}", msg.into());
    match std::io::stdout().flush() {
        Ok(()) => {}
        Err(e) => {
            println!("{:?}", e)
        }
    }
}

/// Poll the input channel for user input.
pub fn get_input(input_channel: &Receiver<String>) -> Option<String> {
    match input_channel.try_recv() {
        Ok(input) => {
            write(BACK_ONE_LINE); // prevent the program from writing beyond the bottom box characters
            Some(input)
        }
        Err(TryRecvError::Empty) => None,
        Err(TryRecvError::Disconnected) => panic!("input channel disconnected, shutting down."),
    }
}

/// Create an input channel in a separate thread for non-blocking user input.
/// Strips the input of carriage return and newline characters.  
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
