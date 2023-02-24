#[allow(dead_code)]
use std::time::{Duration, Instant};
use std::io::{Write, BufWriter};
use std::thread::sleep;
const MINUTE: u64 = 60;
const ONE_SECOND: Duration = Duration::new(1, 0);

#[derive(Debug)]
pub struct Timer {
    name: String,
    duration: Duration,
    short_break: Duration,
    long_break: Duration,
    remaining: Duration
}

impl Default for Timer {
    fn default() -> Self {
        let name = "Default";
        let duration = Duration::from_secs(MINUTE * 25);
        let short_break = Duration::from_secs(MINUTE * 5);
        let long_break = Duration::from_secs(MINUTE * 15);

        Timer::new(name, duration, short_break, long_break)
    }
}

impl Timer {
    pub fn new<S>(name: S, duration: Duration, short_break: Duration, long_break: Duration) -> Self
    where
        S: Into<String>,
    {
        Timer {
            name: name.into(),
            duration,
            short_break,
            long_break,
            remaining: duration.clone()
        }
    }

    fn remaining(&self) -> u64 {
        self.remaining.as_secs()
    }

    fn advance(&mut self) {
        self.remaining = self.remaining.saturating_sub(ONE_SECOND);
    }
}
/*
    grid animation where each cell starts with a dot and then theres a wave going from top left to bottom 
    right where each dot is replaced with / or \
    like this https://github.com/patriciogonzalezvivo/glslViewer/blob/main/.github/images/03.gif
*/





// fn main() {
//     let mut timer = Timer::new("test", Duration::from_secs(5), Duration::from_secs(5), Duration::from_secs(10));
//     println!("--- {} ---", timer.name);
//     println!(">   begin pomodoro: {}", timer.remaining.as_secs());
    
//     while timer.remaining() > 0 {
//         print!("\r    pomo time remaining: {}s", timer.remaining());
//         std::io::stdout().flush();
//         timer.advance();
//         sleep(ONE_SECOND);
//     }
//     println!();
//     println!(">   begin short break: {}", timer.short_break.as_secs());
//     timer.remaining = timer.short_break.clone();

//     while timer.remaining() > 0 {
//         print!("\r    break time remaining: {}s", timer.remaining());
//         std::io::stdout().flush();
//         timer.advance();
//         sleep(ONE_SECOND);
//     }

//     println!();
//     println!(">   done")

// }

/*
    usage flow
    $ pomo
        print pomo details e.g. pomo duration short/long break
        pompt to start pomo
        run 
        prompt to start short or long break
        run
        prompt to rerun or exit
*/

fn start_timer(length: u64) {
    let mut remaining = Duration::new(length, 0);
    let one_second = Duration::new(1, 0);
    let line_clear = "\x1b[2K";
    while remaining.as_secs() >= 0 {
        std::io::stdout().flush();
        print!("\r{line_clear}");
        print!("time remaining: {}\r", remaining.as_secs());
        remaining = remaining.saturating_sub(one_second);
        sleep(one_second);
    }
    std::io::stdout().flush();
    println!();
}

const POMODORO_LENGTH: u64 = 25; // seconds for testing, should be minutes
const SHORT_BREAK: u64 = 5;
const LONG_BREAK: u8 = 10;

fn main() {
    println!("pomodoro    | {POMODORO_LENGTH}:00");
    println!("short break |  {SHORT_BREAK}:00");
    println!("long break  | {LONG_BREAK}:00");

    println!("start?");
    start_timer(POMODORO_LENGTH);

    println!("short or long break?");
    start_timer(SHORT_BREAK);

    println!("Restart or quit?");

}
