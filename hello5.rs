extern mod ncurses;

fn main() {
    use ncurses::*;
    use std::libc::c_int;

    unsafe {
        let mesg = "Just a string";
        let mut row : c_int = 0, col : c_int = 0;
        initscr();
        getmaxyx(stdscr, &mut row, &mut col);
        do str::as_c_str(mesg) |m| { mvaddstr(row/2, (col-mesg.len() as c_int)/2, m); }
        do str::as_c_str(fmt!("This screen has %? rows and %? columns\n", row, col)) |m| { mvaddstr(row-2, 0, m); }
        do str::as_c_str("Try resizing your window (if possible) and then run this program again") |m| { addstr(m); };
        refresh();
        getch();
        endwin();
    }
}
