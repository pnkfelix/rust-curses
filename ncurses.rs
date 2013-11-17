#[link(name="ncurses",vers="5.7")];

#[feature(macro_rules)];

use std::ptr;
use nc = ncurses_core;
use ncurses_core::{WINDOW_p, SCREEN_p};

mod ncurses_core;

pub struct Context<'a> {
    window: Window,
    priv force_new: (),
    priv on_getch_err_do: input::getch_err_response<'a>,
}

impl<'a> Context<'a> {
    pub fn new() -> Context {
        Context{ window: initscr(),
                 force_new: (),
                 on_getch_err_do: input::Immed(input::Fail) }
    }
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) { endwin(); }
}

/// Describes a sub-rectangle of the screen (possibly in its
/// entirety), that you can write to and scroll independently of other
/// windows on the screen.
pub struct Window { priv ptr: WINDOW_p }

/// These are windows as large as the terminal screen (upper-left to
/// lower-right).  `stdscr` is one such screen; it is the default for
/// output.  There is a special screen, the "terminal screen", that
/// corresponds to ncurses' model of what the user sees now.  (It
/// might also correspond to `curscr`.
pub struct Screen { priv ptr: SCREEN_p }

impl Screen {
    unsafe fn wnd_ptr(&self) -> WINDOW_p {
        self.ptr as WINDOW_p
    }
}

macro_rules! fail_if_null {
    ($e:expr) => {
        {
            let result = $e;
            if ptr::is_null(result) {
                fail!();
            } else {
                result
            }
        }
    }
}

macro_rules! fail_if_err {
    ($e:expr) => {
        {
            let result = $e;
            if result == nc::ERR {
                fail!();
            } else {
                result
            }
        }
    }
}

macro_rules! wrap {
    ($i:ident) => {
        impl<'a> super::Context<'a> {
            #[fixed_stack_segment]
            pub fn $i(&mut self) {
                unsafe { fail_if_err!(nc::$i()); }
            }
        }
    }
}

pub mod attrs {
    use std::libc;
    use std::cast;
    use nc = ncurses_core;

    use super::colors;

    pub enum display {
        normal = nc::A_NORMAL, //        Normal display (no highlight)
        standout = nc::A_STANDOUT, //      Best highlighting mode of the terminal.
        underline = nc::A_UNDERLINE, //     Underlining
        reverse = nc::A_REVERSE, //       Reverse video
        blink = nc::A_BLINK, //         Blinking
        dim = nc::A_DIM, //           Half bright
        bold = nc::A_BOLD, //          Extra bright or bold
        protect = nc::A_PROTECT, //       Protected mode
        invis = nc::A_INVIS, //         Invisible or blank mode
        altcharset = nc::A_ALTCHARSET, //    Alternate character set
        // not an enum // A_CHARTEXT      Bit-mask to extract a character
    }

    static all_display_attrs: [display, ..10] = [
        normal, standout, underline, reverse,
        blink, dim, bold, protect,
        invis, altcharset
            ];

    pub enum attr {
        display(display),
        color_pair(super::colors::pair_num),   // Color-pair number n
    }

    pub struct attr_set {
        priv state: libc::c_int
    }

    fn encode_one(a: attr) -> libc::c_int {
        match a {
            display(d) => d as libc::c_int,
            color_pair(pair_num) => unsafe {
                nc::COLOR_PAIR(pair_num as libc::c_int)
            },
        }
    }

    fn encode_attrs(attrs: &[attr]) -> libc::c_int {
        let mut accum: libc::c_int = 0;
        for a in attrs.iter() { accum = accum | encode_one(*a) }
        accum
    }

    fn encode_set(attr_set: &attr_set) -> libc::c_int {
        attr_set.state
    }

    impl attr_set {
        pub fn encode(&self) -> libc::c_int {
            encode_set(self)
        }
        fn new(attrs: &[attr]) -> attr_set {
            attr_set { state: encode_attrs(attrs) }
        }
        fn contents(&self) -> ~[attr] {
            let mut accum = ~[];
            for &a in all_display_attrs.iter() {
                let i: i32 = unsafe { cast::transmute(a) };
                if 0 != (i as libc::c_int & self.state) {
                    accum.push(display(a));
                }
            }
            for i in range(0, colors::color_pair_count()) {
                let i = i as i16;
                let p = unsafe { nc::COLOR_PAIR(i as libc::c_int) };
                if 0 != (p & self.state) {
                    accum.push(color_pair(i))
                }
            }
            accum
        }
    }

