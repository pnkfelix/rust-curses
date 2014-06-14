#![feature(macro_rules)]
#![feature(link_args)]

#![crate_id="ncurses#5.7"]

extern crate libc;

use std::mem;
use std::ptr;
use std::vec;
use nc = ncurses_core;
use ncurses_core::{WINDOW_p, SCREEN_p};

pub fn lines() -> libc::c_int { nc::LINES }
pub fn cols() -> libc::c_int { nc::COLS }

mod ncurses_core;

pub struct Context<'a> {
    on_getch_err_do: input::get_err_response<'a, chars::raw_ch>,
    on_getstr_err_do: input::get_err_response<'a, String>,
}

/// Describes a sub-rectangle of the screen (possibly in its
/// entirety), that you can write to and scroll independently of other
/// windows on the screen.
pub struct Window<'a> {
    ptr: WINDOW_p,
    ctxt: &'a Context<'a>,
}

// "Screens" are windows as large as the terminal screen (upper-left to
// lower-right).  `stdscr` is one such screen; it is the default for
// output.  There is a special screen, the "terminal screen", that
// corresponds to ncurses' model of what the user sees now.  (It
// might also correspond to `curscr`.
//
// However, the ncurses-intro's notion of "screen" is not the same as
// the ncurses SCREEN type, and thus should be conflated with the
// below.  So this text is no longer a doc comment; if I figure out
// why the ncurses-intro was introducing this distinction in the first
// place, then maybe I will find place for this to be revised as doc.


