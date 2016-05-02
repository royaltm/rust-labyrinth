//! # The labyrinth library
//!
//! The labyrinth library provides tools for building and managing labyrinths.
//!
//! # Example
//!
//! ```
//! extern crate labyrinth;
//! 
//! use labyrinth::{Wall, Direction};
//! fn main() {
//!     let mut wall = Wall::new(5, 5);
//!     wall.carve();
//!     wall.print();
//!     wall.open(0, 10, Direction::Up);
//!     assert_eq!(true, wall.is_open(0, 10, Direction::Up));
//!     wall.close(0, 10, Direction::Up);
//!     assert_eq!(false, wall.is_open(0, 10, Direction::Up));
//! }
//! ```

extern crate rand;

pub mod vector2d;
pub mod wall;
pub mod direction;
pub mod room;

pub type Int = i32;
pub type Uint = u32;

pub use wall::Wall;
pub use direction::Direction;
