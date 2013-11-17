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
//
// Howver, there is also the pesky initialization (initscr) and
// destruction (endwin) functions.  Those lead me to think that I
// actually want an RAII style system here, potentially with either a
// global lock or just outright failing if it is initialized more than
// once in a process.

fn main() {
    use ncurses::{keypad, stdscr};
    use ncurses::mode::{cbreak,echo,nonl};
    use ncurses::chars;
    #[allow(unused_imports)]
    use ncurses::chars::{Immed, Delay, Fail, Retry, Return};
    use ncurses::colors::{has_colors, start_color, init_pair};
    use ncurses::colors;
    // use ncurses::chars::{getch};
    use ncurses::attrs;

    let mut num : colors::pair_num = 0;

    let mut context = ncurses::Context::new();

    unsafe {
        finished = false;
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

        // context.on_getch_err(Immed(Retry));
        context.on_getch_err(Delay(inspect_errno));
        fn inspect_errno() -> chars::getch_err_act {
            match os::errno() as libc::c_int {
                libc::EINTR => Retry,
                _ => Fail,
            }
        }
        while !finished {
            let c = context.getch();
            attrs::set([attrs::color_pair(num % 8)]);
            num = num + 1;

            // process the command keystroke
        }

        os::set_exit_status(0);
    }
}

static mut finished: bool = false;

extern "C" fn finish(_sig:libc::c_int)
{
    unsafe {
        finished = true;
    }
}