macro_rules! fail_if_null {
    ($e:expr) => {
        {
            let result = $e;
            if result.is_null() {
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
            pub fn $i(&mut self) {
                unsafe { fail_if_err!(nc::$i()); }
            }
        }
    }
}

pub mod attrs {
    use libc;
    use std::mem;
    use std::ptr;
    use nc = ncurses_core;
    use WIN_p = ncurses_core::WINDOW_p;
    use libc::c_int;

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
        color_pair(colors::pair_num),   // Color-pair number n
    }

    pub struct attr_set {
        state: ::libc::c_int
    }

    impl attr_set {
        pub fn from_chtype(bits: ::libc::c_int) -> attr_set {
            if (bits & !nc::A_ATTRIBUTES) != 0 { fail!() }
            attr_set { state: bits }
        }
    }

    fn encode_one(a: attr) -> ::libc::c_int {
        match a {
            display(d) => d as ::libc::c_int,
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
        fn contents(&self) -> Vec<attr> {
            let mut accum = Vec::new();
            for &a in all_display_attrs.iter() {
                let i: i32 = unsafe { mem::transmute(a) };
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

    pub trait EncodesAttrs { fn encode(&self) -> libc::c_int; }

    impl<'a> EncodesAttrs for &'a [attr] {
        fn encode(&self) -> libc::c_int {
            encode_attrs(*self)
        }
    }

    impl<'a> EncodesAttrs for Vec<attr> {
        fn encode(&self) -> libc::c_int {
            encode_attrs(self.as_slice())
        }
    }

    impl EncodesAttrs for attr {
        fn encode(&self) -> libc::c_int {
            encode_one(*self)
        }
    }

    unsafe fn attroff (x:nc::attr_t) -> libc::c_int { nc::attroff(x) }
    unsafe fn attron  (x:nc::attr_t) -> libc::c_int { nc::attron(x)  }
    unsafe fn attrset (x:nc::attr_t) -> libc::c_int { nc::attrset(x) }
    unsafe fn wattroff (w:WIN_p, x:nc::attr_t) -> c_int { nc::wattroff(w, x) }
    unsafe fn wattron  (w:WIN_p, x:nc::attr_t) -> c_int { nc::wattron(w, x)  }
    unsafe fn wattrset (w:WIN_p, x:nc::attr_t) -> c_int { nc::wattrset(w, x) }

    pub trait AttrSet {
        fn attroff<A:EncodesAttrs>(&mut self, attrs: A);
        fn attron<A:EncodesAttrs>(&mut self, attrs: A);
        fn attrset<A:EncodesAttrs>(&mut self, attrs: A);
    }

    impl<'a> AttrSet for super::Context<'a> {
        fn attroff<A:EncodesAttrs>(&mut self, attrs: A) {
            let i = attrs.encode();
            unsafe { fail_if_err!(attroff(i)); }
        }
        fn attron<A:EncodesAttrs>(&mut self, attrs: A) {
            let i = attrs.encode();
            unsafe { fail_if_err!(attron(i)); }
        }
        fn attrset<A:EncodesAttrs>(&mut self, attrs: A) {
            let i = attrs.encode();
            unsafe { fail_if_err!(attrset(i)); }
        }
    }

    impl<'a> AttrSet for super::Window<'a> {
        fn attroff<A:EncodesAttrs>(&mut self, attrs: A) {
            let i = attrs.encode();
            unsafe { fail_if_err!(nc::wattroff(self.ptr, i)); }
        }
        fn attron<A:EncodesAttrs>(&mut self, attrs: A) {
            let i = attrs.encode();
            unsafe { fail_if_err!(nc::wattron(self.ptr, i)); }
        }
        fn attrset<A:EncodesAttrs>(&mut self, attrs: A) {
            let i = attrs.encode();
            unsafe { fail_if_err!(nc::wattrset(self.ptr, i)); }
        }
    }

    trait AttrGet {
        fn attr_get(&self) -> (attr_set, colors::pair_num);
    }

    impl<'a> AttrGet for super::Context<'a> {
        fn attr_get(&self) -> (attr_set, colors::pair_num) {
            let mut a = attr_set{ state: 0 };
            let mut p : colors::pair_num = 0;
            unsafe {
                fail_if_err!(nc::attr_get(&mut a.state as *mut libc::c_int,
                                          &mut p as *mut libc::c_short,
                                          ptr::null()));
            }
            (a, p)
        }
    }

    impl<'a> super::Window<'a> {
        fn attr_get(&self) -> (attr_set, colors::pair_num) {
            let mut a = attr_set{ state: 0 };
            let mut p : colors::pair_num = 0;
            unsafe {
                fail_if_err!(nc::wattr_get(self.ptr,
                                           &mut a.state as *mut libc::c_int,
                                           &mut p as *mut libc::c_short,
                                           ptr::null()));
            }
            (a, p)
        }
    }

    // impl<'a> super::Context<'a> { }
    // impl<'a> super::Window<'a> { }
}

pub mod mode {
    use nc = ncurses_core;
    use super::Context;

    // wrap!(cbreak)
    impl<'a> super::Context<'a> {
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
        pub fn set_nl(&mut self, val:bool) {
            if val {
                unsafe { fail_if_err!(nc::nl()); }
            } else {
                unsafe { fail_if_err!(nc::nonl()); }
            }
        }
    }

    impl<'a> super::Context<'a> {
        pub fn set_echo(&mut self, val:bool) {
            if val {
                unsafe { fail_if_err!(nc::echo()); }
            } else {
                unsafe { fail_if_err!(nc::noecho()); }
            }
        }
    }

    impl<'a> super::Context<'a> {
        pub fn set_raw(&mut self, val:bool) {
            if val {
                unsafe { fail_if_err!(nc::raw()); }
            } else {
                unsafe { fail_if_err!(nc::noraw()); }
            }
        }
    }

}

fn endwin() {
    unsafe { fail_if_err!(nc::endwin()); }
}

impl<'a> Context<'a> {
    pub fn new() -> Context {
        let c = Context {
            on_getch_err_do: input::Immed(input::Fail),
            on_getstr_err_do: input::Immed(input::Fail),
        };
        c.initscr();
        c
    }
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) { endwin(); }
}

impl<'a> Context<'a> {
    fn initscr(&'a self) -> Window<'a> {
        let result;
        unsafe { result = fail_if_null!(nc::initscr()); }
        Window { ptr: result, ctxt: self }
    }
}

pub mod chars {
    use nc = ncurses_core;
    use libc::{c_int, c_uint, c_char};
    use x = std::mem::transmute;
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
        ascii_ch(c_char),
        wide_ch(char),
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

    pub trait HasChEncoding { fn encode(&self) -> nc::chtype; }

    impl HasChEncoding for raw_ch {
        fn encode(&self) -> nc::chtype { encode_raw(*self) }
    }

    impl HasChEncoding for ch {
        fn encode(&self) -> nc::chtype { encode(*self) }
    }

    impl HasChEncoding for nc::chtype {
        fn encode(&self) -> nc::chtype { *self }
    }

    fn encode_raw(c:raw_ch) -> nc::chtype {
        match c {
            ascii_ch(bk)  => bk as nc::chtype,
            wide_ch(_)    => fail!("cannot encode (wide) char as nc::chtype"),
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
    pub fn decode(c:nc::chtype) -> ch {
        let c = c as c_int;
        // This might fail; I do not yet know how best to extract attrs.
        let chartext = c & nc::A_CHARTEXT;
        let attributes = c & nc::A_ATTRIBUTES;
        let raw = super::input::getch_result_to_ch(chartext);
        if attributes == 0 {
            ch(raw)
        } else {
            ch_with_attr(raw, attrs::attr_set::from_chtype(attributes))
        }
    }
}

pub mod colors {
    use nc = ncurses_core;

    pub type pair_num = ::libc::c_short;

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
        pub fn has_colors(&self) -> bool {
            unsafe { nc::has_colors() != 0 }
        }

        pub fn init_pair(&mut self, pair: pair_num, fg: color, bg: color) {
            assert!((pair as i32) < color_pair_count());
            unsafe { fail_if_err!(nc::init_pair(pair, fg as i16, bg as i16)); }
        }

        pub fn start(&mut self) { self.start_color(); }
    }
    wrap!(start_color)

    pub fn color_pair_count() -> ::libc::c_int {
        nc::COLOR_PAIRS
    }
}

trait VecAsBuf<T> {
    fn as_imm_buf<A>(&self, k:|ptr: *T, len: uint| -> A) -> A;
}

trait VecAsMutBuf<T> : VecAsBuf<T> {
    fn as_mut_buf<A>(&mut self, K:|ptr: *mut T, len: uint| -> A) -> A;
}

impl<'a,T> VecAsBuf<T> for &'a [T] {
    fn as_imm_buf<A>(&self, k:|ptr: *T, len: uint| -> A) -> A {
        let len = self.len();
        let ptr = self.as_ptr();
        k(ptr, len)
    }
}

impl<'a,T> VecAsBuf<T> for &'a mut [T] {
    fn as_imm_buf<A>(&self, k:|ptr: *T, len: uint| -> A) -> A {
        let len = self.len();
        let ptr = self.as_ptr() as *T;
        k(ptr, len)
    }
}

impl<'a,T> VecAsMutBuf<T> for &'a mut [T] {
    fn as_mut_buf<A>(&mut self, k:|ptr: *mut T, len: uint| -> A) -> A {
        let len = self.len();
        let ptr = self.as_mut_ptr() as *mut T;
        k(ptr, len)
    }
}

impl<T> VecAsBuf<T> for Vec<T> {
    fn as_imm_buf<A>(&self, k:|ptr: *T, len: uint| -> A) -> A {
        self.as_slice().as_imm_buf(k)
    }
}

impl<T> VecAsMutBuf<T> for Vec<T> {
    fn as_mut_buf<A>(&mut self, k:|ptr: *mut T, len: uint| -> A) -> A {
        self.as_mut_slice().as_mut_buf(k)
    }
}

pub mod input {
    use nc = ncurses_core;
    use libc::{c_int,c_char};
    use super::chars::{raw_ch,ascii_ch,wide_ch,
                       move_ch,fcn_ch,reset_ch,action_ch,shift_ch,event_ch,
                       move_key,fcn_key,reset_key,action_key,shifted_key,event,
                       FromCInt};
    use super::VecAsMutBuf;

    #[deriving(Clone)]
    pub enum get_err_act<RET> { Fail, Retry, Return(RET) }

    pub enum get_err_response<'a, RET> {
        Immed(get_err_act<RET>),
        Delay(fn() -> get_err_act<RET>),
    }

    pub fn getch_result_to_ch(result: c_int) -> raw_ch {
        if 0 <= result && result < 0x80        { ascii_ch(result as i8) }
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
        pub fn on_getch_err(&mut self, value: get_err_response<'a, raw_ch>) {
            self.on_getch_err_do = value;
        }
        pub fn on_getstr_err(&mut self, value: get_err_response<'a, String>) {
            self.on_getstr_err_do = value;
        }
    }

    pub trait GetCh {
        fn getch(&mut self) -> raw_ch;
    }

    impl<'a> GetCh for super::Context<'a> {
        fn getch(&mut self) -> raw_ch {
            use std::char;
            let mut result : nc::wint_t = 0;
            'getch: loop {
                let r = unsafe { nc::get_wch(&mut result as *mut nc::wint_t) };
                match (r, result, char::from_u32(result as u32)) {

                    (nc::KEY_CODE_YES, _, _) => return getch_result_to_ch(result),

                    (nc::OK, _, Some(c)) => {
                        let ret = if c.is_ascii() { ascii_ch(c as c_char) } else { wide_ch(c) };
                        return ret;
                    }

                    _ => {
                        let act = match &self.on_getch_err_do {
                            &Immed(ref act) => *act,
                            &Delay(ref call) => (*call)(),
                        };
                        match act {
                            Fail       => fail!("ncurses::get_wch"),
                            Retry      => continue 'getch,
                            Return(c)  => return c,
                        }
                    }
                }
            }
        }
    }

    impl<'a> GetCh for super::Window<'a> {
        fn getch(&mut self) -> raw_ch {
            use std::char;
            let mut result : nc::wint_t = 0;
            'getch: loop {
                let r = unsafe { nc::wget_wch(self.ptr, &mut result as *mut nc::wint_t) };
                match (r, result, char::from_u32(result as u32)) {

                    (nc::KEY_CODE_YES, _, _) => return getch_result_to_ch(result),

                    (nc::OK, _, Some(c)) => return wide_ch(c),

                    _ => {
                        let act = match &self.ctxt.on_getch_err_do {
                            &Immed(ref act) => *act,
                            &Delay(ref call) => (*call)(),
                        };
                        match act {
                            Fail       => fail!("ncurses::wget_wch"),
                            Retry      => continue 'getch,
                            Return(c)  => return c,
                        }
                    }
                }
            }
        }
    }

    trait GetAscii {
        fn getascii(&mut self, bytes: &mut [c_char]);
        fn mvgetascii(&mut self, y: c_int, x: c_int, bytes: &mut [c_char]);
        fn getnascii(&mut self, bytes: &mut [c_char], n: uint);
    }

    impl<'a> GetAscii for super::Context<'a> {
        // (deliberately not using unsafe nc::getstr fcn.)

        fn getascii(&mut self, mut bytes: &mut [c_char]) {
            bytes.as_mut_buf(|ptr, len| {
                    let len = len as i32;
                    if len < 0 { fail!(); }
                    unsafe { fail_if_err!(nc::getnstr(ptr, len)); }
                });
        }

        fn mvgetascii(&mut self, y: c_int, x: c_int, mut bytes: &mut [c_char]) {
            bytes.as_mut_buf(|ptr, len| {
                    let len = len as i32;
                    if len < 0 { fail!(); }
                    unsafe { fail_if_err!(nc::mvgetnstr(y, x, ptr, len)); }
                });
        }

        fn getnascii(&mut self, mut bytes: &mut [c_char], n: uint) {
            bytes.as_mut_buf(|ptr, len| {
                    if n > len { fail!(); }
                    let n = n as i32;
                    if n < 0 { fail!(); }
                    unsafe { fail_if_err!(nc::getnstr(ptr, n)); }
                });
        }
    }

    impl<'a> GetAscii for super::Window<'a> {

        fn getascii(&mut self, mut bytes: &mut [c_char]) {
            bytes.as_mut_buf(|ptr, len| {
                    let len = len as i32;
                    if len < 0 { fail!(); }
                    unsafe { fail_if_err!(nc::wgetnstr(self.ptr, ptr, len)); }
                });
        }

        fn mvgetascii(&mut self, y: c_int, x: c_int, mut bytes: &mut [c_char]) {
            bytes.as_mut_buf(|ptr, len| {
                    let len = len as i32;
                    if len < 0 { fail!(); }
                    unsafe { fail_if_err!(nc::mvwgetnstr(self.ptr, y, x, ptr, len)); }
                });
        }

        fn getnascii(&mut self, mut bytes: &mut [c_char], n: uint) {
            bytes.as_mut_buf(|ptr, len| {
                    if n > len { fail!(); }
                    let n = n as i32;
                    if n < 0 { fail!(); }
                    unsafe { fail_if_err!(nc::wgetnstr(self.ptr, ptr, n)); }
                });
        }
    }

    trait GetStr {
        fn getstr(&mut self) -> String;
        fn getnstr(&mut self, n:uint) -> String;
        fn mvgetstr(&mut self, y: c_int, x: c_int) -> String;
        fn mvgetnstr(&mut self, y: c_int, x: c_int, n:uint) -> String;
    }

    impl<'a> GetStr for super::Context<'a> {
        fn getstr(&mut self) -> String {
            use super::HasYX;
            let (y,x) = self.getyx();
            self.mvgetnstr(y, x, nc::COLS as uint)
        }
        fn getnstr(&mut self, n:uint) -> String {
            use super::HasYX;
            let (y,x) = self.getyx();
            self.mvgetnstr(y, x, n)
        }
        fn mvgetstr(&mut self, y: c_int, x: c_int) -> String {
            self.mvgetnstr(y, x, nc::COLS as uint)
        }
        fn mvgetnstr(&mut self, y: c_int, x: c_int, n:uint) -> String {
            use std::{char,str,vec};
            let mut result : Vec<nc::wint_t> = Vec::from_elem(n as uint, 0i32);
            'getstr: loop {
                let r = unsafe {
                    result.as_mut_buf(|p, len| {
                            if n > len { fail!(); }
                            let n = n as c_int;
                            if n < 0 { fail!(); }
                            nc::mvgetn_wstr(y, x, p, n)
                        })};

                let mut saw_nonchar = false;
                let chars : Vec<u8> = result.iter().map(|&i| match char::from_u32(i as u32) {
                        None    => { saw_nonchar = true; '_' as u8 }
                        Some(c) => c as u8,
                    }).collect();

                match r {
                    nc::OK if !saw_nonchar => return String::from_utf8(chars).unwrap(),
                    _ => {
                        let act = match &self.on_getstr_err_do {
                            &Immed(ref act) => act.clone(),
                            &Delay(ref call) => (*call)(),
                        };
                        match act {
                            Fail       => fail!("ncurses::wget_wch"),
                            Retry      => continue 'getstr,
                            Return(c)  => return c,
                        }
                    }
                }
            }
        }
    }
}

