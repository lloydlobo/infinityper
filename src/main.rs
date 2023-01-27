#![doc(
    html_logo_url = "https://raw.githubusercontent.com/lloydlobo/infinityper/main/assets/logo_dark.png"
)]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown, clippy::if_not_else, clippy::non_ascii_literal)]

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

use std::{
    fmt, process,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use anyhow::Result;
use calm_io::{pipefail, stdout, stdoutln};
use colorful::{Color, Colorful};
use crossbeam::{
    channel::{bounded, tick, Receiver},
    select,
};
// use colorful::HSL;
use structopt::StructOpt;

// ------------------------------------------------------------

#[derive(Debug, StructOpt)]
#[structopt(name = "infinityper", about = "inifinityper simulates typed text in the terminal")]
struct Opt {
    /// Activate debug mode
    #[structopt(long, short)]
    debug: bool,

    // Default string to 'print' when to env args are provided when the binary is run.
    /// Content to type. Usage: -i 'hello world'
    #[structopt(
        short,
        long,
        // Source: Devotion - Robert Frost.
        default_value = r#"The heart can think of no devotion
Greater than being shore to the ocean–
Holding the curve of one position,
Counting an endless repetition."#
    )]
    input: String,

    /// No. of times to run iteration of typing
    #[structopt(short = "r", long = "runs", default_value = "18446744073709551615")]
    iteration_runs: u64,

    /// Repeat output without clearing the terminal at every new line
    #[structopt(short, long = "repeat")]
    repeat_output: bool,

    /// Set typing speed.
    #[structopt(short, long, default_value = "150")]
    speed: f64,

    // The number of occurrenes of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Activate colored simulated typing
    #[structopt(short = "c", long = "color")]
    with_color: bool,
}

// ------------------------------------------------------------

/// Loop indefinitely while printing like a user is typing in the terminal.
///
/// * `#[pipefail]` - A `#[calm_io::pipefail]` function attribute that fails the build when applied
///   to a non-function item, or a function which does not return `io::Result<_>`.
/// * `char_indices()` - Iterator over the [`char`]s of a string slice, and their positions.
/// * `len_utf8()` - Number of bytes this `char` would need if encoded in UTF-8.
/// * `stdoutln!()` - Like `println!`, except it returns a `Result` rather than `panic!`king.
#[pipefail]
fn main() -> std::io::Result<String> {
    let ticks = tick(Duration::from_secs(1));
    let cli = Opt::from_args();
    if !cli.repeat_output {
        term_clear_screen_cursor_to_origin();
    }
    let input: String = cli.input.clone();
    let args: Vec<String> = input.lines().map(String::from).collect();

    let colors = get_color_variants();

    let mut counter_color = 0;
    let mut counter_iterations = 0;
    let mut counter_line_break = 0;

    // let running = Arc::new(AtomicUsize::new(0));
    // let r = running.clone();
    // ctrlc::set_handler(move || {
    //     let prev = r.fetch_add(1, Ordering::SeqCst);
    //     if prev == 0 { log::info!("Exiting..."); } else { process::exit(0); }
    // }).expect("Should set Ctrl-C handler");

    // let ctrl_c_events = ctrl_channel().unwrap();

    loop {
        if counter_iterations.cmp(&cli.iteration_runs).is_ge() {
            return Ok(input);
        } // can move this to while instead of loop?
          // select! {
          //     recv(ticks)-> _ => {
          //         println!("working!");
          //     }
          //     recv(ctrl_c_events)->_ => {
          //         println!();
          //         println!("Goodbye!");
          //         break Ok(input);
          //     }
          // }
        for arg in &args {
            for (i, char) in arg.char_indices() {
                // if running.load(Ordering::SeqCst) > 0 {
                //     break Ok(input);
                // }
                if !cli.repeat_output {
                    if i == 0 {
                        counter_line_break += 1;
                    }
                    term_move_cursor_to(counter_line_break, 1);
                }
                let i_len: usize = i + char.len_utf8();
                let string: &str = &arg[..i_len];
                if cli.with_color {
                    // Wrap around available colors back to 0 index.
                    let color = colors[counter_color % colors.len()];
                    stdoutln!("{}", string.gradient(color))?;
                } else {
                    stdoutln!("{}", string)?;
                    // select! {
                    //   recv(ticks)-> _ => {
                    //       print!(" <");
                    //   }
                    // }
                }
                sleep(cli.speed as u64); //150ms to simulate human typing
            }
            counter_color += 1; // each content newline sentence gets next color
        }
        if !cli.repeat_output {
            counter_line_break = 0; // reset line break counter after each iteration.
            term_clear_screen_cursor_to_origin();
        }
        counter_iterations += 1;
    }
}

