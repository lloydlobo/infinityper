//! `inifinityper` simulates typed text in the terminal.
//!
//! A standin for the `yes` command, made with `calm_io`, insprired by `ouai` and built in Rust.
//!
//! # Installation
//!
//! You must have Rust installed on your machine, then you can install ouai by doing:
//!
//! ```shell
//! cargo install infinityper
//! ```
//!
//! # Usage
//!
//! Infinitely generate given strings in a typed form with `infinityper`:
//!
//! ## Generate infinitely
//!
//! ```shell
//! inifinityper Hello, world!
//! ```
//!
//! ## Generate once
//!
//! ```shell
//! inifinityper Hello, world! | head
//! ```
//!
//! # Reference
//!
//! [calm_io](https://github.com/myrrlyn/calm_io/blob/main/examples/good_yes.rs)
//!
//! Reimplementation of `yes(1)`, that does not die from SIGPIPE`.
//! A common idiom in UNIX computing is to prepend `yes |` to a pipeline in order to
//! get interactive scripts to act without user input. The coreutils implementation
//! of `yes(1)` crashes from SIGPIPE when the pipeline ends.
//! This program does not.

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown, clippy::if_not_else, clippy::non_ascii_literal)]

use std::{
    env::{self, Args},
    iter::Skip,
};

use calm_io::{pipefail, stdoutln};

/// Default string to 'print' when to env args are provided when the binary is run.
const DEFAULT_STDOUT_STRING: &str = "infinityper says hello!";

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

    loop {
        for arg in &args {
            for (index, char) in arg.char_indices() {
                sleep(150);

                term_clear_screen();
                term_move_cursor_origin();

                let index: usize = index + char.len_utf8();
                let string: &str = &arg[..index];
                stdoutln!("{}", string)?;
            }
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