pub mod moves {
    use nc = ncurses_core;
    use libc::c_int;

    pub trait Move {
        fn move(&mut self, y: c_int, x: c_int);
    }
    impl<'a> Move for super::Context<'a> {
        fn move(&mut self, y: c_int, x: c_int) {
            unsafe { fail_if_err!(nc::move(y, x)); }
        }
    }

    impl<'a> Move for super::Window<'a> {
        fn move(&mut self, y: c_int, x: c_int) {
            unsafe { fail_if_err!(nc::wmove(self.ptr, y, x)); }
        }
    }
}

pub mod background {
    use nc = ncurses_core;
    use libc::c_int;
    use super::chars;
    use ToCh = super::chars::HasChEncoding;
    use WIN_p = ncurses_core::WINDOW_p;

    unsafe fn bkgdset (c:nc::chtype) { nc::bkgdset(c) }
    unsafe fn wbkgdset (w:WIN_p, c:nc::chtype) { nc::wbkgdset(w, c) }
    unsafe fn bkgd (c:nc::chtype) -> c_int { nc::bkgd(c) }
    unsafe fn wbkgd (w:WIN_p, c:nc::chtype) -> c_int { nc::wbkgd(w, c) }

    pub trait Background {
        fn bkgd<E:ToCh>(&mut self, c:E);
        fn bkgdset<E:ToCh>(&mut self, c:E);
    }
    impl<'a> Background for super::Context<'a> {
        fn bkgd<E:ToCh>(&mut self, c:E) {
            unsafe { fail_if_err!(bkgd(c.encode())); }
        }

