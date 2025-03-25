use seize::{reclaim, Collector, Guard, LocalGuard};

fn main() {}

#[no_mangle]
pub fn do_pin(collector: &Collector) -> LocalGuard {
    collector.enter()
}

#[no_mangle]
pub fn do_unpin(guard: LocalGuard) {
    drop(guard);
}

#[no_mangle]
pub fn do_defer_retire(guard: &LocalGuard, ptr: *mut i32) {
    unsafe {
        guard.defer_retire(ptr, reclaim::boxed);
    }
}
