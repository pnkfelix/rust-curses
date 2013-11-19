#[feature(macro_rules)];

#[link(name="ncurses_core",vers="5.7")];
use std::libc::{c_char, c_int, c_short, c_uchar, c_uint, c_void};
use std::libc::{wchar_t, FILE, EOF};

pub type chtype = c_uint;
pub type attr_t = c_int;
pub type NCURSES_ATTR_T = attr_t;
pub type NCURSES_BOOL = c_char;

type bool_t = NCURSES_BOOL;
type attr_t_p = *attr_t;
type mut_attr_t_p = *mut attr_t;
type short_p = *c_short;
type mut_short_p = *mut c_short;
type void_p = *c_void;
type char_p = *c_char;
type mut_char_p = *mut c_char;
type chtype_p = *chtype;
pub type WINDOW_p = *WINDOW;
pub type SCREEN_p = *SCREEN;
type FILE_p = *FILE;

type va_list = *u8;

struct WINDOW;
struct SCREEN;

#[link_args = "-lncurses"]
extern {
    pub static curscr: WINDOW_p;
    pub static newscr: WINDOW_p;
    pub static stdscr: WINDOW_p;
    pub static ttytype: *c_char;
    pub static COLORS : c_int;
    pub static COLOR_PAIRS : c_int;
    pub static COLS : c_int;
    pub static ESCDELAY : c_int;
    pub static LINES : c_int;
    pub static TABSIZE : c_int;

    pub fn addch (_:chtype) -> c_int;
    pub fn addchnstr (_:*chtype, _:c_int) -> c_int;
    // fn addchstr (_:*chtype) -> c_int;
    pub fn addnstr (_:*c_char, _:c_int) -> c_int;
    pub fn addstr (_:*c_char) -> c_int;
    pub fn attroff (_:attr_t) -> c_int;
    pub fn attron (_:attr_t) -> c_int;
    pub fn attrset (_:attr_t) -> c_int;
    pub fn attr_get (_:mut_attr_t_p, _:mut_short_p, _:void_p) -> c_int;
    // fn attr_off (_:attr_t, _:void_p) -> c_int;
    // fn attr_on (_:attr_t, _:void_p) -> c_int;
    // fn attr_set (_:attr_t, _:c_short, _:void_p) -> c_int;
    pub fn baudrate () -> c_int;
    pub fn beep  () -> c_int;
    pub fn bkgd (_:chtype) -> c_int;
    pub fn bkgdset (_:chtype);
    fn border (_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype) -> c_int;
    fn box (_:WINDOW_p, _:chtype, _:chtype) -> c_int;
    fn can_change_color () -> bool_t;
    pub fn cbreak () -> c_int;
    fn chgat (_:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
    fn clear () -> c_int;
    fn clearok (_:WINDOW_p,_:bool_t) -> c_int;
    fn clrtobot () -> c_int;
    fn clrtoeol () -> c_int;
    fn color_content (_:c_short,_:short_p,_:short_p,_:short_p) -> c_int;
    fn color_set (_:c_short,_:void_p) -> c_int;
    pub fn COLOR_PAIR (_:c_int) -> c_int;
    fn copywin (_:WINDOW_p,_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;
    fn curs_set (_:c_int) -> c_int;
    fn def_prog_mode () -> c_int;
    fn def_shell_mode () -> c_int;
    fn delay_output (_:c_int) -> c_int;
    fn delch () -> c_int;
    pub fn delscreen (_:SCREEN_p);
    fn delwin (_:WINDOW_p) -> c_int;
    fn deleteln () -> c_int;
    fn derwin (_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int) -> *WINDOW;
    fn doupdate () -> c_int;
    fn dupwin (_:WINDOW_p) -> *WINDOW;
    pub fn echo () -> c_int;
    pub fn echochar (_:chtype) -> c_int;
    fn erase () -> c_int;
    pub fn endwin () -> c_int;
    // fn erasechar () -> c_char;
    fn filter ();
    pub fn flash () -> c_int;
    fn flushinp () -> c_int;
    pub fn getbkgd (_:WINDOW_p) -> chtype;
    pub fn getch () -> c_int;
    pub fn getnstr (_:mut_char_p, _:c_int) -> c_int;
    fn getstr (_:char_p) -> c_int;
    fn getwin (_:FILE_p) -> *WINDOW;
    fn halfdelay (_:c_int) -> c_int;
    pub fn has_colors () -> bool_t;
    pub fn has_ic () -> bool_t;
    pub fn has_il () -> bool_t;
    fn hline (_:chtype, _:c_int) -> c_int;
    fn idcok (_:WINDOW_p, _:bool_t);
    fn idlok (_:WINDOW_p, _:bool_t) -> c_int;
    fn immedok (_:WINDOW_p, _:bool_t);
    fn inch () -> chtype;
    fn inchnstr (_:chtype_p, _:c_int) -> c_int;
    fn inchstr (_:chtype_p) -> c_int;
    pub fn initscr () -> *WINDOW;
    fn init_color (_:c_short,_:c_short,_:c_short,_:c_short) -> c_int;
    pub fn init_pair (_:c_short,_:c_short,_:c_short) -> c_int;
    fn innstr (_:char_p, _:c_int) -> c_int;
    fn insch (_:chtype) -> c_int;
    fn insdelln (_:c_int) -> c_int;
    fn insertln () -> c_int;
    fn insnstr (_:char_p, _:c_int) -> c_int;
    fn insstr (_:char_p) -> c_int;
    fn instr (_:char_p) -> c_int;
    fn intrflush (_:WINDOW_p,_:bool_t) -> c_int;
    fn isendwin () -> bool_t;
    fn is_linetouched (_:WINDOW_p,_:c_int) -> bool_t;
    fn is_wintouched (_:WINDOW_p) -> bool_t;
    fn keyname (_:c_int) -> *c_char;
    pub fn keypad (_:WINDOW_p, _:bool_t) -> c_int;
    // fn killchar () -> c_char;
    fn leaveok (_:WINDOW_p,_:bool_t) -> c_int;
    pub fn longname () -> *c_char;
    fn meta (_:WINDOW_p,_:bool_t) -> c_int;
    pub fn move (_:c_int, _:c_int) -> c_int;
    pub fn mvaddch (_:c_int, _:c_int, _:chtype) -> c_int;
    pub fn mvaddchnstr (_:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
    // fn mvaddchstr (_:c_int, _:c_int, _:chtype_p) -> c_int;
    pub fn mvaddnstr (_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    fn mvaddstr (_:c_int, _:c_int, _:char_p) -> c_int;
    fn mvchgat (_:c_int, _:c_int, _:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
    fn mvcur (_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;
    fn mvdelch (_:c_int, _:c_int) -> c_int;
    fn mvderwin (_:WINDOW_p, _:c_int, _:c_int) -> c_int;
    fn mvgetch (_:c_int, _:c_int) -> c_int;
    pub fn mvgetnstr (_:c_int, _:c_int, _:mut_char_p, _:c_int) -> c_int;
    // fn mvgetstr (_:c_int, _:c_int, _:char_p) -> c_int;
    fn mvhline (_:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
    fn mvinch (_:c_int, _:c_int) -> chtype;
    fn mvinchnstr (_:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
    fn mvinchstr (_:c_int, _:c_int, _:chtype_p) -> c_int;
    fn mvinnstr (_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    fn mvinsch (_:c_int, _:c_int, _:chtype) -> c_int;
    fn mvinsnstr (_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    fn mvinsstr (_:c_int, _:c_int, _:char_p) -> c_int;
    fn mvinstr (_:c_int, _:c_int, _:char_p) -> c_int;
    fn mvprintw (_:c_int, _:c_int, _:char_p) -> c_int;
    //  fn mvscanw (_:c_int,_:c_int, _:char_p) -> c_int;
    fn mvvline (_:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
    pub fn mvwaddch (_:WINDOW_p, _:c_int, _:c_int, _:chtype) -> c_int;
    pub fn mvwaddchnstr (_:WINDOW_p, _:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
    fn mvwaddchstr (_:WINDOW_p, _:c_int, _:c_int, _:chtype_p) -> c_int;
    pub fn mvwaddnstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    fn mvwaddstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
    fn mvwchgat (_:WINDOW_p, _:c_int, _:c_int, _:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
    fn mvwdelch (_:WINDOW_p, _:c_int, _:c_int) -> c_int;
    fn mvwgetch (_:WINDOW_p, _:c_int, _:c_int) -> c_int;
    pub fn mvwgetnstr (_:WINDOW_p, _:c_int, _:c_int, _:mut_char_p, _:c_int) -> c_int;
    // fn mvwgetstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
    fn mvwhline (_:WINDOW_p, _:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
    fn mvwin (_:WINDOW_p,_:c_int,_:c_int) -> c_int;
    fn mvwinch (_:WINDOW_p, _:c_int, _:c_int) -> chtype;
    fn mvwinchnstr (_:WINDOW_p, _:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
    fn mvwinchstr (_:WINDOW_p, _:c_int, _:c_int, _:chtype_p) -> c_int;
    fn mvwinnstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    fn mvwinsch (_:WINDOW_p, _:c_int, _:c_int, _:chtype) -> c_int;
    fn mvwinsnstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    fn mvwinsstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
    fn mvwinstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
    fn mvwprintw (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;

    //  fn mvwscanw (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
    fn mvwvline (_:WINDOW_p, _:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
    fn napms (_:c_int) -> c_int;
    fn newpad (_:c_int,_:c_int) -> *WINDOW;
    pub fn newterm (_:char_p,_:FILE_p,_:FILE_p) -> *SCREEN;
    fn newwin (_:c_int,_:c_int,_:c_int,_:c_int) -> *WINDOW;
    pub fn nl () -> c_int;
    pub fn nocbreak () -> c_int;
    fn nodelay (_:WINDOW_p,_:bool_t) -> c_int;
    pub fn noecho () -> c_int;
    pub fn nonl () -> c_int;
    fn noqiflush ();
    pub fn noraw () -> c_int;
    fn notimeout (_:WINDOW_p,_:bool_t) -> c_int;
    fn overlay (_:WINDOW_p,_:WINDOW_p) -> c_int;
    fn overwrite (_:WINDOW_p,_:WINDOW_p) -> c_int;
    fn pair_content (_:c_short,_:short_p,_:short_p) -> c_int;
    fn PAIR_NUMBER (_:c_int) -> c_int;
    fn pechochar (_:WINDOW_p, _:chtype) -> c_int;
    fn pnoutrefresh (_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;
    fn prefresh (_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;

    fn printw (_:char_p) -> c_int;
    fn putwin (_:WINDOW_p, _:FILE_p) -> c_int;
    fn qiflush ();
    pub fn raw () -> c_int;
    fn redrawwin (_:WINDOW_p) -> c_int;
    pub fn refresh () -> c_int;
    fn resetty () -> c_int;
    fn reset_prog_mode () -> c_int;
    fn reset_shell_mode () -> c_int;
    // fn ripoffline (_:c_int, extern  fn f(WINDOW_p, c_int) -> c_int) -> c_int;
    fn ripoffline (_:c_int, f:*u8) -> c_int;
    fn savetty () -> c_int;
    //  fn scanw (_:NCURSES_CONST char_p,...) -> c_int;
    fn scr_dump (_:char_p) -> c_int;
    fn scr_init (_:char_p) -> c_int;
    fn scrl (_:c_int) -> c_int;
    fn scroll (_:WINDOW_p) -> c_int;
    fn scrollok (_:WINDOW_p,_:bool_t) -> c_int;
    fn scr_restore (_:char_p) -> c_int;
    fn scr_set (_:char_p) -> c_int;
    fn setscrreg (_:c_int,_:c_int) -> c_int;
    pub fn set_term (_:SCREEN_p) -> SCREEN_p;
    fn slk_attroff (_:chtype) -> c_int;
    fn slk_attr_off (_:attr_t, _:void_p) -> c_int;
    fn slk_attron (_:chtype) -> c_int;
    fn slk_attr_on (_:attr_t,_:void_p) -> c_int;
    fn slk_attrset (_:chtype) -> c_int;
    fn slk_attr () -> attr_t;
    fn slk_attr_set (_:attr_t,_:c_short,_:void_p) -> c_int;
    fn slk_clear () -> c_int;
    fn slk_color (_:c_short) -> c_int;
    fn slk_init (_:c_int) -> c_int;
    fn slk_label (_:c_int) -> char_p; 
    fn slk_noutrefresh () -> c_int;
    fn slk_refresh () -> c_int;
    fn slk_restore () -> c_int;
    fn slk_set (_:c_int,_:char_p,_:c_int) -> c_int;
    fn slk_touch () -> c_int;
    fn standout () -> c_int;
    fn standend () -> c_int;
    pub fn start_color () -> c_int;
    fn subpad (_:WINDOW_p, _:c_int, _:c_int, _:c_int, _:c_int) -> WINDOW_p;
    fn subwin (_:WINDOW_p, _:c_int, _:c_int, _:c_int, _:c_int) -> WINDOW_p;
    fn syncok (_:WINDOW_p, _:bool_t) -> c_int;
    fn termattrs () -> chtype;
    fn termname () -> char_p;
    fn timeout (_:c_int);
    fn touchline (_:WINDOW_p, _:c_int, _:c_int) -> c_int;
    pub fn touchwin (_:WINDOW_p) -> c_int;
    fn typeahead (_:c_int) -> c_int;
    fn ungetch (_:c_int) -> c_int;
    pub fn untouchwin (_:WINDOW_p) -> c_int;
    fn use_env (_:bool_t);
    fn vidattr (_:chtype) -> c_int;
    //  fn vidputs (_:chtype, extern  fn f(c_int) -> c_int) -> c_int;
    fn vidputs (_:chtype, f:*u8) -> c_int;
    fn vline (_:chtype, _:c_int) -> c_int;
    fn vwprintw (_:WINDOW_p, _:char_p, _:va_list) -> c_int;
    fn vw_printw (_:WINDOW_p, _:char_p,_:va_list) -> c_int;
    //  fn vwscanw (_:WINDOW_p, _:char_p, _:va_list) -> c_int;
    //  fn vw_scanw (_:WINDOW_p, _:char_p, _:va_list) -> c_int;
    pub fn waddch (_:WINDOW_p, _:chtype) -> c_int;
    pub fn waddchnstr (_:WINDOW_p,_:chtype_p,_:c_int) -> c_int;
    // fn waddchstr (_:WINDOW_p,_:chtype_p) -> c_int;
    pub fn waddnstr (_:WINDOW_p,_:char_p,_:c_int) -> c_int;
    // fn waddstr (_:WINDOW_p,_:char_p) -> c_int;
    pub fn wattron (_:WINDOW_p, _:c_int) -> c_int;
    pub fn wattroff (_:WINDOW_p, _:c_int) -> c_int;
    pub fn wattrset (_:WINDOW_p, _:c_int) -> c_int;
    pub fn wattr_get (_:WINDOW_p, _:mut_attr_t_p, _:mut_short_p, _:void_p) -> c_int;
    fn wattr_on (_:WINDOW_p, _:attr_t, _:void_p) -> c_int;
    fn wattr_off (_:WINDOW_p, _:attr_t, _:void_p) -> c_int;
    fn wattr_set (_:WINDOW_p, _:attr_t, _:c_short, _:void_p) -> c_int;
    pub fn wbkgd (_:WINDOW_p, _:chtype) -> c_int;
    pub fn wbkgdset (_:WINDOW_p,_:chtype);
    fn wborder (_:WINDOW_p,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype) -> c_int;
    fn wchgat (_:WINDOW_p, _:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
    fn wclear (_:WINDOW_p) -> c_int;
    fn wclrtobot (_:WINDOW_p) -> c_int;
    fn wclrtoeol (_:WINDOW_p) -> c_int;
    fn wcolor_set (_:WINDOW_p,_:c_short,_:void_p) -> c_int;
    fn wcursyncup (_:WINDOW_p);
    fn wdelch (_:WINDOW_p) -> c_int;
    fn wdeleteln (_:WINDOW_p) -> c_int;
    pub fn wechochar (_:WINDOW_p, _:chtype) -> c_int;
    fn werase (_:WINDOW_p) -> c_int;
    pub fn wgetch (_:WINDOW_p) -> c_int;
    pub fn wgetnstr (_:WINDOW_p,_:mut_char_p,_:c_int) -> c_int;
    fn wgetstr (_:WINDOW_p, _:char_p) -> c_int;
    fn whline (_:WINDOW_p, _:chtype, _:c_int) -> c_int;
    fn winch (_:WINDOW_p) -> chtype;
    fn winchnstr (_:WINDOW_p, _:chtype_p, _:c_int) -> c_int;
    fn winchstr (_:WINDOW_p, _:chtype_p) -> c_int;
    fn winnstr (_:WINDOW_p, _:char_p, _:c_int) -> c_int;
    fn winsch (_:WINDOW_p, _:chtype) -> c_int;
    fn winsdelln (_:WINDOW_p,_:c_int) -> c_int;
    fn winsertln (_:WINDOW_p) -> c_int;
    fn winsnstr (_:WINDOW_p, _:char_p,_:c_int) -> c_int;
    fn winsstr (_:WINDOW_p, _:char_p) -> c_int;
    fn winstr (_:WINDOW_p, _:char_p) -> c_int;
    pub fn wmove (_:WINDOW_p,_:c_int,_:c_int) -> c_int;
    fn wnoutrefresh (_:WINDOW_p) -> c_int;
    fn wprintw (_:WINDOW_p, _:char_p) -> c_int;
    fn wredrawln (_:WINDOW_p,_:c_int,_:c_int) -> c_int;
    pub fn wrefresh (_:WINDOW_p) -> c_int;
    //  fn wscanw (_:WINDOW_p, _:NCURSES_CONST char_p) -> c_int;
    fn wscrl (_:WINDOW_p,_:c_int) -> c_int;
    fn wsetscrreg (_:WINDOW_p,_:c_int,_:c_int) -> c_int;
    fn wstandout (_:WINDOW_p) -> c_int;
    fn wstandend (_:WINDOW_p) -> c_int;
    fn wsyncdown (_:WINDOW_p);
    fn wsyncup (_:WINDOW_p);
    fn wtimeout (_:WINDOW_p,_:c_int);
    fn wtouchln (_:WINDOW_p,_:c_int,_:c_int,_:c_int) -> c_int;
    fn wvline (_:WINDOW_p,_:chtype,_:c_int) -> c_int;

    /*
     * These are also declared in <term.h>:
     */
    fn tigetflag (_:char_p) -> c_int;
    fn tigetnum (_:char_p) -> c_int;
    fn tigetstr (_:char_p) -> *c_char;
    fn putp (_:char_p) -> c_int;

    fn tparm (_:char_p) -> *c_char;

/*
 * These functions are not in X/Open, but we use them in macro definitions:
 */
    fn getattrs (_:WINDOW_p) -> c_int;
    fn getcurx (_:WINDOW_p) -> c_int;
    fn getcury (_:WINDOW_p) -> c_int;
    fn getbegx (_:WINDOW_p) -> c_int;
    fn getbegy (_:WINDOW_p) -> c_int;
    fn getmaxx (_:WINDOW_p) -> c_int;
    fn getmaxy (_:WINDOW_p) -> c_int;
    fn getparx (_:WINDOW_p) -> c_int;
    fn getpary (_:WINDOW_p) -> c_int;
}

/*
 * These functions are extensions - not in X/Open Curses.
 */
extern {
/*
extern NCURSES_EXPORT(bool_t) is_term_resized (int, int);
extern NCURSES_EXPORT(char *) keybound (int, int);
extern NCURSES_EXPORT(const char *) curses_version (void);
extern NCURSES_EXPORT(int) assume_default_colors (int, int);
extern NCURSES_EXPORT(int) define_key (const char *, int);
extern NCURSES_EXPORT(int) key_defined (const char *);
extern NCURSES_EXPORT(int) keyok (int, bool_t);
extern NCURSES_EXPORT(int) resize_term (int, int);
extern NCURSES_EXPORT(int) resizeterm (int, int);
extern NCURSES_EXPORT(int) set_escdelay (int);
extern NCURSES_EXPORT(int) set_tabsize (int);
extern NCURSES_EXPORT(int) use_default_colors (void);
extern NCURSES_EXPORT(int) use_extended_names (bool_t);
extern NCURSES_EXPORT(int) use_legacy_coding (int);
//typedef int (*NCURSES_WINDOW_CB)(WINDOW *, void *);
//extern NCURSES_EXPORT(int) use_screen (SCREEN *, NCURSES_SCREEN_CB, void *);
//typedef int (*NCURSES_SCREEN_CB)(SCREEN *, void *);
//extern NCURSES_EXPORT(int) use_window (WINDOW *, NCURSES_WINDOW_CB, void *);
extern NCURSES_EXPORT(int) wresize (WINDOW *, int, int);
extern NCURSES_EXPORT(void) nofilter(void);
*/
}

/*
 * These extensions provide access to information stored in the WINDOW even
 * when NCURSES_OPAQUE is set:
 */
extern {
    fn wgetparent (_:WINDOW_p) -> WINDOW_p;
    fn is_cleared (_:WINDOW_p) -> bool_t;
    fn is_idcok (_:WINDOW_p) -> bool_t;
    fn is_idlok (_:WINDOW_p) -> bool_t;
    fn is_immedok (_:WINDOW_p) -> bool_t;
    fn is_keypad (_:WINDOW_p) -> bool_t;
    fn is_leaveok (_:WINDOW_p) -> bool_t;
    fn is_nodelay (_:WINDOW_p) -> bool_t;
    fn is_notimeout (_:WINDOW_p) -> bool_t;
    fn is_scrollok (_:WINDOW_p) -> bool_t;
    fn is_syncok (_:WINDOW_p) -> bool_t;
    fn wgetscrreg (_:WINDOW_p, _:*c_int, _:*c_int) -> c_int;
}

/* attributes */

pub static NCURSES_ATTR_SHIFT  : uint =       8;
fn NCURSES_BITS(mask:uint,shift:uint) -> uint { ((mask) << ((shift) + NCURSES_ATTR_SHIFT) as i32) }
macro_rules! ncurses_bits {
    ($mask:expr, $shift:expr)
        =>
        ( (($mask) << (($shift) + NCURSES_ATTR_SHIFT) as i32) )
}
pub static A_NORMAL: c_int     = (1u - 1u) as c_int;
pub static A_ATTRIBUTES: c_int = ncurses_bits!(!(1u - 1u),0)  as c_int;
pub static A_CHARTEXT: c_int   = (ncurses_bits!(1u,0) - 1u)  as c_int;
pub static A_COLOR: c_int      = ncurses_bits!(((1u) << 8) - 1u,0)  as c_int;
pub static A_STANDOUT: c_int   = ncurses_bits!(1u,8)   as c_int;
pub static A_UNDERLINE: c_int  = ncurses_bits!(1u,9)   as c_int;
pub static A_REVERSE: c_int    = ncurses_bits!(1u,10)  as c_int;
pub static A_BLINK: c_int      = ncurses_bits!(1u,11)  as c_int;
pub static A_DIM: c_int        = ncurses_bits!(1u,12)  as c_int;
pub static A_BOLD: c_int       = ncurses_bits!(1u,13)  as c_int;
pub static A_ALTCHARSET: c_int = ncurses_bits!(1u,14)  as c_int;
pub static A_INVIS: c_int      = ncurses_bits!(1u,15)  as c_int;
pub static A_PROTECT: c_int    = ncurses_bits!(1u,16)  as c_int;
pub static A_HORIZONTAL: c_int = ncurses_bits!(1u,17)  as c_int;
pub static A_LEFT: c_int       = ncurses_bits!(1u,18)  as c_int;
pub static A_LOW: c_int        = ncurses_bits!(1u,19)  as c_int;
pub static A_RIGHT: c_int      = ncurses_bits!(1u,20)  as c_int;
pub static A_TOP: c_int        = ncurses_bits!(1u,21)  as c_int;
pub static A_VERTICAL: c_int   = ncurses_bits!(1u,22)  as c_int;

/*
 * Most of the pseudo functions are macros that either provide compatibility
 * with older versions of curses, or provide inline functionality to improve
 * performance.
 */

/*
 * These pseudo functions are always implemented as macros:
 */

#[fixed_stack_segment]
pub unsafe fn getyx(win:WINDOW_p,y: &mut c_int,x: &mut c_int) { *y = getcury(win); *x = getcurx(win); }
#[fixed_stack_segment]
unsafe fn getbegyx(win:WINDOW_p,y: &mut c_int, x: &mut c_int) { *y = getbegy(win); *x = getbegx(win) }
#[fixed_stack_segment]
pub unsafe fn getmaxyx(win:WINDOW_p,y: &mut c_int, x: &mut c_int) { *y = getmaxy(win); *x = getmaxx(win) }
#[fixed_stack_segment]
unsafe fn getparyx(win:WINDOW_p,y: &mut c_int, x: &mut c_int) { *y = getpary(win); *x = getparx(win) }

#[fixed_stack_segment]
unsafe fn getsyx(y:&mut c_int, x:&mut c_int) {
    if newscr != (0 as WINDOW_p) {
        if is_leaveok(newscr) != 0 {
            *x = -1 as c_int;
            *y = -1 as c_int;
        } else {
            getyx(newscr,(y), (x));
        }
    }
}

#[fixed_stack_segment]
unsafe fn setsyx(y:&mut c_int,x:&mut c_int) {
    if newscr != (0 as WINDOW_p) {
        if *y == -1 && *x == -1 {
            leaveok(newscr, true as c_char);
        } else {
            leaveok(newscr, false as c_char);
            wmove(newscr, *y, *x);
        }
    }
}

//#define setterm(term) setupterm(term, 1, (int *)0)

//#define fixterm() reset_prog_mode()
//#define resetterm() reset_shell_mode()
//#define saveterm() def_prog_mode()
//#define crmode() cbreak()
//#define nocrmode() nocbreak()
//#define gettmode()

/* colors */
pub static COLOR_BLACK :   c_int =   0;
pub static COLOR_RED :     c_int =   1;
pub static COLOR_GREEN :   c_int =   2;
pub static COLOR_YELLOW :  c_int =   3;
pub static COLOR_BLUE :    c_int =   4;
pub static COLOR_MAGENTA : c_int =   5;
pub static COLOR_CYAN :    c_int =   6;
pub static COLOR_WHITE :   c_int =   7;

/* line graphics */

extern {
    pub static acs_map : chtype_p;
}

unsafe fn NCURSES_ACS(c:char) -> c_uchar {
    *((acs_map as i32 + 4*(c as c_int)) as chtype_p) as c_uchar
}

/* VT100 symbols begin here */
unsafe fn ACS_ULCORNER() -> c_uchar { NCURSES_ACS('l') } /* upper left corner */
unsafe fn ACS_LLCORNER() -> c_uchar { NCURSES_ACS('m') } /* lower left corner */
unsafe fn ACS_URCORNER() -> c_uchar { NCURSES_ACS('k') } /* upper right corner */
unsafe fn ACS_LRCORNER() -> c_uchar { NCURSES_ACS('j') } /* lower right corner */
unsafe fn ACS_LTEE() -> c_uchar { NCURSES_ACS('t') } /* tee pointing right */
unsafe fn ACS_RTEE() -> c_uchar { NCURSES_ACS('u') } /* tee pointing left */
unsafe fn ACS_BTEE() -> c_uchar { NCURSES_ACS('v') } /* tee pointing up */
unsafe fn ACS_TTEE() -> c_uchar { NCURSES_ACS('w') } /* tee pointing down */
unsafe fn ACS_HLINE() -> c_uchar { NCURSES_ACS('q') } /* horizontal line */
unsafe fn ACS_VLINE() -> c_uchar { NCURSES_ACS('x') } /* vertical line */
unsafe fn ACS_PLUS() -> c_uchar { NCURSES_ACS('n') } /* large plus or crossover */
unsafe fn ACS_S1() -> c_uchar { NCURSES_ACS('o') } /* scan line 1 */
unsafe fn ACS_S9() -> c_uchar { NCURSES_ACS('s') } /* scan line 9 */
unsafe fn ACS_DIAMOND() -> c_uchar { NCURSES_ACS('`') } /* diamond */
unsafe fn ACS_CKBOARD() -> c_uchar { NCURSES_ACS('a') } /* checker board (stipple) */
unsafe fn ACS_DEGREE() -> c_uchar { NCURSES_ACS('f') } /* degree symbol */
unsafe fn ACS_PLMINUS() -> c_uchar { NCURSES_ACS('g') } /* plus/minus */
unsafe fn ACS_BULLET() -> c_uchar { NCURSES_ACS('~') } /* bullet */
/* Teletype 5410v1 symbols begin here */
unsafe fn ACS_LARROW() -> c_uchar { NCURSES_ACS(',') } /* arrow pointing left */
unsafe fn ACS_RARROW() -> c_uchar { NCURSES_ACS('+') } /* arrow pointing right */
unsafe fn ACS_DARROW() -> c_uchar { NCURSES_ACS('.') } /* arrow pointing down */
unsafe fn ACS_UARROW() -> c_uchar { NCURSES_ACS('-') } /* arrow pointing up */
unsafe fn ACS_BOARD() -> c_uchar { NCURSES_ACS('h') } /* board of squares */
unsafe fn ACS_LANTERN() -> c_uchar { NCURSES_ACS('i') } /* lantern symbol */
unsafe fn ACS_BLOCK() -> c_uchar { NCURSES_ACS('0') } /* solid square block */
/*
 * These aren't documented, but a lot of System Vs have them anyway
 * (you can spot pprryyzz{{||}} in a lot of AT&T terminfo strings).
 * The ACS_names may not match AT&T's, our source didn't know them.
 */
unsafe fn ACS_S3() -> c_uchar { NCURSES_ACS('p') } /* scan line 3 */
unsafe fn ACS_S7() -> c_uchar { NCURSES_ACS('r') } /* scan line 7 */
unsafe fn ACS_LEQUAL() -> c_uchar { NCURSES_ACS('y') } /* less/equal */
unsafe fn ACS_GEQUAL() -> c_uchar { NCURSES_ACS('z') } /* greater/equal */
unsafe fn ACS_PI() -> c_uchar { NCURSES_ACS('{') } /* Pi */
unsafe fn ACS_NEQUAL() -> c_uchar { NCURSES_ACS('|') } /* not equal */
unsafe fn ACS_STERLING() -> c_uchar { NCURSES_ACS('}') } /* UK pound sign */

/*
 * Line drawing ACS names are of the form ACS_trbl, where t is the top, r
 * is the right, b is the bottom, and l is the left.  t, r, b, and l might
 * be B (blank), S (single), D (double), or T (thick).  The subset defined
 * here only uses B and S.
 */
unsafe fn ACS_BSSB() -> c_uchar { ACS_ULCORNER() }
unsafe fn ACS_SSBB() -> c_uchar { ACS_LLCORNER() }
unsafe fn ACS_BBSS() -> c_uchar { ACS_URCORNER() }
unsafe fn ACS_SBBS() -> c_uchar { ACS_LRCORNER() }
unsafe fn ACS_SBSS() -> c_uchar { ACS_RTEE() }
unsafe fn ACS_SSSB() -> c_uchar { ACS_LTEE() }
unsafe fn ACS_SSBS() -> c_uchar { ACS_BTEE() }
unsafe fn ACS_BSSS() -> c_uchar { ACS_TTEE() }
unsafe fn ACS_BSBS() -> c_uchar { ACS_HLINE() }
unsafe fn ACS_SBSB() -> c_uchar { ACS_VLINE() }
unsafe fn ACS_SSSS() -> c_uchar { ACS_PLUS() }


pub static ERR:c_int =     (-1 as c_int);

pub static OK:c_int =      (0);

/* values for the _flags member */
pub static _SUBWIN:c_int    =     0x01; /* is this a sub-window? */
pub static _ENDLINE:c_int   =     0x02;/* is the window flush right? */
pub static _FULLWIN:c_int   =     0x04;/* is the window full-screen? */
pub static _SCROLLWIN:c_int =     0x08;/* bottom edge is at screen bottom? */
pub static _ISPAD:c_int     =     0x10;/* is this window a pad? */
pub static _HASMOVED:c_int  =     0x20;/* has cursor moved since last refresh? */
pub static _WRAPPED:c_int   =     0x40;/* cursor was just wrappped */

/*
 * this value is used in the firstchar and lastchar fields to mark
 * unchanged lines
 */
pub static _NOCHANGE:c_int =       -1;

/*
 * this value is used in the oldindex field to mark lines created by insertions
 * and scrolls.
 */
pub static _NEWINDEX:c_int = -1;

/*
 * Pseudo-character tokens outside ASCII range.  The curses wgetch() function
 * will return any given one of these only if the corresponding k- capability
 * is defined in your terminal's terminfo entry.
 *
 * Some keys (KEY_A1, etc) are arranged like this:
 *      a1     up    a3
 *      left   b2    right
 *      c1     down  c3
 *
 * A few key codes do not depend upon the terminfo entry.
 */
pub static KEY_CODE_YES:c_int = 0o400;      /* A wchar_t contains a key code */
pub static KEY_MIN:c_int =      0o401;      /* Minimum curses key */
pub static KEY_BREAK:c_int =    0o401;      /* Break key (unreliable) */
pub static KEY_SRESET:c_int =   0o530;      /* Soft (partial) reset (unreliable) */
pub static KEY_RESET:c_int =    0o531;      /* Reset or hard reset (unreliable) */
/*
 * These definitions were generated by /tmp/ncurses-27.roots/ncurses-27/ncurses/include/MKkey_defs.sh /tmp/ncurses-27.roots/ncurses-27/ncurses/include/Caps
 */
pub static KEY_DOWN:c_int      = 0o402; /* down-arrow key */
pub static KEY_UP:c_int        = 0o403; /* up-arrow key */
pub static KEY_LEFT:c_int      = 0o404; /* left-arrow key */
pub static KEY_RIGHT:c_int     = 0o405; /* right-arrow key */
pub static KEY_HOME:c_int      = 0o406; /* home key */
pub static KEY_BACKSPACE:c_int = 0o407; /* backspace key */
pub static KEY_F0:c_int        = 0o410;/* Function keys.  Space for 64 */
pub static KEY_F1:c_int        = KEY_F0 + 1;
pub static KEY_F2:c_int        = KEY_F0 + 2;
pub static KEY_F3:c_int        = KEY_F0 + 3;
pub static KEY_F4:c_int        = KEY_F0 + 4;
pub static KEY_F5:c_int        = KEY_F0 + 5;
pub static KEY_F6:c_int        = KEY_F0 + 6;
pub static KEY_F7:c_int        = KEY_F0 + 7;
pub static KEY_F8:c_int        = KEY_F0 + 8;
pub static KEY_F9:c_int        = KEY_F0 + 9;
pub static KEY_F10:c_int       = KEY_F0 + 10;
pub static KEY_F11:c_int       = KEY_F0 + 11;
pub static KEY_F12:c_int       = KEY_F0 + 12;
pub static KEY_F13:c_int       = KEY_F0 + 13;
pub static KEY_F14:c_int       = KEY_F0 + 14;
pub static KEY_F15:c_int       = KEY_F0 + 15;
//pub static KEY_F(n) (KEY_F0+(n))      /* Value of function key n */

pub static KEY_DL:c_int        = 0o510; /* delete-line key */
pub static KEY_IL:c_int        = 0o511; /* insert-line key */
pub static KEY_DC:c_int        = 0o512; /* delete-character key */
pub static KEY_IC:c_int        = 0o513; /* insert-character key */
pub static KEY_EIC:c_int       = 0o514; /* sent by rmir or smir in insert mode */
pub static KEY_CLEAR:c_int     = 0o515; /* clear-screen or erase key */
pub static KEY_EOS:c_int       = 0o516; /* clear-to-end-of-screen key */
pub static KEY_EOL:c_int       = 0o517; /* clear-to-end-of-line key */
pub static KEY_SF:c_int        = 0o520; /* scroll-forward key */
pub static KEY_SR:c_int        = 0o521; /* scroll-backward key */
pub static KEY_NPAGE:c_int     = 0o522; /* next-page key */
pub static KEY_PPAGE:c_int     = 0o523; /* previous-page key */
pub static KEY_STAB:c_int      = 0o524; /* set-tab key */
pub static KEY_CTAB:c_int      = 0o525; /* clear-tab key */
pub static KEY_CATAB:c_int     = 0o526; /* clear-all-tabs key */
pub static KEY_ENTER:c_int     = 0o527; /* enter/send key */
pub static KEY_PRINT:c_int     = 0o532; /* print key */
pub static KEY_LL:c_int        = 0o533; /* lower-left key (home down) */
pub static KEY_A1:c_int        = 0o534; /* upper left of keypad */
pub static KEY_A3:c_int        = 0o535; /* upper right of keypad */
pub static KEY_B2:c_int        = 0o536; /* center of keypad */
pub static KEY_C1:c_int        = 0o537; /* lower left of keypad */
pub static KEY_C3:c_int        = 0o540; /* lower right of keypad */
pub static KEY_BTAB:c_int      = 0o541; /* back-tab key */
pub static KEY_BEG:c_int       = 0o542; /* begin key */
pub static KEY_CANCEL:c_int    = 0o543; /* cancel key */
pub static KEY_CLOSE:c_int     = 0o544; /* close key */
pub static KEY_COMMAND:c_int   = 0o545; /* command key */
pub static KEY_COPY:c_int      = 0o546; /* copy key */
pub static KEY_CREATE:c_int    = 0o547; /* create key */
pub static KEY_END:c_int       = 0o550; /* end key */
pub static KEY_EXIT:c_int      = 0o551; /* exit key */
pub static KEY_FIND:c_int      = 0o552; /* find key */
pub static KEY_HELP:c_int      = 0o553; /* help key */
pub static KEY_MARK:c_int      = 0o554; /* mark key */
pub static KEY_MESSAGE:c_int   = 0o555; /* message key */
pub static KEY_MOVE:c_int      = 0o556; /* move key */
pub static KEY_NEXT:c_int      = 0o557; /* next key */
pub static KEY_OPEN:c_int      = 0o560; /* open key */
pub static KEY_OPTIONS:c_int   = 0o561; /* options key */
pub static KEY_PREVIOUS:c_int  = 0o562; /* previous key */
pub static KEY_REDO:c_int      = 0o563; /* redo key */
pub static KEY_REFERENCE:c_int = 0o564; /* reference key */
pub static KEY_REFRESH:c_int   = 0o565; /* refresh key */
pub static KEY_REPLACE:c_int   = 0o566; /* replace key */
pub static KEY_RESTART:c_int   = 0o567; /* restart key */
pub static KEY_RESUME:c_int    = 0o570; /* resume key */
pub static KEY_SAVE:c_int      = 0o571; /* save key */
pub static KEY_SBEG:c_int      = 0o572; /* shifted begin key */
pub static KEY_SCANCEL:c_int   = 0o573; /* shifted cancel key */
pub static KEY_SCOMMAND:c_int  = 0o574; /* shifted command key */
pub static KEY_SCOPY:c_int     = 0o575; /* shifted copy key */
pub static KEY_SCREATE:c_int   = 0o576; /* shifted create key */
pub static KEY_SDC:c_int       = 0o577; /* shifted delete-character key */
pub static KEY_SDL:c_int       = 0o600; /* shifted delete-line key */
pub static KEY_SELECT:c_int    = 0o601; /* select key */
pub static KEY_SEND:c_int      = 0o602; /* shifted end key */
pub static KEY_SEOL:c_int      = 0o603; /* shifted clear-to-end-of-line key */
pub static KEY_SEXIT:c_int     = 0o604; /* shifted exit key */
pub static KEY_SFIND:c_int     = 0o605; /* shifted find key */
pub static KEY_SHELP:c_int     = 0o606; /* shifted help key */
pub static KEY_SHOME:c_int     = 0o607; /* shifted home key */
pub static KEY_SIC:c_int       = 0o610; /* shifted insert-character key */
pub static KEY_SLEFT:c_int     = 0o611; /* shifted left-arrow key */
pub static KEY_SMESSAGE:c_int  = 0o612; /* shifted message key */
pub static KEY_SMOVE:c_int     = 0o613; /* shifted move key */
pub static KEY_SNEXT:c_int     = 0o614; /* shifted next key */
pub static KEY_SOPTIONS:c_int  = 0o615; /* shifted options key */
pub static KEY_SPREVIOUS:c_int = 0o616; /* shifted previous key */
pub static KEY_SPRINT:c_int    = 0o617; /* shifted print key */
pub static KEY_SREDO:c_int     = 0o620; /* shifted redo key */
pub static KEY_SREPLACE:c_int  = 0o621; /* shifted replace key */
pub static KEY_SRIGHT:c_int    = 0o622; /* shifted right-arrow key */
pub static KEY_SRSUME:c_int    = 0o623; /* shifted resume key */
pub static KEY_SSAVE:c_int     = 0o624; /* shifted save key */
pub static KEY_SSUSPEND:c_int  = 0o625; /* shifted suspend key */
pub static KEY_SUNDO:c_int     = 0o626; /* shifted undo key */
pub static KEY_SUSPEND:c_int   = 0o627; /* suspend key */
pub static KEY_UNDO:c_int      = 0o630; /* undo key */
pub static KEY_MOUSE:c_int     = 0o631; /* Mouse event has occurred */
pub static KEY_RESIZE:c_int    = 0o632; /* Terminal resize event */
pub static KEY_EVENT:c_int     = 0o633; /* We were interrupted by an event */

pub static KEY_MAX:c_int       = 0x1ff; /* Maximum key value is 0633 */

static CCHARW_MAX:c_uint = 5;
pub struct cchar_t
{
    attr: attr_t,
    chars: [wchar_t, ..CCHARW_MAX],
}

// FSK skipping NCURSES_WACS table lookup macros.  (It is not clear if
// they are really of interest in a world with Unicode, we will see.)

/*
 * Function prototypes for wide-character operations.
 *
 * "generated" comments should include ":WIDEC" to make the corresponding
 * functions ifdef'd in lib_gen.c
 *
 * "implemented" comments do not need this marker.
 */

// (this at least works on OS X, apparently.)
pub type wint_t = i32;
pub static WEOF : wint_t = EOF;

extern {
    pub fn add_wch(_:*cchar_t) -> c_int;
    fn add_wchnstr(_:*cchar_t, _:c_int) -> c_int;
    fn add_wchstr(_:*cchar_t) -> c_int;
    pub fn addnwstr(_:*wchar_t, _:c_int) -> c_int;
    fn addwstr(_:*wchar_t) -> c_int;
    fn bkgrnd(_:*cchar_t) -> c_int;
    fn bkgrndset(_:*cchar_t);
    fn border_set(_:*cchar_t,_:*cchar_t,_:*cchar_t,_:*cchar_t,_:*cchar_t,_:*cchar_t,_:*cchar_t,_:*cchar_t) -> c_int;
    fn box_set(_:WINDOW_p, _:*cchar_t, _:*cchar_t) -> c_int;
    fn echo_wchar(_:*cchar_t) -> c_int;
    pub fn erasewchar(_:*mut wchar_t) -> c_int;
    pub fn get_wch(_:*mut wint_t) -> c_int;
    // fn get_wstr(_:*mut wint_t) -> c_int;
    fn getbkgrnd(_:*mut cchar_t) -> c_int;
    fn getcchar(_:*cchar_t, _:*mut wchar_t, _:*mut attr_t, _:*mut c_short, _:*c_void) -> c_int;
    pub fn getn_wstr(_:*mut wint_t, _:c_int) -> c_int;
    fn hline_set(_:*cchar_t, _:c_int) -> c_int;
    fn in_wch(_:*mut cchar_t) -> c_int;
    fn in_wchnstr(_:*mut cchar_t, _:c_int) -> c_int;
    fn in_wchstr(_:*mut cchar_t) -> c_int;
    fn innwstr(_:*mut wchar_t, _:c_int) -> c_int;
    fn ins_nwstr(_:*wchar_t, _:c_int) -> c_int;
    fn ins_wch(_:*cchar_t) -> c_int;
    fn ins_wstr(_:*wchar_t) -> c_int;
    fn inwstr(_:*mut wchar_t) -> c_int;
    fn key_name(_:wchar_t) -> *c_char;
    pub fn killwchar(_:*mut wchar_t) -> c_int;
    fn mvadd_wch(_:c_int, _:c_int, _:*cchar_t) -> c_int;
    fn mvadd_wchnstr(_:c_int, _:c_int, _:*cchar_t, _:c_int) -> c_int;
    fn mvadd_wchstr(_:c_int, _:c_int, _:*cchar_t) -> c_int;
    fn mvaddnwstr(_:c_int, _:c_int, _:*wchar_t, _:c_int) -> c_int;
    fn mvaddwstr(_:c_int, _:c_int, _:*wchar_t) -> c_int;
    fn mvget_wch(_:c_int, _:c_int, _:*mut wint_t) -> c_int;
    fn mvget_wstr(_:c_int, _:c_int, _:*mut wint_t) -> c_int;
    pub fn mvgetn_wstr(_:c_int, _:c_int, _:*mut wint_t, _:c_int) -> c_int;
    fn mvhline_set(_:c_int, _:c_int, _:*cchar_t, _:c_int) -> c_int;
    fn mvin_wch(_:c_int, _:c_int, _:*mut cchar_t) -> c_int;
    fn mvin_wchnstr(_:c_int, _:c_int, _:*mut cchar_t, _:c_int) -> c_int;
    fn mvin_wchstr(_:c_int, _:c_int, _:*mut cchar_t) -> c_int;
    fn mvinnwstr(_:c_int, _:c_int, _:*mut wchar_t, _:c_int) -> c_int;
    fn mvins_nwstr (_:c_int, _:c_int, _:*wchar_t, _:c_int) -> c_int;
    fn mvins_wch (_:c_int, _:c_int, _:*cchar_t) -> c_int;
    fn mvins_wstr (_:c_int, _:c_int, _:*wchar_t) -> c_int;
    fn mvinwstr (_:c_int, _:c_int, _:*mut wchar_t) -> c_int;
    fn mvvline_set (_:c_int, _:c_int, _:*cchar_t, _:c_int) -> c_int;
    fn mvwadd_wch (_:WINDOW_p, _:c_int, _:c_int, _:*cchar_t) -> c_int;
    fn mvwadd_wchnstr (_:WINDOW_p, _:c_int, _:c_int, _:*cchar_t, _:c_int) -> c_int;
    fn mvwadd_wchstr (_:WINDOW_p, _:c_int, _:c_int, _:*cchar_t) -> c_int;
    fn mvwaddnwstr (_:WINDOW_p, _:c_int, _:c_int, _:*wchar_t, _:c_int) -> c_int;
    fn mvwaddwstr (_:WINDOW_p, _:c_int, _:c_int, _:*wchar_t) -> c_int;
    fn mvwget_wch (_:WINDOW_p, _:c_int, _:c_int, _:*mut wint_t) -> c_int;
    fn mvwget_wstr (_:WINDOW_p, _:c_int, _:c_int, _:*mut wint_t) -> c_int;
    fn mvwgetn_wstr (_:WINDOW_p, _:c_int, _:c_int, _:*mut wint_t, _:c_int) -> c_int;
    fn mvwhline_set (_:WINDOW_p, _:c_int, _:c_int, _:*cchar_t, _:c_int) -> c_int;
    fn mvwin_wch (_:WINDOW_p, _:c_int, _:c_int, _:*mut cchar_t) -> c_int;
    fn mvwin_wchnstr (_:WINDOW_p, _:c_int, _:c_int, _:*mut cchar_t, _:c_int) -> c_int;
    fn mvwin_wchstr (_:WINDOW_p, _:c_int, _:c_int, _:*mut cchar_t) -> c_int;
    fn mvwinnwstr (_:WINDOW_p, _:c_int, _:c_int, _:*mut wchar_t, _:c_int) -> c_int;
    fn mvwins_nwstr (_:WINDOW_p, _:c_int, _:c_int, _:*wchar_t, _:c_int) -> c_int;
    fn mvwins_wch (_:WINDOW_p, _:c_int, _:c_int, _:*cchar_t) -> c_int;
    fn mvwins_wstr (_:WINDOW_p, _:c_int, _:c_int, _:*wchar_t) -> c_int;
    fn mvwinwstr (_:WINDOW_p, _:c_int, _:c_int, _:*mut wchar_t) -> c_int;
    fn mvwvline_set (_:WINDOW_p, _:c_int, _:c_int, _:*cchar_t, _:c_int) -> c_int;
    fn pecho_wchar (_:WINDOW_p, _:*cchar_t) -> c_int;
    fn setcchar (_:*mut cchar_t, _:*wchar_t, _:attr_t, _:c_short, _:*c_void) -> c_int;
    fn slk_wset (_:c_int, _:*wchar_t, _:c_int) -> c_int;
    fn term_attrs () -> attr_t;
    fn unget_wch (_:wchar_t) -> c_int;
    fn vid_attr (_:attr_t, _:c_short, _:*c_void) -> c_int;
    fn vid_puts (_:attr_t, _:c_short, _:*c_void, _:&fn (_:c_int) -> c_int) -> c_int;
    fn vline_set (_:*cchar_t, _:c_int) -> c_int;
    fn wadd_wch (_:WINDOW_p, _:*cchar_t) -> c_int;
    fn wadd_wchnstr (_:WINDOW_p, _:*cchar_t, _:c_int) -> c_int;
    fn wadd_wchstr (_:WINDOW_p, _:*cchar_t) -> c_int;
    fn waddnwstr (_:WINDOW_p, _:*wchar_t, _:c_int) -> c_int;
    fn waddwstr (_:WINDOW_p, _:*wchar_t) -> c_int;
    fn wbkgrnd (_:WINDOW_p, _:*cchar_t) -> c_int;
    fn wbkgrndset (_:WINDOW_p, _:*cchar_t);
    fn wborder_set (_:WINDOW_p, _:*cchar_t, _:*cchar_t, _:*cchar_t, _:*cchar_t, _:*cchar_t, _:*cchar_t, _:*cchar_t, _:*cchar_t) -> c_int;
    fn wecho_wchar (_:WINDOW_p, _:*cchar_t) -> c_int;
    pub fn wget_wch (_:WINDOW_p, _:*mut wint_t) -> c_int;
    fn wget_wstr (_:WINDOW_p, _:*mut wint_t) -> c_int;
    fn wgetbkgrnd (_:WINDOW_p, _:*mut cchar_t) -> c_int;
    fn wgetn_wstr (_:WINDOW_p, _:*mut wint_t, _:c_int) -> c_int;
    fn whline_set (_:WINDOW_p, _:* cchar_t, _:c_int) -> c_int;
    fn win_wch (_:WINDOW_p, _:*mut cchar_t) -> c_int;
    fn win_wchnstr (_:WINDOW_p, _:*mut cchar_t, _:c_int) -> c_int;
    fn win_wchstr (_:WINDOW_p, _:*mut cchar_t) -> c_int;
    fn winnwstr (_:WINDOW_p, _:*mut wchar_t, _:c_int) -> c_int;
    fn wins_nwstr (_:WINDOW_p, _:*wchar_t, _:c_int) -> c_int;
    fn wins_wch (_:WINDOW_p, _:*cchar_t) -> c_int;
    fn wins_wstr (_:WINDOW_p, _:*wchar_t) -> c_int;
    fn winwstr (_:WINDOW_p, _:*mut wchar_t) -> c_int;
    fn wunctrl (_:*mut cchar_t) -> *wchar_t;
    fn wvline_set (_:WINDOW_p, _:*cchar_t, _:c_int) -> c_int;
}
// FSK skipping mouse support material
/* $Id: curses.tail,v 1.16 2008/07/05 20:20:38 tom Exp $ */