        fn bkgdset<E:ToCh>(&mut self, c:E) {
            unsafe { bkgdset(c.encode()) }
        }
    }
    impl<'a> Background for super::Window<'a> {
        fn bkgd<E:ToCh>(&mut self, c:E) {
            unsafe { fail_if_err!(wbkgd(self.ptr, c.encode())); }
        }
        fn bkgdset<E:ToCh>(&mut self, c:E) {
            unsafe { wbkgdset(self.ptr, c.encode()) }
        }
    }
    impl<'a> super::Window<'a> {
        fn getbkgd(&self) -> chars::ch {
            let c = unsafe { nc::getbkgd(self.ptr) };
            chars::decode(c)
        }
    }
}

pub mod output {
    use nc = ncurses_core;
    use libc::c_int;
    use super::chars::{ch,encode};
    use super::VecAsBuf;
    use ToCh = super::chars::HasChEncoding;

    fn addch(c: nc::chtype) {
        unsafe { fail_if_err!(nc::addch(c)); }
    }
    fn mvaddch(y: c_int, x: c_int, c: nc::chtype) {
        unsafe { fail_if_err!(nc::mvaddch(y, x, c)); }
    }
    fn waddch(p:nc::WINDOW_p, c: nc::chtype) {
        unsafe { fail_if_err!(nc::waddch(p, c)); }
    }
    fn mvwaddch(p:nc::WINDOW_p, y: c_int, x: c_int, c: nc::chtype) {
        unsafe { fail_if_err!(nc::mvwaddch(p, y, x, c)); }
    }

