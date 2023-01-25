#![doc(
    html_logo_url = "https://raw.githubusercontent.com/lloydlobo/infinityper/main/assets/logo_dark.png"
)]
// #![doc = include_str!("../docs/README.md")]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown, clippy::if_not_else, clippy::non_ascii_literal)]

// extern crate colorful;

use std::{
    env::{self, Args},
    iter::Skip,
};

use calm_io::{pipefail, stdoutln};
use colorful::{Color, Colorful};
// use colorful::HSL;

/// Default string to 'print' when to env args are provided when the binary is run.
const DEFAULT_STDOUT_STRING: &str = "infinityper says hello...";

/// Loop indefinitely while printing like a user is typing in the terminal.
///
/// * `#[pipefail]` - A `#[calm_io::pipefail]` function attribute that fails the build when applied
///   to a non-function item, or a function which does not return `io::Result<_>`.
/// * `char_indices()` - Iterator over the [`char`]s of a string slice, and their positions.
/// * `len_utf8()` - Number of bytes this `char` would need if encoded in UTF-8.
/// * `stdoutln!()` - Like `println!`, except it returns a `Result` rather than `panic!`king.
#[pipefail]
fn main() -> std::io::Result<()> {
    let args: Skip<Args> = env::args().skip(1);
    let args: Vec<String> = match args.len() {
        0 => vec![String::from(DEFAULT_STDOUT_STRING)],
        _ => args.collect(),
    };
    // This allows to print all args in one single line.
    let args: Vec<String> = vec![args.join(" ").trim().to_string()];

    let colors = vec![
        Color::Purple4b,
        Color::Cyan,
        Color::DodgerBlue1,
        Color::Green3b,
        Color::Yellow,
        Color::OrangeRed1,
        Color::PaleVioletRed1,
    ];
    let mut counter_color = 0;
    loop {
        for arg in &args {
            for (index, char) in arg.char_indices() {
                let color = colors[counter_color % colors.len()];
                sleep(150);

                term_clear_screen();
                term_move_cursor_origin();

                let index: usize = index + char.len_utf8();
                let string: &str = &arg[..index];
                stdoutln!("{}", string.gradient(color))?;
            }
            counter_color += 1;
        }
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
fn term_move_cursor_origin() {
    print!("\x1B[1;1H");
}

/// Puts the current thread to sleep for `millis` amount of time in milliseconds.
fn sleep(millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis))
}
