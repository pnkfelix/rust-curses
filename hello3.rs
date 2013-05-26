extern mod ncurses;

fn main() {
    use ncurses::{initscr,printw,refresh,getch,endwin};

    unsafe {
        initscr();                    /* Start curses mode      */
        do str::as_c_str("Hello World !!!")
            |msg| { printw(msg); }    /* Print msg (buffered)   */
        refresh();                    /* Print buffer to screen */
        getch();                      /* Wait for user input    */
        endwin();                     /* Terminate and cleanup  */
    }
}