// ------------------------------------------------------------

/// [Reference](https://rust-cli.github.io/book/in-depth/signals.html#using-channels)
fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;
    Ok(receiver)
}

// ------------------------------------------------------------

/// Clear the screen and put the cursor at first row & first col of the screen.
///
/// * `\x1b[2J` - clears the screen.
/// * `\x1b[1;1H` - sets the cursor position to `(1;1)`.
///
/// [Source](https://stackoverflow.com/a/62101709)
fn term_clear_screen_cursor_to_origin() {
    print!("\x1B[2J\x1B[1;1H");
}
/// Clear the screen.
/// ANSII escape chars - `\x1b[2J`
#[allow(dead_code)]
fn term_clear_screen() {
    print!("\x1B[2J");
}
/// Move the cursor to the first row & first col of the screen.
/// ANSII escape chars - `\x1B[1;1H`
fn term_move_cursor_to(x: usize, y: usize) {
    print!("\x1B[{x};{y}H");
}

// ------------------------------------------------------------

/// Puts the current thread to sleep for `millis` amount of time in milliseconds.
fn sleep(millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis))
}

// ------------------------------------------------------------

fn get_color_variants() -> Vec<Color> {
    vec![
        Color::Purple4b,
        Color::Cyan,
        Color::DodgerBlue1,
        Color::Green3b,
        Color::Yellow,
        Color::OrangeRed1,
        Color::PaleVioletRed1,
    ]
}

// ------------------------------------------------------------

/// Represents a key.
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Key {
    /// Enter or Return key
    Enter,
    Tab,
    Backspace,
    Esc,

    Char(char),
    Ctrl(char),
    Unknown,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Key::Char(' ') => write!(f, "<Space>"),
            Key::Ctrl(' ') => write!(f, "<Ctrl+Space>"),
            Key::Char(c) => write!(f, "{}", c),
            Key::Ctrl(c) => write!(f, "<Ctrl+{}>", c),
            Key::Enter | Key::Tab | Key::Backspace | Key::Esc => write!(f, "<{:?}>", self),
            _ => write!(f, "{:?}", self), // Keys::Unknown => todo!(),
        }
    }
}

// ------------------------------------------------------------

/// .context("Should wrap around colors infinitely for consecutive indexes")
// fn next_color_wrap(i: usize, colors: Vec<Color>) -> anyhow::Result<Color> {
//     let len = colors.len();
//     Ok(colors[i % len])
// }

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use quickcheck::{quickcheck, TestResult};

    use super::*;

    // ------------------------------------------------------------

    fn reverse<T>(xs: &[T]) -> Vec<T>
    where
        T: Clone,
    {
        let mut rev = vec![];
        for x in xs.iter() {
            rev.insert(0, x.clone())
        }
        rev
    }

    // fn wrap_nums(nums: Vec<u32>) -> u32 { nums.iter().next().cloned().unwrap() }

    // ------------------------------------------------------------

    quickcheck! {
        fn prop_rev(xs: Vec<u32>) -> bool {
            xs == reverse(&reverse(&xs))
        }
    }

    #[test]
    fn it_sleeps() {
        for i in 0..1000 {
            let start = Instant::now();
            sleep(i);
            let elapsed = start.duration_since(start);
            assert_eq!(Duration::from_millis(i), elapsed);
        }
    }

    #[test]
    fn it_reverse_single() {
        fn prop(xs: Vec<isize>) -> TestResult {
            if xs.len() != 1 {
                return TestResult::discard();
            }
            TestResult::from_bool(xs == reverse(&*xs))
        }
        quickcheck(prop as fn(Vec<isize>) -> TestResult);
    }
}
