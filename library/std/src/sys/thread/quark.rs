use crate::ffi::CStr;
use crate::io;
use crate::num::NonZero;
use crate::thread::ThreadInit;
use crate::time::Duration;

pub const DEFAULT_MIN_STACK_SIZE: usize = 1024 * 64;

pub struct Thread(!);

impl Thread {
    pub unsafe fn new(_stack: usize, _init: Box<ThreadInit>) -> io::Result<Thread> {
        Err(io::Error::UNSUPPORTED_PLATFORM)
    }

    pub fn join(self) {
        self.0
    }
}

pub fn set_name(_name: &CStr) {}

pub fn current_os_id() -> Option<u64> {
    None
}

pub fn available_parallelism() -> io::Result<NonZero<usize>> {
    Ok(NonZero::new(1).unwrap())
}

pub fn yield_now() {
    quark_rt::syscall::sys_yield();
}

pub fn sleep(dur: Duration) {
    let ms = dur.as_millis() as u64;
    quark_rt::rt::sleep_ms(ms);
}

pub fn sleep_until(deadline: crate::time::Instant) {
    // No direct absolute-time sleep in Quark; fall back to relative sleep.
    use crate::time::Instant;
    while let Some(delay) = deadline.checked_duration_since(Instant::now()) {
        sleep(delay);
    }
}
