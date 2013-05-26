#[link(name="ncurses",vers="5.7")];
use std::libc::{c_char, c_int, c_short, c_uchar, c_uint, c_void,FILE};

pub type chtype = c_uint;
pub type attr_t = c_int;
pub type NCURSES_ATTR_T = attr_t;

type attr_t_p = *attr_t;
type short_p = *c_short;
type void_p = *c_void;
type char_p = *c_char;
type chtype_p = *chtype;
type WINDOW_p = *WINDOW;
type SCREEN_p = *SCREEN;
type FILE_p = *FILE;

type va_list = *u8;

struct WINDOW;
struct SCREEN;

#[link_args = "-lncurses"]
pub extern {
    static curscr: WINDOW_p;
    static newscr: WINDOW_p;
    static stdscr: WINDOW_p;
    static ttytype: *c_char;
    static COLORS : c_int;
    static COLOR_PAIRS : int;
    static COLS : int;
    static ESCDELAY : int;
    static LINES : int;
    static TABSIZE : int;

    unsafe fn addch (_:chtype) -> c_int;
    unsafe fn addchnstr (_:*chtype, _:c_int) -> c_int;
    unsafe fn addchstr (_:*chtype) -> c_int;
    unsafe fn addnstr (_:*c_char, _:c_int) -> c_int;
    unsafe fn addstr (_:*c_char) -> c_int;
    unsafe fn attroff (_:NCURSES_ATTR_T) -> c_int;
    unsafe fn attron (_:NCURSES_ATTR_T) -> c_int;
    unsafe fn attrset (_:NCURSES_ATTR_T) -> c_int;
    unsafe fn attr_get (_:attr_t_p, _:short_p, _:void_p) -> c_int;
    unsafe fn attr_off (_:attr_t, _:void_p) -> c_int;
    unsafe fn attr_on (_:attr_t, _:void_p) -> c_int;
    unsafe fn attr_set (_:attr_t, _:c_short, _:void_p) -> c_int;
    unsafe fn baudrate () -> c_int;
    unsafe fn beep  () -> c_int;
    unsafe fn bkgd (_:chtype) -> c_int;
    unsafe fn bkgdset (_:chtype);
    unsafe fn border (_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype) -> c_int;
    unsafe fn box (_:WINDOW_p, _:chtype, _:chtype) -> c_int;
    unsafe fn can_change_color () -> bool;
    unsafe fn cbreak () -> c_int;
    unsafe fn chgat (_:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
    unsafe fn clear () -> c_int;
    unsafe fn clearok (_:WINDOW_p,_:bool) -> c_int;
    unsafe fn clrtobot () -> c_int;
    unsafe fn clrtoeol () -> c_int;
    unsafe fn color_content (_:c_short,_:short_p,_:short_p,_:short_p) -> c_int;
    unsafe fn color_set (_:c_short,_:void_p) -> c_int;
    unsafe fn COLOR_PAIR (_:c_int) -> c_int;
    unsafe fn copywin (_:WINDOW_p,_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;
    unsafe fn curs_set (_:c_int) -> c_int;
    unsafe fn def_prog_mode () -> c_int;
    unsafe fn def_shell_mode () -> c_int;
    unsafe fn delay_output (_:c_int) -> c_int;
    unsafe fn delch () -> c_int;
    unsafe fn delscreen (_:SCREEN_p);
    unsafe fn delwin (_:WINDOW_p) -> c_int;
    unsafe fn deleteln () -> c_int;
    unsafe fn derwin (_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int) -> *WINDOW;
    unsafe fn doupdate () -> c_int;
    unsafe fn dupwin (_:WINDOW_p) -> *WINDOW;
    unsafe fn echo () -> c_int;
    unsafe fn echochar (_:chtype) -> c_int;
    unsafe fn erase () -> c_int;
    unsafe fn endwin () -> c_int;
    unsafe fn erasechar () -> c_char;
    unsafe fn filter ();
    unsafe fn flash () -> c_int;
    unsafe fn flushinp () -> c_int;
    unsafe fn getbkgd (_:WINDOW_p) -> chtype;
    unsafe fn getch () -> c_int;
    unsafe fn getnstr (_:char_p, _:c_int) -> c_int;
    unsafe fn getstr (_:char_p) -> c_int;
    unsafe fn getwin (_:FILE_p) -> *WINDOW;
    unsafe fn halfdelay (_:c_int) -> c_int;
    unsafe fn has_colors () -> bool;
    unsafe fn has_ic () -> bool;
    unsafe fn has_il () -> bool;
    unsafe fn hline (_:chtype, _:c_int) -> c_int;
    unsafe fn idcok (_:WINDOW_p, _:bool);
    unsafe fn idlok (_:WINDOW_p, _:bool) -> c_int;
    unsafe fn immedok (_:WINDOW_p, _:bool);
    unsafe fn inch () -> chtype;
    unsafe fn inchnstr (_:chtype_p, _:c_int) -> c_int;
    unsafe fn inchstr (_:chtype_p) -> c_int;
    unsafe fn initscr () -> *WINDOW;
    unsafe fn init_color (_:c_short,_:c_short,_:c_short,_:c_short) -> c_int;
    unsafe fn init_pair (_:c_short,_:c_short,_:c_short) -> c_int;
    unsafe fn innstr (_:char_p, _:c_int) -> c_int;
    unsafe fn insch (_:chtype) -> c_int;
    unsafe fn insdelln (_:c_int) -> c_int;
    unsafe fn insertln () -> c_int;
    unsafe fn insnstr (_:char_p, _:c_int) -> c_int;
    unsafe fn insstr (_:char_p) -> c_int;
    unsafe fn instr (_:char_p) -> c_int;
    unsafe fn intrflush (_:WINDOW_p,_:bool) -> c_int;
    unsafe fn isendwin () -> bool;
    unsafe fn is_linetouched (_:WINDOW_p,_:c_int) -> bool;
    unsafe fn is_wintouched (_:WINDOW_p) -> bool;
    unsafe fn keyname (_:c_int) -> *c_char;
    unsafe fn keypad (_:WINDOW_p, _:bool) -> c_int;
    unsafe fn killchar () -> char;
    unsafe fn leaveok (_:WINDOW_p,_:bool) -> c_int;
    unsafe fn longname () -> *c_char;
    unsafe fn meta (_:WINDOW_p,_:bool) -> c_int;
    unsafe fn move (_:c_int, _:c_int) -> c_int;
    unsafe fn mvaddch (_:c_int, _:c_int, _:chtype) -> c_int;
    unsafe fn mvaddchnstr (_:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
    unsafe fn mvaddchstr (_:c_int, _:c_int, _:chtype_p) -> c_int;
    unsafe fn mvaddnstr (_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    unsafe fn mvaddstr (_:c_int, _:c_int, _:char_p) -> c_int;
    unsafe fn mvchgat (_:c_int, _:c_int, _:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
    unsafe fn mvcur (_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;
    unsafe fn mvdelch (_:c_int, _:c_int) -> c_int;
    unsafe fn mvderwin (_:WINDOW_p, _:c_int, _:c_int) -> c_int;
    unsafe fn mvgetch (_:c_int, _:c_int) -> c_int;
    unsafe fn mvgetnstr (_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    unsafe fn mvgetstr (_:c_int, _:c_int, _:char_p) -> c_int;
    unsafe fn mvhline (_:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
    unsafe fn mvinch (_:c_int, _:c_int) -> chtype;
    unsafe fn mvinchnstr (_:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
    unsafe fn mvinchstr (_:c_int, _:c_int, _:chtype_p) -> c_int;
    unsafe fn mvinnstr (_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    unsafe fn mvinsch (_:c_int, _:c_int, _:chtype) -> c_int;
    unsafe fn mvinsnstr (_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    unsafe fn mvinsstr (_:c_int, _:c_int, _:char_p) -> c_int;
    unsafe fn mvinstr (_:c_int, _:c_int, _:char_p) -> c_int;
    //unsafe fn mvprintw (_:c_int, _:c_int, _:char_p) -> c_int;
    // unsafe fn mvscanw (_:c_int,_:c_int, _:char_p) -> c_int;
    unsafe fn mvvline (_:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
    unsafe fn mvwaddch (_:WINDOW_p, _:c_int, _:c_int, _:chtype) -> c_int;
    unsafe fn mvwaddchnstr (_:WINDOW_p, _:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
    unsafe fn mvwaddchstr (_:WINDOW_p, _:c_int, _:c_int, _:chtype_p) -> c_int;
    unsafe fn mvwaddnstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    unsafe fn mvwaddstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
    unsafe fn mvwchgat (_:WINDOW_p, _:c_int, _:c_int, _:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
    unsafe fn mvwdelch (_:WINDOW_p, _:c_int, _:c_int) -> c_int;
    unsafe fn mvwgetch (_:WINDOW_p, _:c_int, _:c_int) -> c_int;
    unsafe fn mvwgetnstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    unsafe fn mvwgetstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
    unsafe fn mvwhline (_:WINDOW_p, _:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
    unsafe fn mvwin (_:WINDOW_p,_:c_int,_:c_int) -> c_int;
    unsafe fn mvwinch (_:WINDOW_p, _:c_int, _:c_int) -> chtype;
    unsafe fn mvwinchnstr (_:WINDOW_p, _:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
    unsafe fn mvwinchstr (_:WINDOW_p, _:c_int, _:c_int, _:chtype_p) -> c_int;
    unsafe fn mvwinnstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    unsafe fn mvwinsch (_:WINDOW_p, _:c_int, _:c_int, _:chtype) -> c_int;
    unsafe fn mvwinsnstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    unsafe fn mvwinsstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
    unsafe fn mvwinstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
    //unsafe fn mvwprintw (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;

    // unsafe fn mvwscanw (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
    unsafe fn mvwvline (_:WINDOW_p, _:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
    unsafe fn napms (_:c_int) -> c_int;
    unsafe fn newpad (_:c_int,_:c_int) -> *WINDOW;
    unsafe fn newterm (_:char_p,_:FILE_p,_:FILE_p) -> *SCREEN;
    unsafe fn newwin (_:c_int,_:c_int,_:c_int,_:c_int) -> *WINDOW;
    unsafe fn nl () -> c_int;
    unsafe fn nocbreak () -> c_int;
    unsafe fn nodelay (_:WINDOW_p,_:bool) -> c_int;
    unsafe fn noecho () -> c_int;
    unsafe fn nonl () -> c_int;
    unsafe fn noqiflush ();
    unsafe fn noraw () -> c_int;
    unsafe fn notimeout (_:WINDOW_p,_:bool) -> c_int;
    unsafe fn overlay (_:WINDOW_p,_:WINDOW_p) -> c_int;
    unsafe fn overwrite (_:WINDOW_p,_:WINDOW_p) -> c_int;
    unsafe fn pair_content (_:c_short,_:short_p,_:short_p) -> c_int;
    unsafe fn PAIR_NUMBER (_:c_int) -> c_int;
    unsafe fn pechochar (_:WINDOW_p, _:chtype) -> c_int;
    unsafe fn pnoutrefresh (_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;
    unsafe fn prefresh (_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;

    //unsafe fn printw (_:char_p) -> c_int;
    unsafe fn putwin (_:WINDOW_p, _:FILE_p) -> c_int;
    unsafe fn qiflush ();
    unsafe fn raw () -> c_int;
    unsafe fn redrawwin (_:WINDOW_p) -> c_int;
    unsafe fn refresh () -> c_int;
    unsafe fn resetty () -> c_int;
    unsafe fn reset_prog_mode () -> c_int;
    unsafe fn reset_shell_mode () -> c_int;
    //unsafe fn ripoffline (_:c_int, extern unsafe fn f(WINDOW_p, c_int) -> c_int) -> c_int;
    unsafe fn ripoffline (_:c_int, f:*u8) -> c_int;
    unsafe fn savetty () -> c_int;
    // unsafe fn scanw (_:NCURSES_CONST char_p,...) -> c_int;
    unsafe fn scr_dump (_:char_p) -> c_int;
    unsafe fn scr_init (_:char_p) -> c_int;
    unsafe fn scrl (_:c_int) -> c_int;
    unsafe fn scroll (_:WINDOW_p) -> c_int;
    unsafe fn scrollok (_:WINDOW_p,_:bool) -> c_int;
    unsafe fn scr_restore (_:char_p) -> c_int;
    unsafe fn scr_set (_:char_p) -> c_int;
    unsafe fn setscrreg (_:c_int,_:c_int) -> c_int;
    unsafe fn set_term (_:SCREEN_p) -> SCREEN_p;
    unsafe fn slk_attroff (_:chtype) -> c_int;
    unsafe fn slk_attr_off (_:attr_t, _:void_p) -> c_int;
    unsafe fn slk_attron (_:chtype) -> c_int;
    unsafe fn slk_attr_on (_:attr_t,_:void_p) -> c_int;
    unsafe fn slk_attrset (_:chtype) -> c_int;
    unsafe fn slk_attr () -> attr_t;
    unsafe fn slk_attr_set (_:attr_t,_:c_short,_:void_p) -> c_int;
    unsafe fn slk_clear () -> c_int;
    unsafe fn slk_color (_:c_short) -> c_int;
    unsafe fn slk_init (_:c_int) -> c_int;
    unsafe fn slk_label (_:c_int) -> char_p; 
    unsafe fn slk_noutrefresh () -> c_int;
    unsafe fn slk_refresh () -> c_int;
    unsafe fn slk_restore () -> c_int;
    unsafe fn slk_set (_:c_int,_:char_p,_:c_int) -> c_int;
    unsafe fn slk_touch () -> c_int;
    unsafe fn standout () -> c_int;
    unsafe fn standend () -> c_int;
    unsafe fn start_color () -> c_int;
    unsafe fn subpad (_:WINDOW_p, _:c_int, _:c_int, _:c_int, _:c_int) -> WINDOW_p;
    unsafe fn subwin (_:WINDOW_p, _:c_int, _:c_int, _:c_int, _:c_int) -> WINDOW_p;
    unsafe fn syncok (_:WINDOW_p, _:bool) -> c_int;
    unsafe fn termattrs () -> chtype;
    unsafe fn termname () -> char_p;
    unsafe fn timeout (_:c_int);
    unsafe fn touchline (_:WINDOW_p, _:c_int, _:c_int) -> c_int;
    unsafe fn touchwin (_:WINDOW_p) -> c_int;
    unsafe fn typeahead (_:c_int) -> c_int;
    unsafe fn ungetch (_:c_int) -> c_int;
    unsafe fn untouchwin (_:WINDOW_p) -> c_int;
    unsafe fn use_env (_:bool);
    unsafe fn vidattr (_:chtype) -> c_int;
    // unsafe fn vidputs (_:chtype, extern unsafe fn f(c_int) -> c_int) -> c_int;
    unsafe fn vidputs (_:chtype, f:*u8) -> c_int;
    unsafe fn vline (_:chtype, _:c_int) -> c_int;
    //unsafe fn vwprintw (_:WINDOW_p, _:char_p, _:va_list) -> c_int;
    //unsafe fn vw_printw (_:WINDOW_p, _:char_p,_:va_list) -> c_int;
    // unsafe fn vwscanw (_:WINDOW_p, _:char_p, _:va_list) -> c_int;
    // unsafe fn vw_scanw (_:WINDOW_p, _:char_p, _:va_list) -> c_int;
    unsafe fn waddch (_:WINDOW_p, _:chtype) -> c_int;
    unsafe fn waddchnstr (_:WINDOW_p,_:chtype_p,_:c_int) -> c_int;
    unsafe fn waddchstr (_:WINDOW_p,_:chtype_p) -> c_int;
    unsafe fn waddnstr (_:WINDOW_p,_:char_p,_:c_int) -> c_int;
    unsafe fn waddstr (_:WINDOW_p,_:char_p) -> c_int;
    unsafe fn wattron (_:WINDOW_p, _:c_int) -> c_int;
    unsafe fn wattroff (_:WINDOW_p, _:c_int) -> c_int;
    unsafe fn wattrset (_:WINDOW_p, _:c_int) -> c_int;
    unsafe fn wattr_get (_:WINDOW_p, _:attr_t_p, _:short_p, _:void_p) -> c_int;
    unsafe fn wattr_on (_:WINDOW_p, _:attr_t, _:void_p) -> c_int;
    unsafe fn wattr_off (_:WINDOW_p, _:attr_t, _:void_p) -> c_int;
    unsafe fn wattr_set (_:WINDOW_p, _:attr_t, _:c_short, _:void_p) -> c_int;
    unsafe fn wbkgd (_:WINDOW_p, _:chtype) -> c_int;
    unsafe fn wbkgdset (_:WINDOW_p,_:chtype);
    unsafe fn wborder (_:WINDOW_p,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype) -> c_int;
    unsafe fn wchgat (_:WINDOW_p, _:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
    unsafe fn wclear (_:WINDOW_p) -> c_int;
    unsafe fn wclrtobot (_:WINDOW_p) -> c_int;
    unsafe fn wclrtoeol (_:WINDOW_p) -> c_int;
    unsafe fn wcolor_set (_:WINDOW_p,_:c_short,_:void_p) -> c_int;
    unsafe fn wcursyncup (_:WINDOW_p);
    unsafe fn wdelch (_:WINDOW_p) -> c_int;
    unsafe fn wdeleteln (_:WINDOW_p) -> c_int;
    unsafe fn wechochar (_:WINDOW_p, _:chtype) -> c_int;
    unsafe fn werase (_:WINDOW_p) -> c_int;
    unsafe fn wgetch (_:WINDOW_p) -> c_int;
    unsafe fn wgetnstr (_:WINDOW_p,_:char_p,_:c_int) -> c_int;
    unsafe fn wgetstr (_:WINDOW_p, _:char_p) -> c_int;
    unsafe fn whline (_:WINDOW_p, _:chtype, _:c_int) -> c_int;
    unsafe fn winch (_:WINDOW_p) -> chtype;
    unsafe fn winchnstr (_:WINDOW_p, _:chtype_p, _:c_int) -> c_int;
    unsafe fn winchstr (_:WINDOW_p, _:chtype_p) -> c_int;
    unsafe fn winnstr (_:WINDOW_p, _:char_p, _:c_int) -> c_int;
    unsafe fn winsch (_:WINDOW_p, _:chtype) -> c_int;
    unsafe fn winsdelln (_:WINDOW_p,_:c_int) -> c_int;
    unsafe fn winsertln (_:WINDOW_p) -> c_int;
    unsafe fn winsnstr (_:WINDOW_p, _:char_p,_:c_int) -> c_int;
    unsafe fn winsstr (_:WINDOW_p, _:char_p) -> c_int;
    unsafe fn winstr (_:WINDOW_p, _:char_p) -> c_int;
    unsafe fn wmove (_:WINDOW_p,_:c_int,_:c_int) -> c_int;
    unsafe fn wnoutrefresh (_:WINDOW_p) -> c_int;
    //unsafe fn wprintw (_:WINDOW_p, _:char_p) -> c_int;
    unsafe fn wredrawln (_:WINDOW_p,_:c_int,_:c_int) -> c_int;
    unsafe fn wrefresh (_:WINDOW_p) -> c_int;
    // unsafe fn wscanw (_:WINDOW_p, _:NCURSES_CONST char_p) -> c_int;
    unsafe fn wscrl (_:WINDOW_p,_:c_int) -> c_int;
    unsafe fn wsetscrreg (_:WINDOW_p,_:c_int,_:c_int) -> c_int;
    unsafe fn wstandout (_:WINDOW_p) -> c_int;
    unsafe fn wstandend (_:WINDOW_p) -> c_int;
    unsafe fn wsyncdown (_:WINDOW_p);
    unsafe fn wsyncup (_:WINDOW_p);
    unsafe fn wtimeout (_:WINDOW_p,_:c_int);
    unsafe fn wtouchln (_:WINDOW_p,_:c_int,_:c_int,_:c_int) -> c_int;
    unsafe fn wvline (_:WINDOW_p,_:chtype,_:c_int) -> c_int;

    /*
     * These are also declared in <term.h>:
     */
    unsafe fn tigetflag (_:char_p) -> c_int;
    unsafe fn tigetnum (_:char_p) -> c_int;
    unsafe fn tigetstr (_:char_p) -> *c_char;
    unsafe fn putp (_:char_p) -> c_int;

    unsafe fn tparm (_:char_p) -> *c_char;

/*
 * These functions are not in X/Open, but we use them in macro definitions:
 */
    unsafe fn getattrs (_:WINDOW_p) -> c_int;
    unsafe fn getcurx (_:WINDOW_p) -> c_int;
    unsafe fn getcury (_:WINDOW_p) -> c_int;
    unsafe fn getbegx (_:WINDOW_p) -> c_int;
    unsafe fn getbegy (_:WINDOW_p) -> c_int;
    unsafe fn getmaxx (_:WINDOW_p) -> c_int;
    unsafe fn getmaxy (_:WINDOW_p) -> c_int;
    unsafe fn getparx (_:WINDOW_p) -> c_int;
    unsafe fn getpary (_:WINDOW_p) -> c_int;
}

/*
 * These functions are extensions - not in X/Open Curses.
 */
extern {
/*
extern NCURSES_EXPORT(bool) is_term_resized (int, int);
extern NCURSES_EXPORT(char *) keybound (int, int);
extern NCURSES_EXPORT(const char *) curses_version (void);
extern NCURSES_EXPORT(int) assume_default_colors (int, int);
extern NCURSES_EXPORT(int) define_key (const char *, int);
extern NCURSES_EXPORT(int) key_defined (const char *);
extern NCURSES_EXPORT(int) keyok (int, bool);
extern NCURSES_EXPORT(int) resize_term (int, int);
extern NCURSES_EXPORT(int) resizeterm (int, int);
extern NCURSES_EXPORT(int) set_escdelay (int);
extern NCURSES_EXPORT(int) set_tabsize (int);
extern NCURSES_EXPORT(int) use_default_colors (void);
extern NCURSES_EXPORT(int) use_extended_names (bool);
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
unsafe fn wgetparent (_:WINDOW_p) -> WINDOW_p;
unsafe fn is_cleared (_:WINDOW_p) -> bool;
unsafe fn is_idcok (_:WINDOW_p) -> bool;
unsafe fn is_idlok (_:WINDOW_p) -> bool;
unsafe fn is_immedok (_:WINDOW_p) -> bool;
unsafe fn is_keypad (_:WINDOW_p) -> bool;
unsafe fn is_leaveok (_:WINDOW_p) -> bool;
unsafe fn is_nodelay (_:WINDOW_p) -> bool;
unsafe fn is_notimeout (_:WINDOW_p) -> bool;
unsafe fn is_scrollok (_:WINDOW_p) -> bool;
unsafe fn is_syncok (_:WINDOW_p) -> bool;
unsafe fn wgetscrreg (_:WINDOW_p, _:*c_int, _:*c_int) -> c_int;

}

/* attributes */

static NCURSES_ATTR_SHIFT  : uint =       8;
fn NCURSES_BITS(mask:uint,shift:uint) -> uint { ((mask) << ((shift) + NCURSES_ATTR_SHIFT) as i32) }

pub fn A_NORMAL() -> c_int		{ (1u - 1u) as c_int }
pub fn A_ATTRIBUTES() -> c_int	{ NCURSES_BITS(!(1u - 1u),0)  as c_int }
pub fn A_CHARTEXT() -> c_int	{ (NCURSES_BITS(1u,0) - 1u)  as c_int }
pub fn A_COLOR() -> c_int		{ NCURSES_BITS(((1u) << 8) - 1u,0)  as c_int }
pub fn A_STANDOUT() -> c_int	{ NCURSES_BITS(1u,8) as c_int }
pub fn A_UNDERLINE() -> c_int	{ NCURSES_BITS(1u,9)  as c_int }
pub fn A_REVERSE() -> c_int	{ NCURSES_BITS(1u,10)  as c_int }
pub fn A_BLINK() -> c_int		{ NCURSES_BITS(1u,11)  as c_int }
pub fn A_DIM() -> c_int		{ NCURSES_BITS(1u,12)  as c_int }
pub fn A_BOLD() -> c_int		{ NCURSES_BITS(1u,13)  as c_int }
pub fn A_ALTCHARSET() -> c_int	{ NCURSES_BITS(1u,14)  as c_int }
pub fn A_INVIS() -> c_int		{ NCURSES_BITS(1u,15)  as c_int }
pub fn A_PROTECT() -> c_int	{ NCURSES_BITS(1u,16)  as c_int }
pub fn A_HORIZONTAL() -> c_int	{ NCURSES_BITS(1u,17)  as c_int }
pub fn A_LEFT() -> c_int		{ NCURSES_BITS(1u,18)  as c_int }
pub fn A_LOW() -> c_int		{ NCURSES_BITS(1u,19) as c_int }
pub fn A_RIGHT() -> c_int		{ NCURSES_BITS(1u,20)  as c_int }
pub fn A_TOP() -> c_int		{ NCURSES_BITS(1u,21)  as c_int }
pub fn A_VERTICAL() -> c_int	{ NCURSES_BITS(1u,22)  as c_int }

/*
 * Most of the pseudo functions are macros that either provide compatibility
 * with older versions of curses, or provide inline functionality to improve
 * performance.
 */

/*
 * These pseudo functions are always implemented as macros:
 */

pub unsafe fn getyx(win:WINDOW_p,y: &mut c_int,x: &mut c_int) { *y = getcury(win); *x = getcurx(win); }
pub unsafe fn getbegyx(win:WINDOW_p,y: &mut c_int, x: &mut c_int) { *y = getbegy(win); *x = getbegx(win) }
pub unsafe fn getmaxyx(win:WINDOW_p,y: &mut c_int, x: &mut c_int) { *y = getmaxy(win); *x = getmaxx(win) }
pub unsafe fn getparyx(win:WINDOW_p,y: &mut c_int, x: &mut c_int) { *y = getpary(win); *x = getparx(win) }

pub unsafe fn getsyx(y:&mut c_int, x:&mut c_int) {
    if newscr != (0 as WINDOW_p) {
        if is_leaveok(newscr) {
            *x = -1 as c_int;
            *y = -1 as c_int;
        } else {
            getyx(newscr,(y), (x));
        }
    }
}


pub unsafe fn setsyx(y:&mut c_int,x:&mut c_int) {
    if newscr != (0 as WINDOW_p) {
        if *y == -1 && *x == -1 {
            leaveok(newscr, true);
        } else {
            leaveok(newscr, false);
            wmove(newscr, *y, *x);
        }
    }
}

//#define setterm(term)		setupterm(term, 1, (int *)0)

//#define fixterm()		reset_prog_mode()
//#define resetterm()		reset_shell_mode()
//#define saveterm()		def_prog_mode()
//#define crmode()		cbreak()
//#define nocrmode()		nocbreak()
//#define gettmode()

/* colors */
static COLOR_BLACK : c_int =	0;
static COLOR_RED : c_int =	1;
static COLOR_GREEN : c_int =    2;
static COLOR_YELLOW : c_int =    3;
static COLOR_BLUE : c_int =      4;
static COLOR_MAGENTA : c_int =   5;
static COLOR_CYAN : c_int =      6;
static COLOR_WHITE : c_int =      7;

/* line graphics */

extern {
    static acs_map : chtype_p;
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
unsafe fn ACS_S1() -> c_uchar	 { NCURSES_ACS('o') } /* scan line 1 */
unsafe fn ACS_S9() -> c_uchar	 { NCURSES_ACS('s') } /* scan line 9 */
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
unsafe fn ACS_S3() -> c_uchar	 { NCURSES_ACS('p') } /* scan line 3 */
unsafe fn ACS_S7() -> c_uchar	 { NCURSES_ACS('r') } /* scan line 7 */
unsafe fn ACS_LEQUAL() -> c_uchar { NCURSES_ACS('y') } /* less/equal */
unsafe fn ACS_GEQUAL() -> c_uchar { NCURSES_ACS('z') } /* greater/equal */
unsafe fn ACS_PI() -> c_uchar	 { NCURSES_ACS('{') } /* Pi */
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


static ERR:c_int =     (-1);

static OK:c_int =      (0);

/* values for the _flags member */
static _SUBWIN:c_int =         0x01;	/* is this a sub-window? */
static _ENDLINE:c_int =       0x02;	/* is the window flush right? */
static _FULLWIN:c_int  =      0x04;	/* is the window full-screen? */
static _SCROLLWIN:c_int =     0x08;	/* bottom edge is at screen bottom? */
static _ISPAD:c_int	  =      0x10;	/* is this window a pad? */
static _HASMOVED:c_int    =   0x20;	/* has cursor moved since last refresh? */
static _WRAPPED:c_int      =  0x40;	/* cursor was just wrappped */

/*
 * this value is used in the firstchar and lastchar fields to mark
 * unchanged lines
 */
static _NOCHANGE:c_int =       -1;

/*
 * this value is used in the oldindex field to mark lines created by insertions
 * and scrolls.
 */
static _NEWINDEX:c_int =	-1;

/*
 * Pseudo-character tokens outside ASCII range.  The curses wgetch() function
 * will return any given one of these only if the corresponding k- capability
 * is defined in your terminal's terminfo entry.
 *
 * Some keys (KEY_A1, etc) are arranged like this:
 *	a1     up    a3
 *	left   b2    right
 *	c1     down  c3
 *
 * A few key codes do not depend upon the terminfo entry.
 */
static KEY_CODE_YES:c_int =	0x100;		/* A wchar_t contains a key code */
static KEY_MIN:c_int =		0x101;		/* Minimum curses key */
static KEY_BREAK:c_int =	0x101;		/* Break key (unreliable) */
static KEY_SRESET:c_int =	0x158;		/* Soft (partial) reset (unreliable) */
static KEY_RESET:c_int =	0x159;		/* Reset or hard reset (unreliable) */
/*
 * These definitions were generated by /tmp/ncurses-27.roots/ncurses-27/ncurses/include/MKkey_defs.sh /tmp/ncurses-27.roots/ncurses-27/ncurses/include/Caps
 */
static KEY_DOWN:c_int =	0x102;		/* down-arrow key */
static KEY_UP:c_int=		0x103;		/* up-arrow key */
static KEY_LEFT:c_int=	0x104;		/* left-arrow key */
static KEY_RIGHT:c_int=	0x105;		/* right-arrow key */
static KEY_HOME:c_int=	0x106;		/* home key */
static KEY_BACKSPACE:c_int=	0x107;		/* backspace key */
static KEY_F0:c_int=		0x108;		/* Function keys.  Space for 64 */
static KEY_F1:c_int=		0x109;
static KEY_F2:c_int=		0x10a;
static KEY_F3:c_int=		0x10b;
static KEY_F4:c_int=		0x10c;
static KEY_F5:c_int=		0x10d;
static KEY_F6:c_int=		0x10e;
static KEY_F7:c_int=		0x10f;
static KEY_F8:c_int=		0x110;
static KEY_F9:c_int=		0x111;
static KEY_F10:c_int=		0x112;
static KEY_F11:c_int=		0x113;
static KEY_F12:c_int=		0x114;
static KEY_F13:c_int=		0x115;
static KEY_F14:c_int=		0x116;
static KEY_F15:c_int=		0x117;
//static KEY_F(n)	(KEY_F0+(n))	/* Value of function key n */

static KEY_DL:c_int=		0x148;		/* delete-line key */
static KEY_IL:c_int=		0x149;		/* insert-line key */
static KEY_DC:c_int=		0x150;		/* delete-character key */
static KEY_IC:c_int=		0x151;		/* insert-character key */
static KEY_EIC:c_int=		0x152;		/* sent by rmir or smir in insert mode */
static KEY_CLEAR:c_int=	0x14d;		/* clear-screen or erase key */
static KEY_EOS:c_int=		0x14e;		/* clear-to-end-of-screen key */
static KEY_EOL:c_int=		0x14f;		/* clear-to-end-of-line key */
static KEY_SF:c_int=		0x150;		/* scroll-forward key */
static KEY_SR:c_int=		0x151;		/* scroll-backward key */
static KEY_NPAGE:c_int=	0x152;		/* next-page key */
static KEY_PPAGE:c_int=	0x153;		/* previous-page key */
static KEY_STAB:c_int=	0x154;		/* set-tab key */
static KEY_CTAB:c_int=	0x155;		/* clear-tab key */
static KEY_CATAB:c_int=	0x156;		/* clear-all-tabs key */
static KEY_ENTER:c_int=	0x157;		/* enter/send key */
static KEY_PRINT:c_int=	0x15a;		/* print key */
static KEY_LL:c_int=		0x15b;		/* lower-left key (home down) */
static KEY_A1:c_int=		0x15c;		/* upper left of keypad */
static KEY_A3:c_int=		0x15d;		/* upper right of keypad */
static KEY_B2:c_int=		0x15e;		/* center of keypad */
static KEY_C1:c_int=		0x15f;		/* lower left of keypad */
static KEY_C3:c_int=		0x160;		/* lower right of keypad */
static KEY_BTAB:c_int=	0x161;		/* back-tab key */
static KEY_BEG:c_int=		0x162;		/* begin key */
static KEY_CANCEL:c_int=	0x163;		/* cancel key */
static KEY_CLOSE:c_int=	0x164;		/* close key */
static KEY_COMMAND:c_int=	0x165;		/* command key */
static KEY_COPY:c_int=	0x166;		/* copy key */
static KEY_CREATE:c_int=	0x167;		/* create key */
static KEY_END:c_int=		0x168;		/* end key */
static KEY_EXIT:c_int=	0x169;		/* exit key */
static KEY_FIND:c_int=	0x16a;		/* find key */
static KEY_HELP:c_int=	0x16b;		/* help key */
static KEY_MARK:c_int=	0x16c;		/* mark key */
static KEY_MESSAGE:c_int=	0x16d;		/* message key */
static KEY_MOVE:c_int=	0x16e;		/* move key */
static KEY_NEXT:c_int=	0x16f;		/* next key */
static KEY_OPEN:c_int=	0x170;		/* open key */
static KEY_OPTIONS:c_int=	0x171;		/* options key */
static KEY_PREVIOUS:c_int=	0x172;		/* previous key */
static KEY_REDO:c_int=	0x173;		/* redo key */
static KEY_REFERENCE:c_int=	0x174;		/* reference key */
static KEY_REFRESH:c_int=	0x175;		/* refresh key */
static KEY_REPLACE:c_int=	0x176;		/* replace key */
static KEY_RESTART:c_int=	0x177;		/* restart key */
static KEY_RESUME:c_int=	0x178;		/* resume key */
static KEY_SAVE:c_int=	0x179;		/* save key */
static KEY_SBEG:c_int=	0x17a;		/* shifted begin key */
static KEY_SCANCEL:c_int=	0x17b;		/* shifted cancel key */
static KEY_SCOMMAND:c_int=	0x17c;		/* shifted command key */
static KEY_SCOPY:c_int=	0x17d;		/* shifted copy key */
static KEY_SCREATE:c_int=	0x17e;		/* shifted create key */
static KEY_SDC	:c_int=	0x17f;		/* shifted delete-character key */
static KEY_SDL	:c_int=	0x180;		/* shifted delete-line key */
static KEY_SELECT:c_int=	0x181;		/* select key */
static KEY_SEND:c_int=	0x182;		/* shifted end key */
static KEY_SEOL:c_int=	0x183;		/* shifted clear-to-end-of-line key */
static KEY_SEXIT:c_int=	0x184;		/* shifted exit key */
static KEY_SFIND:c_int=	0x185;		/* shifted find key */
static KEY_SHELP:c_int=	0x186;		/* shifted help key */
static KEY_SHOME:c_int=	0x187;		/* shifted home key */
static KEY_SIC	:c_int=	0x188;		/* shifted insert-character key */
static KEY_SLEFT:c_int=	0x189;		/* shifted left-arrow key */
static KEY_SMESSAGE:c_int=	0x18a;		/* shifted message key */
static KEY_SMOVE:c_int=	0x18b;		/* shifted move key */
static KEY_SNEXT:c_int=	0x18c;		/* shifted next key */
static KEY_SOPTIONS:c_int=	0x18d;		/* shifted options key */
static KEY_SPREVIOUS:c_int=	0x18e;		/* shifted previous key */
static KEY_SPRINT:c_int=	0x18f;		/* shifted print key */
static KEY_SREDO:c_int=	0x190;		/* shifted redo key */
static KEY_SREPLACE:c_int=	0x191;		/* shifted replace key */
static KEY_SRIGHT:c_int=	0x192;		/* shifted right-arrow key */
static KEY_SRSUME:c_int=	0x193;		/* shifted resume key */
static KEY_SSAVE:c_int=	0x194;		/* shifted save key */
static KEY_SSUSPEND:c_int=	0x195;		/* shifted suspend key */
static KEY_SUNDO:c_int=	0x196;		/* shifted undo key */
static KEY_SUSPEND:c_int=	0x197;		/* suspend key */
static KEY_UNDO:c_int=	0x198;		/* undo key */
static KEY_MOUSE:c_int=	0x199;		/* Mouse event has occurred */
static KEY_RESIZE:c_int=	0x19a;		/* Terminal resize event */
static KEY_EVENT:c_int=	0x19b;		/* We were interrupted by an event */

static KEY_MAX	:c_int=	0x1ff;		/* Maximum key value is 0633 */

// FSK skipping functions involving cchar_t which has "interesting"
// structure and which are all guarded by #ifdef
// _XOPEN_SOURCE_EXTENDED

// FSK skipping mouse support material
/* $Id: curses.tail,v 1.16 2008/07/05 20:20:38 tom Exp $ */
