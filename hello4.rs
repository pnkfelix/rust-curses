extern mod ncurses_core (vers = "5.7");

#[fixed_stack_segment]
fn main() {
    use ncurses_core::{initscr, raw, keypad, noecho, printw, getch, attroff, A_BOLD, stdscr, refresh, endwin, attron, KEY_F1};

    unsafe {
        // raw(), cbreak(): turn off terminal buffering.  raw() passes
        // ctrl chars (^Z, ^C) through; cbreak() leaves interpretation
        // to terminal driver.
        // echo(), noecho(): control echoing of chars to terminal.
        // keypad(): enable reading of fcn keys, arrow keys, etc.
        // halfdelay(): This is some mode where input functions will only
        // wait for a timeout and then return ERR

        initscr();                    /* Start curses mode      */
        raw();
        keypad(stdscr, true as ncurses_core::NCURSES_BOOL);
        noecho();

        do "Type any character to see it in bold\n".to_c_str().with_ref |m| {
            printw(m);
        }

        let ch = getch(); // Without raw the input would be buffered to line break

        if ch == KEY_F1 { // Without keypad we'd miss F1
            do "F1 Key pressed".to_c_str().with_ref |m| { printw(m); }
        } else {
            do "The pressed key is ".to_c_str().with_ref |m| { printw(m); }
            attron(A_BOLD);
            // FSK: actual invocation was printw("%c", ch);
            // FSK: I am now curious as to whether curses needs to
            // FSK: muck with the internals within format strings,
            // FSK: and thus my simplification of passing one (fmt!'ed) string
            // FSK: will be broken.
            let mychar = std::char::from_u32(ch as u32).unwrap().to_str().to_c_str();
            do mychar.with_ref |m| { printw(m); }
            attroff(A_BOLD);
        }
        refresh();                    // 
        getch();
        endwin();                     /* Terminate and cleanup  */
    }
}
