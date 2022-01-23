mod raw;
mod tls;
mod utils;

use std::marker::PhantomData;

pub struct Crystalline<const SLOTS: usize> {
    raw: raw::Crystalline<SLOTS>,
}

impl<const SLOTS: usize> Crystalline<SLOTS> {
    const DEFAULT_EPOCH_TICK: u64 = 110;
    const DEFAULT_RETIRE_TICK: usize = 120;

    pub fn new() -> Self {
        Self {
            raw: raw::Crystalline::with_threads(
                num_cpus::get(),
                Self::DEFAULT_EPOCH_TICK,
                Self::DEFAULT_RETIRE_TICK,
            ),
        }
    }

    pub fn epoch_frequency(mut self, n: u64) -> Self {
        self.raw.epoch_tick = n;
        self
    }

    pub fn retire_frequency(mut self, n: usize) -> Self {
        self.raw.retire_tick = n;
        self
    }

    pub fn guard(&self) -> Guard<'_, SLOTS> {
        Guard {
            crystalline: self,
            _not_send: PhantomData,
        }
    }

    pub fn link<T>(&self, value: T) -> Linked<T> {
        Linked {
            value,
            node: self.raw.node_for::<T>(),
        }
    }

    pub fn link_boxed<T>(&self, value: T) -> *mut Linked<T> {
        Box::into_raw(Box::new(self.link(value)))
    }
}

pub struct Protect(pub usize);

pub struct Guard<'a, const SLOTS: usize> {
    crystalline: &'a Crystalline<SLOTS>,
    _not_send: PhantomData<*mut ()>,
}

impl<'g, const SLOTS: usize> Guard<'g, SLOTS> {
    pub unsafe fn retire<T>(&self, ptr: *mut Linked<T>, retire: unsafe fn(Link)) {
        self.crystalline.raw.retire(ptr, retire)
    }

    pub fn protect<T>(
        &self,
        op: impl FnMut() -> *mut Linked<T>,
        protect: Protect,
    ) -> *mut Linked<T> {
        self.crystalline.raw.protect(op, protect.0)
    }
}

impl<const SLOTS: usize> Drop for Guard<'_, SLOTS> {
    fn drop(&mut self) {
        unsafe { self.crystalline.raw.clear_all() }
    }
}

pub struct Link {
    node: *mut raw::Node,
}

impl Link {
    pub unsafe fn as_ptr<T>(&mut self) -> *mut Linked<T> {
        self.node as *mut _
    }
}

#[repr(C)]
pub struct Linked<T> {
    node: raw::Node,
    value: T,
}

impl<T> Linked<T> {
    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> std::ops::Deref for Linked<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::ops::DerefMut for Linked<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

pub mod retire {
    use crate::Link;

    pub unsafe fn boxed<T>(mut link: Link) {
        let _ = Box::from_raw(link.as_ptr::<T>());
    }

    pub unsafe fn in_place<T>(mut link: Link) {
        let _ = std::ptr::drop_in_place(link.as_ptr::<T>());
    }
}