    trait EncodesAttrs { fn encode(&self) -> libc::c_int; }

    impl<'a> EncodesAttrs for &'a [attr] {
        fn encode(&self) -> libc::c_int {
            encode_attrs(*self)
        }
    }

    impl<'a> super::Context<'a> {
        pub fn attrset(&mut self, attrs: &[attr]) {
            let i = encode_attrs(attrs);
            unsafe { fail_if_err!(nc::attrset(i)); }
        }
    }
}

pub mod mode {
    use nc = ncurses_core;
    use super::Context;

    // wrap!(cbreak)
    impl<'a> super::Context<'a> {
        #[fixed_stack_segment]
        pub fn set_cbreak(&mut self, val:bool) {
            if val {
                unsafe { fail_if_err!(nc::cbreak()); }
            } else {
                unsafe { fail_if_err!(nc::nocbreak()); }
            }
        }
    }

    // wrap!(nonl)
    impl<'a> super::Context<'a> {
        #[fixed_stack_segment]
        pub fn set_nl(&mut self, val:bool) {
            if val {
                unsafe { fail_if_err!(nc::nl()); }
            } else {
                unsafe { fail_if_err!(nc::nonl()); }
            }
        }
    }

    impl<'a> super::Context<'a> {
        #[fixed_stack_segment]
        pub fn set_echo(&mut self, val:bool) {
            if val {
                unsafe { fail_if_err!(nc::echo()); }
            } else {
                unsafe { fail_if_err!(nc::noecho()); }
            }
        }
    }

}

#[fixed_stack_segment]
fn endwin() {
    unsafe { fail_if_err!(nc::endwin()); }
}

fn initscr() -> Window {
    let result;
    unsafe { result = fail_if_null!(nc::initscr()); }
    Window { ptr: result }
}

pub mod chars {
    use nc = ncurses_core;
    use std::libc::{c_int, c_uint};
    use x = std::cast::transmute;
    use super::attrs;

    pub trait FromCInt { fn from_c_int(i:c_int) -> Self; }

    pub enum move_key {
        down = nc::KEY_DOWN , up = nc::KEY_UP, left = nc::KEY_LEFT, right = nc::KEY_RIGHT,
        home = nc::KEY_HOME, backspace = nc::KEY_BACKSPACE
    }

    impl move_key {
        pub fn covers(i:c_int) -> bool { nc::KEY_DOWN <= i && i <= nc::KEY_BACKSPACE }
    }
    impl FromCInt for move_key {
        fn from_c_int(i:c_int) -> move_key {
            let i = i as i16; assert!(move_key::covers(i as i32)); unsafe { x(i) }
        }
    }

    pub enum fcn_key {
        f0  = nc::KEY_F0,  f1  = nc::KEY_F1,  f2  = nc::KEY_F2,  f3  = nc::KEY_F3,
        f4  = nc::KEY_F4,  f5  = nc::KEY_F5,  f6  = nc::KEY_F6,  f7  = nc::KEY_F7,
        f8  = nc::KEY_F8,  f9  = nc::KEY_F9,  f10 = nc::KEY_F10, f11 = nc::KEY_F11,
        f12 = nc::KEY_F12, f13 = nc::KEY_F13, f14 = nc::KEY_F14, f15 = nc::KEY_F15,
    }

    impl fcn_key {
        pub fn covers(i:c_int) -> bool { nc::KEY_F0 <= i && i <= nc::KEY_F15 }
    }
    impl FromCInt for fcn_key {
        fn from_c_int(i:c_int) -> fcn_key {
            let i = i as i16; assert!(fcn_key::covers(i as i32)); unsafe { x(i) }
        }
    }

    pub enum reset_key {
        // all of these are listed as "unreliable" in the header
        break_ = nc::KEY_BREAK, sreset = nc::KEY_SRESET, reset = nc::KEY_RESET
    }

    impl reset_key {
        pub fn covers(i:c_int) -> bool {
            match i { nc::KEY_BREAK | nc::KEY_SRESET | nc::KEY_RESET => true, _ => false }
        }
    }
    impl FromCInt for reset_key {
        fn from_c_int(i:c_int) -> reset_key { 
            let i = i as i16; assert!(reset_key::covers(i as i32)); unsafe { x(i) }
        }
    }

