Labyrinth
=========

[![Build Status](https://travis-ci.org/royaltm/rust-labyrinth.svg?branch=master)](https://travis-ci.org/royaltm/rust-labyrinth)

This library creates a true labyrinth using low-memory, cpu-intensive hunt'n'seek algorithm. 

Program
-------

### Build

```
cargo build --release
target/release/labyrinth [cols=10 [rows=cols [times=1 [concurrency=num_cpus]]]]

# or

cargo run --release -- [cols=10 [rows=cols [times=1 [concurrency=num_cpus]]]]
```

### Create

```
$ labyrinth

┏━━━━━━━┳━━━━━━━━━┳━┓
┃ ╺━┳━╸ ┃ ┏━━━━━┓ ╹ ┃
┣━━━┫ ╺━┻━┛ ┏━╸ ┃ ╺━┫
┃ ╻ ┗━━━━━┳━┛ ┏━┻━┓ ┃
┃ ┃ ┏━╸ ╻ ┃ ╻ ┃ ╻ ╹ ┃
┃ ┗━┫ ╺━┻━┛ ┗━┫ ┗━━━┫
┣━╸ ┣━╸ ┏━━━┓ ╹ ┏━━━┫
┃ ┏━┛ ┏━┻━┓ ┗━┳━┛ ╻ ┃
┃ ┗━┓ ┃ ╻ ┃ ╻ ╹ ┏━┫ ┃
┃ ╻ ┃ ╹ ┃ ┗━┻━╸ ┃ ╹ ┃
┗━┻━┻━━━┻━━━━━━━┻━━━┛

$ labyrinth 40 5

┏━━━━━┳━┳━━━━━━━━━┳━━━━━┳━━━┳━━━━━┳━━━━━┳━━━━━┳━┳━━━━━┳━━━━━━━━━━━━━━━┳━━━━━┳━━━┓
┃ ┏━╸ ┃ ┃ ╺━┓ ╺━┳━┛ ┏━┓ ╹ ╺━┫ ╺━┓ ╹ ┏━╸ ┃ ╺━┓ ┃ ┣━━━┓ ╹ ┏━━━━━━━┳━━━┓ ┗━━━┓ ╹ ╻ ┃
┃ ┣━━━┛ ┃ ╻ ┣━╸ ┃ ╺━┫ ┗━━━┓ ╹ ┏━┻━┓ ┣━━━┻━┓ ┃ ╹ ┃ ╻ ┗━┳━┫ ┏━━━╸ ┃ ╺━┻━━━┓ ┃ ╺━┫ ┃
┃ ╹ ╻ ╺━╋━┛ ┃ ╺━┫ ╻ ╹ ┏━┓ ┗━┳━┛ ╻ ╹ ┃ ╻ ╻ ┃ ┗━━━┫ ┗━┓ ╹ ┃ ┃ ╺━━━┻━━━┳━╸ ┃ ┣━╸ ┃ ┃
┣━━━┛ ╻ ╹ ╺━┻━┓ ╹ ┣━╸ ┃ ┗━╸ ┃ ╺━┻━━━┛ ┃ ┗━┛ ╺━┓ ╹ ╻ ┗━┓ ╹ ┃ ╺━━━┓ ╺━┛ ╻ ╹ ╹ ┏━┛ ┃
┗━━━━━┻━━━━━━━┻━━━┻━━━┻━━━━━┻━━━━━━━━━┻━━━━━━━┻━━━┻━━━┻━━━┻━━━━━┻━━━━━┻━━━━━┻━━━┛
```


Library
-------

Add to `Cargo.toml`:

```
[dependencies.labyrinth]
git = "https://github.com/royaltm/rust-labyrinth.git"
```

Put to `src/main.rs`:

```rust
extern crate labyrinth;

use labyrinth::{Wall, Direction};
fn main() {
    let mut wall = Wall::new(20, 20);
    wall.carve();
    wall.print();
    wall.open(0, 10, Direction::Up);
    assert_eq!(true, wall.is_open(0, 10, Direction::Up));
    wall.close(0, 10, Direction::Up);
    assert_eq!(false, wall.is_open(0, 10, Direction::Up));
}
```

Run `cargo run`.
