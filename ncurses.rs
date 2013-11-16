#[link(name="ncurses",vers="5.7")];

use ncurses_core::{WINDOW_p, SCREEN_p};

mod ncurses_core;

/// Describes a sub-rectangle of the screen (possibly in its
/// entirety), that you can write to and scroll independently of other
/// windows on the screen.
pub struct window { priv ptr: WINDOW_p }

/// These are windows as large as the terminal screen (upper-left to
/// lower-right).  `stdscr` is one such screen; it is the default for
/// output.  There is a special screen, the "terminal screen", that
/// corresponds to ncurses' model of what the user sees now.  (It
/// might also correspond to `curscr`.
pub struct screen { priv ptr: SCREEN_p }