    pub enum action_key {
        delete_line = nc::KEY_DL,
        insert_line = nc::KEY_IL,
        delete_char = nc::KEY_DC,
        insert_char = nc::KEY_IC,
        eic = nc::KEY_EIC, // sent by rmir or smir in insert mode
        clear = nc::KEY_CLEAR, //  /* clear-screen or erase key */
        eos = nc::KEY_EOS, //  /* clear-to-end-of-screen key */
        eol = nc::KEY_EOL, //  /* clear-to-end-of-line key */
        scroll_forward = nc::KEY_SF,
        scroll_backward = nc::KEY_SR,
        next_page = nc::KEY_NPAGE,
        prev_page = nc::KEY_PPAGE,
        set_tab = nc::KEY_STAB, // /* set-tab key */
        clear_tab = nc::KEY_CTAB, // /* clear-tab key */
        clear_all_tabs = nc::KEY_CATAB, // /* clear-all-tabs key */
        enter = nc::KEY_ENTER, // /* enter/send key */
        print = nc::KEY_PRINT, // /* print key */
        ll = nc::KEY_LL, // /* lower-left key (home down) */
        a1 = nc::KEY_A1, // /* upper left of keypad */
        a3 = nc::KEY_A3, // /* upper right of keypad */
        b2 = nc::KEY_B2, // /* center of keypad */
        c1 = nc::KEY_C1, // /* lower left of keypad */
        c3 = nc::KEY_C3, // /* lower right of keypad */
        back_tab = nc::KEY_BTAB,
        begin = nc::KEY_BEG,
        cancel = nc::KEY_CANCEL,
        close = nc::KEY_CLOSE,
        command = nc::KEY_COMMAND,
        copy = nc::KEY_COPY,
        create = nc::KEY_CREATE,
        end = nc::KEY_END,
        exit = nc::KEY_EXIT,
        find = nc::KEY_FIND,
        help = nc::KEY_HELP,
        mark = nc::KEY_MARK,
        message = nc::KEY_MESSAGE,
        move = nc::KEY_MOVE,
        next = nc::KEY_NEXT,
        open = nc::KEY_OPEN,
        options = nc::KEY_OPTIONS,
        previous = nc::KEY_PREVIOUS,
        redo = nc::KEY_REDO,
        reference = nc::KEY_REFERENCE,
        refresh = nc::KEY_REFRESH,
        replace = nc::KEY_REPLACE,
        restart = nc::KEY_RESTART,
        resume = nc::KEY_RESUME,
        save = nc::KEY_SAVE,
        suspend = nc::KEY_SUSPEND, // /* suspend key */
        undo = nc::KEY_UNDO, // /* undo key */
    }

    impl action_key {
        pub fn covers(i:c_int) -> bool { // ugh
            (nc::KEY_DL <= i && i <= nc::KEY_EIC)
                || (nc::KEY_CLEAR <= i && i <= nc::KEY_ENTER)
                || (nc::KEY_PRINT <= i && i <= nc::KEY_SAVE)
                || (nc::KEY_SUSPEND <= i && i <= nc::KEY_UNDO)
        }
    }
    impl FromCInt for action_key {
        fn from_c_int(i:c_int) -> action_key {
            let i = i as i16; assert!(action_key::covers(i as i32)); unsafe { x(i) }
        }
    }

    pub enum shifted_key {
        shifted_begin = nc::KEY_SBEG,
        shifted_cancel = nc::KEY_SCANCEL,
        shifted_command = nc::KEY_SCOMMAND,
        shifted_copy = nc::KEY_SCOPY,
        shifted_create = nc::KEY_SCREATE,
        shifted_delete_char = nc::KEY_SDC,
        shifted_delete_line = nc::KEY_SDL,
        shifted_select = nc::KEY_SELECT,
        shifted_end = nc::KEY_SEND,
        shifted_eol = nc::KEY_SEOL,
        shifted_exit = nc::KEY_SEXIT,
        shifted_find = nc::KEY_SFIND,
        shifted_help = nc::KEY_SHELP,
        shifted_home = nc::KEY_SHOME,
        shifted_insert_char = nc::KEY_SIC,
        shifted_left = nc::KEY_SLEFT,
        shifted_message = nc::KEY_SMESSAGE, // /* shifted message key */
        shifted_move = nc::KEY_SMOVE, // /* shifted move key */
        shifted_next = nc::KEY_SNEXT, // /* shifted next key */
        shifted_options = nc::KEY_SOPTIONS, // /* shifted options key */
        shifted_previous = nc::KEY_SPREVIOUS, // /* shifted previous key */
        shifted_print = nc::KEY_SPRINT, // /* shifted print key */
        shifted_redo = nc::KEY_SREDO, // /* shifted redo key */
        shifted_replace = nc::KEY_SREPLACE, // /* shifted replace key */
        shifted_right = nc::KEY_SRIGHT, // /* shifted right-arrow key */
        shifted_rsume = nc::KEY_SRSUME, // /* shifted resume key */
        shifted_save = nc::KEY_SSAVE, // /* shifted save key */
        shifted_suspend = nc::KEY_SSUSPEND, // /* shifted suspend key */
        shifted_undo = nc::KEY_SUNDO, // /* shifted undo key */
    }