    trait AddCh {
        fn addch<E:ToCh>(&mut self, c: E);
        fn mvaddch<E:ToCh>(&mut self, y: c_int, x: c_int, c: E);
    }
    impl<'a> AddCh for super::Context<'a> {
        fn addch<E:ToCh>(&mut self, c: E) {
            let c = c.encode(); addch(c);
        }
        fn mvaddch<E:ToCh>(&mut self, y: c_int, x: c_int, c: E) {
            let c = c.encode(); mvaddch(y, x, c);
        }
    }
    impl<'a> AddCh for super::Window<'a> {
        fn addch<E:ToCh>(&mut self, c: E) {
            let c = c.encode();
            waddch(self.ptr, c);
        }
        fn mvaddch<E:ToCh>(&mut self, y: c_int, x: c_int, c: E) {
            let c = c.encode();
            unsafe { fail_if_err!(nc::mvwaddch(self.ptr, y, x, c)); }
        }
    }

    fn add_wch(c:&nc::cchar_t) {
        unsafe { fail_if_err!(nc::add_wch(c as *nc::cchar_t)); }
    }

    // impl<'a> super::Context<'a> { }
    // impl<'a> super::Window<'a> { }

    // impl<'a> super::Context<'a> { }
    // impl<'a> super::Window<'a> { }

