#![feature(macro_rules)]
#![feature(globs)]
#![feature(link_args)]
#![crate_name="ncurses_core"]

extern crate libc;

pub use ncurses_core::*;
mod ncurses_core;