    impl shifted_key {
        pub fn covers(i:c_int) -> bool { nc::KEY_SBEG <= i && i <= nc::KEY_SUNDO }
    }
    impl FromCInt for shifted_key {
        fn from_c_int(i:c_int) -> shifted_key { let i = i as i16; assert!(shifted_key::covers(i as i32)); unsafe { x(i) } }
    }

    pub enum event {
        mouse = nc::KEY_MOUSE, // /* Mouse event has occurred */
        resize = nc::KEY_RESIZE, // /* Terminal resize event */
        event = nc::KEY_EVENT, // /* We were interrupted by an event */
    }

    impl event {
        pub fn covers(i:c_int) -> bool {
            match i { nc::KEY_MOUSE | nc::KEY_RESIZE | nc::KEY_EVENT => true, _ => false }
        }
    }

    impl FromCInt for event {
        fn from_c_int(i:c_int) -> event { let i = i as i16; assert!(event::covers(i as i32)); unsafe { x(i) } }
    }

    pub enum raw_ch {
        ascii_ch(u8),
        move_ch(move_key),
        fcn_ch(fcn_key),
        reset_ch(reset_key),
        action_ch(action_key),
        shift_ch(shifted_key),
        event_ch(event)
    }

    pub enum ch {
        ch(raw_ch),
        ch_with_attr(raw_ch, attrs::attr_set)
    }

    fn encode_raw(c:raw_ch) -> nc::chtype {
        match c {
            ascii_ch(bk)  => bk as nc::chtype,
            move_ch(mk)   => mk as nc::chtype,
            fcn_ch(fk)    => fk as nc::chtype,
            reset_ch(rk)  => rk as nc::chtype,
            action_ch(ak) => ak as nc::chtype,
            shift_ch(sk)  => sk as nc::chtype,
            event_ch(ek)  => ek as nc::chtype,
        }
    }
    pub fn encode(c:ch) -> nc::chtype {
        match c {
            ch(rc)                  => encode_raw(rc),
            ch_with_attr(rc, attrs) => encode_raw(rc) | (attrs.encode() as c_uint)
        }
    }
}

pub mod colors {
    use nc = ncurses_core;
    use std::libc;

    pub type pair_num = libc::c_short;

    pub enum color {
        Black = nc::COLOR_BLACK,
        Red = nc::COLOR_RED,
        Green = nc::COLOR_GREEN,
        Yellow = nc::COLOR_YELLOW,
        Blue = nc::COLOR_BLUE,
        Magenta = nc::COLOR_MAGENTA,
        Cyan = nc::COLOR_CYAN,
        White = nc::COLOR_WHITE,
    }

    impl<'a> super::Context<'a> {
        #[fixed_stack_segment]
        pub fn has_colors(&self) -> bool {
            unsafe { nc::has_colors() != 0 }
        }

        #[fixed_stack_segment]
        pub fn init_pair(&mut self, pair: pair_num, fg: color, bg: color) {
            assert!((pair as i32) < color_pair_count());
            unsafe { fail_if_err!(nc::init_pair(pair, fg as i16, bg as i16)); }
        }

        #[fixed_stack_segment]
        pub fn start(&mut self) { self.start_color(); }
    }
    wrap!(start_color)

    pub fn color_pair_count() -> libc::c_int {
        nc::COLOR_PAIRS
    }
}

pub mod input {
    use nc = ncurses_core;
    use std::libc::c_int;
    use super::chars::{raw_ch,ascii_ch,
                       move_ch,fcn_ch,reset_ch,action_ch,shift_ch,event_ch,
                       move_key,fcn_key,reset_key,action_key,shifted_key,event,
                       FromCInt};

