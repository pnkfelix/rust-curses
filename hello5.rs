#[feature(globs)];

extern mod ncurses_core (vers = "5.7");

fn body() {
    use ncurses_core::*;
    use std::libc::{c_int,c_char};

    unsafe {
        let mesg = "Just a string";
        let mut row : c_int = 0;
        let mut col : c_int = 0;
        initscr();
        getmaxyx(stdscr, &mut row, &mut col);
        mesg.with_c_str(|m| { mvaddstr(row/2, (col-mesg.len() as c_int)/2, m); });
        let mesg = format!("This screen has {} rows and {} columns\n", row, col);
        mesg.with_c_str(|m| { mvaddstr(row-2, 0, m); });
        let mesg = "Try resizing your window (if possible) \
                    and then run this program again";
        mesg.with_c_str(|m:*c_char| { addstr(m); });
        refresh();
        getch();
        endwin();
    }
}

fn main() {
    body();
}
