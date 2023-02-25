use std::{
    fmt::Display,
    io::{self, Write},
};

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
