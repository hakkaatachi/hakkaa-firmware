//! Board support for the Hakkaa soldering and programming excersise board.
//!
//! There is some initializaton magic going on behing the scenes which you can call into action
//! with a simple wink from your magic wand:
//!
//! ```rust
//! let board = Board::init();
//! ```
//!
//! Congratulations! Now have a look at [`board::Board`] to see what you just got and where to go
//! on from here. Have fun!

#![no_std]

pub mod board;
pub mod led;
pub mod switch;
