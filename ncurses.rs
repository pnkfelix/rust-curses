#[link(name="ncurses",vers="5.7")];

#[feature(macro_rules)];

use nc = ncurses_core;
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

impl screen {
    unsafe fn wnd_ptr(&self) -> WINDOW_p {
        self.ptr as WINDOW_p
    }
}

macro_rules! fail_if_err {
    ($e:expr) => {
        if $e == nc::ERR { fail!(); }
    }
}

macro_rules! wrap {
    ($i:ident) => {
        #[fixed_stack_segment]
        pub fn $i() {
            unsafe { fail_if_err!(nc::$i()) }
        }
    }
}

pub mod mode {
    use nc = ncurses_core;

    wrap!(cbreak)
    wrap!(echo)
    wrap!(nonl)
}

#[fixed_stack_segment]
pub fn endwin() {
    unsafe { fail_if_err!(nc::endwin()) }
}

pub mod colors {
    use nc = ncurses_core;

    pub enum color_t {
        Black = nc::COLOR_BLACK,
        Red = nc::COLOR_RED,
        Green = nc::COLOR_GREEN,
        Yellow = nc::COLOR_YELLOW,
        Blue = nc::COLOR_BLUE,
        Magenta = nc::COLOR_MAGENTA,
        Cyan = nc::COLOR_CYAN,
        White = nc::COLOR_WHITE,
    }

    #[fixed_stack_segment]
    pub fn has_colors() -> bool {
        unsafe { nc::has_colors() != 0 }
    }

    #[fixed_stack_segment]
    pub fn init_pair(pair: i16, fg: color_t, bg: color_t) {
        assert!(pair < color_pair_count());
        unsafe { fail_if_err!(nc::init_pair(pair, fg as i16, bg as i16)); }
    }

    pub fn color_pair_count() -> i16 {
        nc::COLOR_PAIRS as i16
    }

    #[fixed_stack_segment]
    pub fn start() { start_color(); }
    wrap!(start_color)
}


#[fixed_stack_segment]
pub fn keypad(s:&mut screen, enabled: bool) {
    unsafe {
        fail_if_err!(nc::keypad(s.wnd_ptr(), enabled as nc::NCURSES_BOOL))
    }
}

pub fn stdscr() -> screen {
    screen { ptr: nc::stdscr as SCREEN_p}
}
