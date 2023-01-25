#![doc(
    html_logo_url = "https://raw.githubusercontent.com/lloydlobo/infinityper/main/assets/logo_dark.png"
)]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown, clippy::if_not_else, clippy::non_ascii_literal)]

use std::{
    env::{self, Args},
    iter::Skip,
    path::PathBuf,
};

use calm_io::{pipefail, stdoutln};
use colorful::{Color, Colorful}; // use colorful::HSL;
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

    /// No. of times to repeat typing
    #[structopt(long, short, default_value = "18446744073709551615")]
    iterations: u64,

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

    /// Clear terminal at every new line
    #[structopt(long = "clear")]
    clear_screen: bool,
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
fn main() -> std::io::Result<()> {
    let cli = Opt::from_args();
    let args: String = cli.input;
    // This allows to print all args in one single line.
    let args: Vec<String> = args.lines().map(String::from).collect();
    // let args: Vec<String> = vec![args.trim().to_string()];

    let colors = get_color_variants();

    let mut counter_color = 0;
    let mut counter_iter = 0;
    let mut counter_line_break = 0;

    if cli.clear_screen {
        term_clear_screen();
        term_move_cursor_to(1, 1); // term_move_cursor_to(i, i_len); // Bizzare
    }
    loop {
        for arg in &args {
            for (i, char) in arg.char_indices() {
                sleep(cli.speed as u64); // 150ms
                if cli.clear_screen {
                    if i == 0 {
                        counter_line_break += 1;
                    }
                    term_move_cursor_to(counter_line_break, 1);
                }
                let color = colors[counter_color % colors.len()];
                let i_len: usize = i + char.len_utf8();
                let string: &str = &arg[..i_len];
                if cli.with_color {
                    stdoutln!("{}", string.gradient(color))?;
                } else {
                    stdoutln!("{}", string)?;
                }
            }
            if counter_iter >= cli.iterations {
                return Ok(());
            }
            counter_color += 1;
        }
        if cli.clear_screen {
            counter_line_break = 0; // reset line break counter after each iteration.
            term_clear_screen();
            term_move_cursor_to(1, 1);
        }
        counter_iter += 1;
    }
}

/// Clear the screen and put the cursor at first row & first col of the screen.
///
/// * `\x1b[2J` - clears the screen.
/// * `\x1b[1;1H` - sets the cursor position to `(1;1)`.
///
/// [Source](https://stackoverflow.com/a/62101709)
#[allow(dead_code)]
fn term_clear_screen_cursor_origin() {
    print!("\x1B[2J\x1B[1;1H");
}

fn term_clear_screen() {
    print!("\x1B[2J");
}
/// Basic - print!("\x1B[1;1H");
fn term_move_cursor_to(x: usize, y: usize) {
    print!("\x1B[{x};{y}H");
}

/// Puts the current thread to sleep for `millis` amount of time in milliseconds.
fn sleep(millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis))
}

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
