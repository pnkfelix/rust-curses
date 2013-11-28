extern mod ncurses_core (vers = "5.7"); 

#[fixed_stack_segment]
fn body() {
    use ncurses_core::{initscr,printw,refresh,getch,endwin};

    unsafe {
        initscr();                    /* Start curses mode      */
        let msg = "Hello World !!!".to_c_str();
        do msg.with_ref |msg| { printw(msg); } /* Print msg (buffered)   */
        refresh();                    /* Print buffer to screen */
        getch();                      /* Wait for user input    */
        endwin();                     /* Terminate and cleanup  */
    }
}

fn main() {
    body();
}
