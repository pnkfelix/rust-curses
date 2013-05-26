extern mod ncurses;

fn main() {
    use ncurses::*;

    unsafe {
        // raw(), cbreak(): turn off terminal buffering.  raw() passes
        // ctrl chars (^Z, ^C) through; cbreak() leaves interpretation
        // to terminal driver.
        // echo(), noecho(): control echoing of chars to terminal.
        // keypad(): enable reading of fcn keys, arrow keys, etc.
        // halfdelay(): This is some mode where input functions will only
        // wait for a timeout and then return ERR

        let ch:libc::c_int;
        initscr();                    /* Start curses mode      */
        raw();
        keypad(stdscr, true);
        noecho();

        do str::as_c_str("Type any character to see it in bold\n") |m| {
            printw(m);
        }

        ch = getch(); // Without raw the input would be buffered to line break

        if ch == KEY_F1 { // Without keypad we'd miss F1
            do str::as_c_str("F1 Key pressed") |m| { printw(m); }
        } else {
            do str::as_c_str("The pressed key is") |m| { printw(m); }
            attron(A_BOLD());
            // FSK: actual invocation was printw("%c", ch);
            // FSK: I am now curious as to whether curses needs to
            // FSK: muck with the internals within format strings,
            // FSK: and thus my simplification of passing one (fmt!'ed) string
            // FSK: will be broken.
            do str::as_c_str(fmt!("%c", ch as char)) |m| { printw(m); }
            attroff(A_BOLD());
        }
        refresh(); // 
        getch();
        endwin();                     /* Terminate and cleanup  */
    }
}
