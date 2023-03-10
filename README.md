# Infinityper

![infinityper logo](https://github.com/lloydlobo/infinityper/blob/master/assets/logo.png)

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

## Demo with Docker

- TODO: Add exit command to stop running `docker` container.
  - Note: Press `Ctrl`+`c` to exit.
- TODO: Add `infinityper` arguments.

To start a `docker` container with the application, run:

```shell
docker run -it --rm --name infinityper lloydlobo/infinityper:latest
```

To open an interactive `docker` container with the application binary in it, run:

```shell
docker run -it --rm --name infinityper --entrypoint bin/bash lloydlobo/infinityper:latest
$ ./infinityper <infinityper arguments here>
```

To stop the container from another shell instance, run:

```shell
docker stop $(docker ps | grep infinityper | head -c12)
```

## Build

### To build your own image

Download the repo:

```shell
git clone <https://github.com/lloydlobo/infinityper.git>
```

Ensure you navigate to the download location of the repo:

```shell
cd /path/to/download/infinityper
```

Build away!

- With `Cargo`:

```shell
cargo build --release
```

- With `docker`:

```shell
docker build -t <yourimagename> .
```

## Dependencies

- [calm_io](https://crates.io/calm_io)

## Background

[From calm_io](https://github.com/myrrlyn/calm_io/blob/main/examples/good_yes.rs):
Reimplementation of `yes(1)`, that does not die from `SIGPIPE`.
A common idiom in UNIX computing is to prepend `yes |` to a pipeline in order to
get interactive scripts to act without user input. The coreutils implementation
of`yes(1)` crashes from SIGPIPE when the pipeline ends.
This program does not.
