use std::libc;
pub use signal = self::wrap_signal;

pub static SIGHUP    : libc::c_int = 1; /* hangup */
pub static SIGINT    : libc::c_int = 2; /* interrupt */
pub static SIGQUIT   : libc::c_int = 3; /* quit */
pub static SIGILL    : libc::c_int = 4; /* illegal instruction (not reset when caught) */
pub static SIGTRAP   : libc::c_int = 5; /* trace trap (not reset when caught) */
pub static SIGABRT   : libc::c_int = 6; /* abort() */
pub static SIGFPE    : libc::c_int = 8; /* floating point exception */
pub static SIGKILL   : libc::c_int = 9; /* kill (cannot be caught or ignored) */
pub static SIGBUS    : libc::c_int = 10; /* bus error */
pub static SIGSEGV   : libc::c_int = 11; /* segmentation violation */
pub static SIGSYS    : libc::c_int = 12; /* bad argument to system call */
pub static SIGPIPE   : libc::c_int = 13; /* write on a pipe with no one to read it */
pub static SIGALRM   : libc::c_int = 14; /* alarm clock */
pub static SIGTERM   : libc::c_int = 15; /* software termination signal from kill */
pub static SIGURG    : libc::c_int = 16; /* urgent condition on IO channel */
pub static SIGSTOP   : libc::c_int = 17; /* sendable stop signal not from tty */
pub static SIGTSTP   : libc::c_int = 18; /* stop signal from tty */
pub static SIGCONT   : libc::c_int = 19; /* continue a stopped process */
pub static SIGCHLD   : libc::c_int = 20; /* to parent on child stop or exit */
pub static SIGTTIN   : libc::c_int = 21; /* to readers pgrp upon background tty read */
pub static SIGTTOU   : libc::c_int = 22; /* like TTIN for output if (tp->t_local&LTOSTOP) */
pub static SIGXCPU   : libc::c_int = 24; /* exceeded CPU time limit */
pub static SIGXFSZ   : libc::c_int = 25; /* exceeded file size limit */
pub static SIGVTALRM : libc::c_int = 26; /* virtual time alarm */
pub static SIGPROF   : libc::c_int = 27; /* profiling time alarm */
pub static SIGUSR1   : libc::c_int = 30; /* user defined signal 1 */
pub static SIGUSR2   : libc::c_int = 31; /* user defined signal 2 */

#[deriving(Clone)]
pub enum name_t {
    HUP  = SIGHUP,  INT  = SIGINT,  QUIT = SIGQUIT,
    ILL  = SIGILL,  TRAP = SIGTRAP, ABRT = SIGABRT,
    FPE  = SIGFPE,  KILL = SIGKILL, BUS  = SIGBUS,
    SEGV = SIGSEGV, SYS  = SIGSYS,  PIPE = SIGPIPE,
    ALRM = SIGALRM, TERM = SIGTERM, URG  = SIGURG,
    STOP = SIGSTOP, TSTP = SIGTSTP, CONT = SIGCONT,
    CHLD = SIGCHLD, TTIN = SIGTTIN, TTOU = SIGTTOU,
    XCPU = SIGXCPU, XFSZ = SIGXFSZ, VTALRM = SIGVTALRM,
    PROF = SIGPROF, USR1 = SIGUSR1, USR2 = SIGUSR2,
}

impl name_t {
    fn description(self) -> &'static str {
        match self {
            HUP    =>  "terminal line hangup",
            INT    =>  "interrupt program",
            QUIT   =>  "quit program",
            ILL    =>  "illegal instruction",
            TRAP   =>  "trace trap",
            ABRT   =>  "abort program (formerly SIGIOT)",
            // EMT    =>  "emulate instruction executed",
            FPE    =>  "floating-point exception",
            KILL   =>  "kill program",
            BUS    =>  "bus error",
            SEGV   =>  "segmentation violation",
            SYS    =>  "non-existent system call invoked",
            PIPE   =>  "write on a pipe with no reader",
            ALRM   =>  "real-time timer expired",
            TERM   =>  "software termination signal",
            URG    =>  "urgent condition present on socket",
            STOP   =>  "stop (cannot be caught or ignored)",
            TSTP   =>  "stop signal generated from keyboard",
            CONT   =>  "continue after stop",
            CHLD   =>  "child status has changed",
            TTIN   =>  "background read attempted from control terminal",
            TTOU   =>  "background write attempted to control terminal",
            // IO     =>  "I/O is possible on a descriptor (see fcntl(2))",
            XCPU   =>  "cpu time limit exceeded (see setrlimit(2))",
            XFSZ   =>  "file size limit exceeded (see setrlimit(2))",
            VTALRM =>  "virtual time alarm (see setitimer(2))",
            PROF   =>  "profiling timer alarm (see setitimer(2))",
            // WINCH  =>  "Window size change",
            // INFO   =>  "status request from keyboard",
            USR1   =>  "User defined signal 1",
            USR2   =>  "User defined signal 2",
        }
    }
}

pub type handler_t = extern "C" fn(libc::c_int);

mod ffi {
    use std::libc;
    use super::handler_t;

    extern "C" {
        pub fn signal(sig: libc::c_int, func: handler_t) -> handler_t;
    }
}

unsafe fn wrap_signal(sig: name_t, func: handler_t) -> handler_t {
    ffi::signal(sig as libc::c_int, func)
}