    fn addchnstr(chs:&[nc::chtype], n:uint) {
        chs.as_imm_buf(|ptr, len| {
                let len = len as i32;
                let n = n as c_int;
                if (n < 0) || (len < n) { fail!(); }
                unsafe { fail_if_err!(nc::addchnstr(ptr, n)); }
            });
    }

    fn mvaddchnstr(y: c_int, x: c_int, chs:&[nc::chtype], n:uint) {
        chs.as_imm_buf(|ptr, len| {
                let len = len as i32;
                let n = n as c_int;
                if (n < 0) || (len < n) { fail!(); }
                unsafe { fail_if_err!(nc::mvaddchnstr(y, x, ptr, n)); }
            });
    }

    fn waddchnstr(w: nc::WINDOW_p, chs:&[nc::chtype], n:uint) {
        chs.as_imm_buf(|ptr, len| {
                let len = len as i32;
                let n = n as c_int;
                if (n < 0) || (len < n) { fail!(); }
                unsafe { fail_if_err!(nc::waddchnstr(w, ptr, n)); }
            });
    }

    fn mvwaddchnstr(w: nc::WINDOW_p, y: c_int, x: c_int, chs:&[nc::chtype], n:uint) {
        chs.as_imm_buf(|ptr, len| {
                let len = len as i32;
                let n = n as c_int;
                if (n < 0) || (len < n) { fail!(); }
                unsafe { fail_if_err!(nc::mvwaddchnstr(w, y, x, ptr, n)); }
            });
    }

    trait AddChstr {
        fn addchnstr<E:ToCh>(&mut self, chs:&[E], n:uint);
        fn addchstr<E:ToCh>(&mut self, chs:&[E]);
        fn mvaddchnstr<E:ToCh>(&mut self, y: c_int, x: c_int, chs:&[E], n:uint);
        fn mvaddchstr<E:ToCh>(&mut self, y: c_int, x: c_int, chs:&[E]);
    }

    impl<'a> AddChstr for super::Context<'a> {
        fn addchnstr<E:ToCh>(&mut self, chs:&[E], n:uint) {
            let v: Vec<nc::chtype> = chs.iter().map(|x| x.encode()).collect();
            addchnstr(v.as_slice(), n);
        }
        fn addchstr<E:ToCh>(&mut self, chs:&[E]) {
            let v: Vec<nc::chtype> = chs.iter().map(|x| x.encode()).collect();
            addchnstr(v.as_slice(), v.len());
        }
        fn mvaddchnstr<E:ToCh>(&mut self, y: c_int, x: c_int, chs:&[E], n:uint) {
            let v: Vec<nc::chtype> = chs.iter().map(|x| x.encode()).collect();
            mvaddchnstr(y, x, v.as_slice(), n);
        }
        fn mvaddchstr<E:ToCh>(&mut self, y: c_int, x: c_int, chs:&[E]) {
            let v: Vec<nc::chtype> = chs.iter().map(|x| x.encode()).collect();
            mvaddchnstr(y, x, v.as_slice(), v.len());
        }
    }

    impl<'a> AddChstr for super::Window<'a> {
        fn addchnstr<E:ToCh>(&mut self, chs:&[E], n:uint) {
            let v: Vec<nc::chtype> = chs.iter().map(|x| x.encode()).collect();
            waddchnstr(self.ptr, v.as_slice(), n);
        }
        fn addchstr<E:ToCh>(&mut self, chs:&[E]) {
            let v: Vec<nc::chtype> = chs.iter().map(|x| x.encode()).collect();
            waddchnstr(self.ptr, v.as_slice(), v.len());
        }
        fn mvaddchnstr<E:ToCh>(&mut self, y: c_int, x: c_int, chs:&[E], n:uint) {
            let v: Vec<nc::chtype> = chs.iter().map(|x| x.encode()).collect();
            mvwaddchnstr(self.ptr, y, x, v.as_slice(), n);
        }
        fn mvaddchstr<E:ToCh>(&mut self, y: c_int, x: c_int, chs:&[E]) {
            let v: Vec<nc::chtype> = chs.iter().map(|x| x.encode()).collect();
            mvwaddchnstr(self.ptr, y, x, v.as_slice(), v.len());
        }
    }

