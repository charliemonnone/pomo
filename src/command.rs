#[derive(Debug)]
pub enum Command {
    Start,
    No,
    Quit,
    Pause,
    Resume,
    Invalid,
    Pass,
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s {
            "S" | "s" => Command::Start,
            "N" | "n" => Command::No,
            "Q" | "q" => Command::Quit,
            "P" | "p" => Command::Pause,
            "R" | "r" => Command::Resume,
            "" => Command::Pass,
            _ => Command::Invalid,
        }
    }
}
