// Adapted from a read-through of:
//   http://invisible-island.net/ncurses/ncurses-intro.html

#![allow(unused_imports)]

extern crate ncurses; // = "ncurses_core#5.7";
extern crate libc;
extern crate debug;

use sig = signal_h;
use std::os;

mod signal_h;

mod locale;

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

pub fn main() {
    use ncurses::chars;
    #[allow(unused_imports)] // Issue #10534
    // use ncurses::chars::{Immed, Return}
    use ncurses::input::{Delay, Fail, Retry};
    use ncurses::colors;
    // use ncurses::chars::{getch};
    use ncurses::input;
    use ncurses::input::GetCh;
    // use ncurses::input::GetStr;
    use ncurses::output::AddCh;
    use ncurses::HasYX;
    use ncurses::attrs;
    use ncurses::attrs::AttrSet;
    use ncurses::background::Background;
    use ncurses::moves::Move;

    let mut num : colors::pair_num = 0;

    locale::setlocale(locale::all, "en_US.utf-8");
    let mut context = ncurses::Context::new();

    unsafe {
        finished = false;
        sig::signal(sig::INT, finish);

        // Danger, danger: privacy bug alert!
        // let mut scr : ncurses::screens::Screen = context.stdscr();
        let mut scr = context.stdscr();

        // N.B. on the below: "Since the screen package needs to know
        // what is on the terminal at all times, if characters are to
        // be echoed, the tty must be in raw or cbreak mode. Since
        // initially the terminal has echoing enabled and is in
        // ordinary ``cooked'' mode, one or the other has to changed
        // before calling getch(); otherwise, the program's output
        // will be unpredictable."

        context.set_nl(false);
        context.set_cbreak(true);
        context.set_echo(true);

        // This turns on automatic conversion of character-sequences
        // for arrow + function keys into pseudo-character values
        // representing the corresponding arrow or function key.
        context.keypad(&mut scr, true);

        if context.has_colors() {
            context.start_color();

            // simple color assignment; color 0 cannot be redefined.
            context.init_pair(1, colors::Red,     colors::Black);
            context.init_pair(2, colors::Green,   colors::Black);
            context.init_pair(3, colors::Yellow,  colors::Black);
            context.init_pair(4, colors::Blue,    colors::Black);
            context.init_pair(5, colors::Cyan,    colors::Black);
            context.init_pair(6, colors::Magenta, colors::Black);
            context.init_pair(7, colors::White,   colors::Black);
        }

        // context.on_getch_err(Immed(Retry));
        context.on_getch_err(Delay(inspect_errno));
        fn inspect_errno() -> input::get_err_act<chars::raw_ch> {
            let transient = unsafe { !finished };
            match os::errno() as libc::c_int {
                libc::EINTR if transient => Retry,
                _ => Fail,
            }
        }

        let name = context.longname();
        context.addstr(name.as_slice());
        context.bkgd(chars::ascii_ch('_' as i8));
        context.bkgdset(chars::ascii_ch('*' as i8));

        while !finished {
            let (y,x) = context.getyx();
            let c = context.getch();
            let desc = match c {
                chars::ascii_ch(cc) => format!("ascii_ch('{}')", cc as u8 as char),
                chars::wide_ch(wc)  => format!("wide_ch('{}')", wc),
                _                   => format!("{:?}", c),
            };
            context.mvaddstr(0, 0, desc.as_slice());
            context.move(y, x);

            // process the command keystroke
            match c {
                chars::move_ch(chars::down)  if y < ncurses::lines()-1 => context.move(y+1, x),
                chars::move_ch(chars::up)    if y > 0                => context.move(y-1, x),
                chars::move_ch(chars::left)  if x > 0                => context.move(y, x-1),
                chars::move_ch(chars::right) if x < ncurses::cols()-1  => context.move(y, x+1),
                _ => {}
            }
            // let s = context.mvgetstr(num as i32, 0);
            // context.addch(c);

/*
            let mut b = [0i8, ..16];
            context.mvgetascii(num as i32, 0, b.mut_slice_from(0));
            // let c = chars::ascii_ch(b[0]);
            let cs : ~[chars::raw_ch] =
                b.iter().map(|&c| chars::ascii_ch(c)).collect();
            context.addchstr(cs);
*/
            context.flash();

            context.attrset(attrs::color_pair(num % 8));
            num = num + 1;

            context.refresh();
        }

        context.beep();
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
