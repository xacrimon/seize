use cfg_if::cfg_if;

#[allow(unused_macros)]
macro_rules! fatal_assert {
    ($cond:expr) => {
        if !$cond {
            #[allow(unused_unsafe)]
            unsafe {
                libc::abort();
            }
        }
    };
}

pub use default::*;

mod default {
    use core::sync::atomic::{fence, AtomicPtr, AtomicU64, Ordering};

    /// Issues a light memory barrier for fast path.
    ///
    /// It just issues the normal memory barrier instruction.
    #[inline]
    pub fn light() {
        fence(Ordering::SeqCst);
    }

    #[inline]
    pub fn light_ptr_store<T>(m: &AtomicPtr<T>, value: *mut T) {
        m.store(value, Ordering::SeqCst);
    }

    #[inline]
    pub fn light_u64_store(m: &AtomicU64, value: u64) {
        m.store(value, Ordering::SeqCst);
    }

    #[inline]
    pub fn light_ptr_load<T>(m: &AtomicPtr<T>, _ordering: Ordering) -> *mut T {
        m.load(Ordering::SeqCst)
    }

    /// Issues a heavy memory barrier for slow path.
    ///
    /// It just issues the normal memory barrier instruction.
    #[inline]
    pub fn heavy() {
        fence(Ordering::SeqCst);
    }
}