    pub enum getch_err_act { Fail, Retry, Return(raw_ch) }

    pub enum getch_err_response<'a> {
        Immed(getch_err_act),
        Delay(&'static fn:Send() -> getch_err_act),
    }

    pub fn getch_result_to_ch(result: c_int) -> raw_ch {
        if 0 <= result && result < 0x100        { ascii_ch(result as u8) }
            else if move_key::covers(result)    { move_ch(FromCInt::from_c_int(result)) }
            else if fcn_key::covers(result)     { fcn_ch(FromCInt::from_c_int(result)) }
            else if reset_key::covers(result)   { reset_ch(FromCInt::from_c_int(result)) }
            else if action_key::covers(result)  { action_ch(FromCInt::from_c_int(result)) }
            else if shifted_key::covers(result) { shift_ch(FromCInt::from_c_int(result)) }
            else if event::covers(result)       { event_ch(FromCInt::from_c_int(result)) }
            else { fail!("unrecognized result from getch: {}", result);
        }
    }

    impl<'a> super::Context<'a> {
        pub fn on_getch_err(&mut self, value: getch_err_response<'a>) {
            self.on_getch_err_do = value;
        }
        pub fn getch(&mut self) -> raw_ch {
            let mut result : c_int;
            'getch: loop {
                result = unsafe { nc::getch() };
                if result == nc::ERR {
                    let act = match &self.on_getch_err_do {
                        &Immed(ref act) => *act,
                        &Delay(ref call) => (*call)(),
                    };
                    match act {
                        Fail       => fail!("ncurses::getch"),
                        Retry      => continue 'getch,
                        Return(c)  => return c,
                    }
                } else {
                    return getch_result_to_ch(result);
                }
            }
        }
    }
}

pub mod moves {
    use nc = ncurses_core;
    use std::libc::c_int;

    impl<'a> super::Context<'a> {
        fn move(&mut self, y: c_int, x: c_int) {
            unsafe { fail_if_err!(nc::move(y, x)); }
        }
    }

    impl super::Window {
        fn move(&mut self, y: c_int, x: c_int) {
            unsafe { fail_if_err!(nc::wmove(self.ptr, y, x)); }
        }
    }
}

pub mod output {
    use nc = ncurses_core;
    use std::libc::c_int;
    use super::chars::{ch,encode};

    impl<'a> super::Context<'a> {
        pub fn addch(&mut self, c: ch) {
            let c = encode(c);
            unsafe { fail_if_err!(nc::addch(c)); }
        }
        pub fn mvaddch(&mut self, y: c_int, x: c_int, c: ch) {
            let c = encode(c);
            unsafe { fail_if_err!(nc::mvaddch(y, x, c)); }
        }
        pub fn echo(&mut self, c: ch) {
            let c = encode(c);
            unsafe { fail_if_err!(nc::echochar(c)); }
        }
        pub fn refresh(&mut self) {
            unsafe { fail_if_err!(nc::refresh()); }
        }
    }

    impl super::Window {
        pub fn addch(&mut self, c: ch) {
            let c = encode(c);
            unsafe { fail_if_err!(nc::waddch(self.ptr, c)); }
        }
        pub fn mvaddch(&mut self, y: c_int, x: c_int, c: ch) {
            let c = encode(c);
            unsafe { fail_if_err!(nc::mvwaddch(self.ptr, y, x, c)); }
        }
        pub fn echo(&mut self, c: ch) {
            let c = encode(c);
            unsafe { fail_if_err!(nc::wechochar(self.ptr, c)); }
        }
        pub fn refresh(&mut self) {
            unsafe { fail_if_err!(nc::wrefresh(self.ptr)); }
        }
    }
    impl super::Window {
        pub fn touch(&mut self) {
            unsafe { fail_if_err!(nc::touchwin(self.ptr)); }
        }
        pub fn untouch(&mut self) {
            unsafe { fail_if_err!(nc::untouchwin(self.ptr)); }
        }
    }
}

impl<'a> Context<'a> {
    #[fixed_stack_segment]
    pub fn keypad(&mut self, s:&mut Screen, enabled: bool) {
        unsafe {
            fail_if_err!(nc::keypad(s.wnd_ptr(), enabled as nc::NCURSES_BOOL));
        }
    }

    pub fn stdscr(&self) -> Screen {
        Screen { ptr: nc::stdscr as SCREEN_p}
    }
}