    impl<'a> super::Context<'a> {
        pub fn addnstr(&mut self, s: &str, n:uint) {
            if n > s.len() { fail!(); }
            let n = n as c_int;
            if n < 0 { fail!(); }
            s.with_c_str(|p| { unsafe { fail_if_err!(nc::addnstr(p, n)) } });
        }
        pub fn addstr(&mut self, s: &str) {
            // let n = s.len() as c_int;
            // if n < 0 { fail!(); }
            // s.with_c_str(|p| { unsafe { fail_if_err!(nc::addnstr(p, n)) } });
            let c : Vec<char> = s.chars().collect();
            c.as_imm_buf(|p, n| { unsafe {
                        let n = n as c_int;
                        if n < 0 { fail!(); }
                        fail_if_err!(nc::addnwstr(p as *i32, n))
                    } });
        }
        pub fn mvaddstr(&mut self, y: c_int, x: c_int, s: &str) {
            let n = s.len() as c_int;
            if n < 0 { fail!(); }
            s.with_c_str(|p| { unsafe { fail_if_err!(nc::mvaddnstr(y, x, p, n)) } });
        }
        pub fn mvaddnstr(&mut self, y: c_int, x: c_int, s: &str, n:uint) {
            if n > s.len() { fail!(); }
            let n = s.len() as c_int;
            if n < 0 { fail!(); }
            s.with_c_str(|p| { unsafe { fail_if_err!(nc::mvaddnstr(y, x, p, n)) } });
        }
    }

    impl<'a> super::Window<'a> {
        pub fn addnstr(&mut self, s: &str, n:uint) {
            if n > s.len() { fail!(); }
            let n = n as c_int;
            if n < 0 { fail!(); }
            s.with_c_str(|p| { unsafe { fail_if_err!(nc::waddnstr(self.ptr, p, n)) } });
        }
        pub fn addstr(&mut self, s: &str) {
            let n = s.len() as c_int;
            if n < 0 { fail!(); }
            s.with_c_str(|p| { unsafe { fail_if_err!(nc::waddnstr(self.ptr, p, n)) } });
        }
        pub fn mvaddstr(&mut self, y: c_int, x: c_int, s: &str) {
            let n = s.len() as c_int;
            if n < 0 { fail!(); }
            s.with_c_str(|p| { unsafe { fail_if_err!(nc::mvwaddnstr(self.ptr, y, x, p, n)) } });
        }
        pub fn mvaddnstr(&mut self, y: c_int, x: c_int, s: &str, n:uint) {
            if n > s.len() { fail!(); }
            let n = s.len() as c_int;
            if n < 0 { fail!(); }
            s.with_c_str(|p| { unsafe { fail_if_err!(nc::mvwaddnstr(self.ptr, y, x, p, n)) } });
        }
    }

    impl<'a> super::Window<'a> {
        pub fn addnstr(&mut self, s: &str, n:uint) {
            if n > s.len() { fail!(); }
            let n = n as c_int;
            if n < 0 { fail!(); }
            s.with_c_str(|p| {
                    unsafe { fail_if_err!(nc::waddnstr(self.ptr, p, n)) }
                });
        }
        pub fn addstr(&mut self, s: &str) {
            let n = s.len() as c_int;
            if n < 0 { fail!(); }
            s.with_c_str(|p| {
                    unsafe { fail_if_err!(nc::waddnstr(self.ptr, p, n)) }
                });
        }
    }
    impl<'a> super::Context<'a> {
        pub fn echo(&mut self, c: ch) {
            let c = encode(c);
            unsafe { fail_if_err!(nc::echochar(c)); }
        }
        pub fn refresh(&mut self) {
            unsafe { fail_if_err!(nc::refresh()); }
        }
    }

    impl<'a> super::Window<'a> {
        pub fn echo(&mut self, c: ch) {
            let c = encode(c);
            unsafe { fail_if_err!(nc::wechochar(self.ptr, c)); }
        }
        pub fn refresh(&mut self) {
            unsafe { fail_if_err!(nc::wrefresh(self.ptr)); }
        }
    }
    impl<'a> super::Window<'a> {
        pub fn touch(&mut self) {
            unsafe { fail_if_err!(nc::touchwin(self.ptr)); }
        }
        pub fn untouch(&mut self) {
            unsafe { fail_if_err!(nc::untouchwin(self.ptr)); }
        }
    }
}

mod screens {
    // use std::io::native::file::CFile;
    use libc::types::common::c95::FILE;
    use nc = ncurses_core;
    use ncurses_core::{WINDOW_p, SCREEN_p};
    type FilePtr = *FILE;

    pub struct Screen { pub ptr: SCREEN_p }

    impl Screen {
        pub unsafe fn wnd_ptr(&self) -> WINDOW_p {
            self.ptr as WINDOW_p
        }
    }

