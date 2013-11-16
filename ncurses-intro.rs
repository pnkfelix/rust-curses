// Adapted from a read-through of:
//   http://invisible-island.net/ncurses/ncurses-intro.html

extern mod ncurses;

use sig = signal_h;
use std::libc;
use std::os;

mod signal_h;

// Some notes at the outset:
// (1.) In general:
//                 move(y, x);
//                 addch(ch);
//                                   ==>
//                                         mvaddch(y, x, ch);
//
//   and:
//                 wmove(win, y, x);
//                 waddch(win, ch);
//                                   ==>
//                                        mvwaddch(win, y, x, ch);
//
//   Though of course in Rust we might prefer methods for the latter, e.g.
//                 win.move(y, x);
//                 win.addch(ch);
//                                   ==>
//                                        win.mvaddch(y, x, ch);
//
//   as that preserves a perfect analogy with the first example.
//
// (2.) The curses library has some global variables of interest:
//      LINES (number of lines on the terminal), COLS (number of
//      columns on the terminal), as well as some types and constants
//      (bool, TRUE, FALSE, ERR, OK).
//
//      Those all sound adaptable to a Rusty way of doing things.


fn main() {
    use ncurses::{keypad, stdscr};
    use ncurses::mode::{cbreak,echo,nonl};
    use ncurses::colors::{has_colors, start_color, init_pair};
    use ncurses::colors;
    // use ncurses::{getch, attrset};

    let num : int;

    unsafe {
        sig::signal(sig::INT, finish);
        let mut scr = stdscr();
        keypad(&mut scr, true);
        nonl();
        cbreak();
        echo();

        if has_colors() {
            start_color();

            // simple color assignment; color 0 cannot be redefined.
            init_pair(1, colors::Red,     colors::Black);
            init_pair(2, colors::Green,   colors::Black);
            init_pair(3, colors::Yellow,  colors::Black);
            init_pair(4, colors::Blue,    colors::Black);
            init_pair(5, colors::Cyan,    colors::Black);
            init_pair(6, colors::Magenta, colors::Black);
            init_pair(7, colors::White,   colors::Black);
        }

/*
        loop {
            let c = getch();
            attrset(COLOR_PAIR(num % 8));
            num = num + 1;

            // process the command keystroke
        }
*/
    }
}

extern "C" fn finish(sig:libc::c_int)
{
    ncurses::endwin();

    os::set_exit_status(0);
}
