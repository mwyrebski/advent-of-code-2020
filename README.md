# Advent of Code 2020

Here are my solutions to the Advent of Code 2020.
I wanted to learn a little bit more about Rust, so this year's edition of AoC was dedicated to [Rust](https://www.rust-lang.org/).

This project doesn't rely on any external crate. Using externals like [regex](https://docs.rs/regex/) would make things a little easier and maybe more readable but most often the inputs could be parsed by simple split functions.

Things I'm proud of:

- solving the problems by myself
- learning rust syntax and its standard library
- not using external crates
- making it run in a reasonable time (except solution for the day 19)
- solving all of the puzzles (even though not on time)

I kept a separate journal, more or less detailed, mostly about Rust things I learnt. It is attached in [NOTES.md](./NOTES.md).

## Rust

I solved all these puzzles on Windows box with Rust 1.48.0:

    $ rustc --version
    rustc 1.48.0 (7eac88abb 2020-11-16)
    $ cargo --version
    cargo 1.48.0 (65cbdd2dc 2020-10-14)

## Running

Execute this program with cargo:

    $ cargo run

Most of the solutions contains some tests. They can be executed with:

    $ cargo test

# License

[MIT](https://opensource.org/licenses/MIT)