    fn newterm(type_: *::libc::c_char, out_: FilePtr, in_: FilePtr) -> Screen {
        let p = unsafe { nc::newterm(type_, out_, in_) };
        Screen { ptr: p }
    }
    fn set_term(s: Screen) -> Screen {
        let old_p = unsafe { nc::set_term(s.ptr) };
        Screen { ptr: old_p }
    }
    fn delscreen(s: Screen) {
        unsafe { nc::delscreen(s.ptr); }
    }

}

pub trait HasYX { fn getyx(&self) -> (libc::c_int, libc::c_int); }

impl<'a> HasYX for Context<'a> {
    fn getyx(&self) -> (libc::c_int, libc::c_int) {
        let mut y: libc::c_int = 0;
        let mut x: libc::c_int = 0;
        unsafe { nc::getyx(nc::stdscr, &mut y, &mut x); }
        (y, x)
    }
}

impl<'a> HasYX for Window<'a> {
    fn getyx(&self) -> (libc::c_int, libc::c_int) {
        let mut y: libc::c_int = 0;
        let mut x: libc::c_int = 0;
        unsafe { nc::getyx(self.ptr, &mut y, &mut x); }
        (y, x)
    }
}

impl<'a> Context<'a> {
    // XXX fail on err may be overkill for these routines; consider putting
    // in similar condition handling analogous to get_err_response above.
    pub fn beep(&mut self) { unsafe { fail_if_err!(nc::beep()); } }
    pub fn flash(&mut self) { unsafe { fail_if_err!(nc::flash()); } }
}

impl<'a> Context<'a> {
    pub fn baudrate(&self) -> libc::c_int { unsafe { nc::baudrate() } }
    pub fn erasechar(&self) -> char {
        use std::char;
        let mut c : libc::wchar_t = 0;
        unsafe { fail_if_err!(nc::erasewchar(&mut c as *mut libc::wchar_t)); }
        char::from_u32(c as u32).unwrap()
    }
    pub fn killchar(&self) -> char {
        use std::char;
        let mut c : libc::wchar_t = 0;
        unsafe { fail_if_err!(nc::killwchar(&mut c as *mut libc::wchar_t)); }
        char::from_u32(c as u32).unwrap()
    }
    pub fn has_ins_del_char(&self) -> bool { unsafe { nc::has_ic() != 0 } }
    pub fn has_ins_del_line(&self) -> bool { unsafe { nc::has_il() != 0 } }
    pub fn longname(&self) -> String {
        use std::str;
        unsafe {
            let s: *libc::c_char = nc::longname();
            str::raw::from_c_str(s)
        }
    }

    pub fn keypad(&mut self, s:&mut self::screens::Screen, enabled: bool) {
        unsafe {
            fail_if_err!(nc::keypad(s.wnd_ptr(), enabled as nc::NCURSES_BOOL));
        }
    }

    pub fn stdscr(&self) -> self::screens::Screen {
        self::screens::Screen { ptr: nc::stdscr as SCREEN_p}
    }
}

pub mod windows {
    use std::ptr;
    use super::Window;
    use nc = ncurses_core;

    // Oddly `impl<'a> Window<'a>` does not work here, despite the `use` above.
    impl<'a> super::Window<'a> {
        fn parent(&self) -> Option<Window<'a>> {
            let p = unsafe { nc::wgetparent(self.ptr) };
            if p == ptr::null() {
                None
            } else {
                Some(Window { ptr: p, ctxt: self.ctxt })
            }
        }

        fn is_cleared(&self)   -> bool { unsafe { nc::is_cleared(self.ptr)   != 0 } }
        fn is_idcok(&self)     -> bool { unsafe { nc::is_idcok(self.ptr)     != 0 } }
        fn is_idlok(&self)     -> bool { unsafe { nc::is_idlok(self.ptr)     != 0 } }
        fn is_immedok(&self)   -> bool { unsafe { nc::is_immedok(self.ptr)   != 0 } }
        fn is_keypad(&self)    -> bool { unsafe { nc::is_keypad(self.ptr)    != 0 } }
        fn is_leaveok(&self)   -> bool { unsafe { nc::is_leaveok(self.ptr)   != 0 } }
        fn is_nodelay(&self)   -> bool { unsafe { nc::is_nodelay(self.ptr)   != 0 } }
        fn is_notimeout(&self) -> bool { unsafe { nc::is_notimeout(self.ptr) != 0 } }
        fn is_scrollok(&self)  -> bool { unsafe { nc::is_scrollok(self.ptr)  != 0 } }
        fn is_syncok(&self)    -> bool { unsafe { nc::is_syncok(self.ptr)    != 0 } }

        fn scrollok(&mut self, val:bool) {
            unsafe { fail_if_err!(nc::scrollok(self.ptr, val as nc::bool_t)); }
        }
    }
}
