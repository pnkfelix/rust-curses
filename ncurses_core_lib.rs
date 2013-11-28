#[feature(macro_rules)];
#[feature(globs)];

#[link(name="ncurses_core",vers="5.7")];

pub use ncurses_core::*;
mod ncurses_core;
