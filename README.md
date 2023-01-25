# Infinityper

`inifinityper` simulates typed text in the terminal.

A standin for the `yes` command, made with [calm_io](https://github.com/myrrlyn/calm_io),
insprired by [ouai](https://github.com/Kerollmops/ouai) and built in Rust.

## Installation

You must have Rust installed on your machine, then you can install ouai by doing:

```shell
cargo install infinityper
```

## Usage

Infinitely generate given strings in a typed form with `infinityper`:

### Generate infinitely

```shell
inifinityper Hello, world!
```

### Generate once

```shell
inifinityper Hello, world! | head
```

## Background

[From calm_io](https://github.com/myrrlyn/calm_io/blob/main/examples/good_yes.rs):
> Reimplementation of `yes(1)`, that does not die from SIGPIPE`.
> A common idiom in UNIX computing is to prepend`yes |`to a pipeline in order to
> get interactive scripts to act without user input. The coreutils implementation
> of`yes(1)` crashes from SIGPIPE when the pipeline ends.
> This program does not.

## Dependencies

- [calm_io](https://crates.io/calm_io)