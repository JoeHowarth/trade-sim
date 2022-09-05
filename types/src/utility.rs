use crate::prelude::*;
use std::{
    cell,
    cell::Ref,
    sync::{atomic, atomic::Ordering},
};

pub struct ReadIfSet<T> {
    set: atomic::AtomicBool,
    inner: cell::UnsafeCell<Option<T>>,
}

impl<T> Default for ReadIfSet<T> {
    fn default() -> Self {
        Self {
            set: Default::default(),
            inner: cell::UnsafeCell::new(None),
        }
    }
}

unsafe impl<T: Send> Send for ReadIfSet<T> {}

unsafe impl<T: Sync> Sync for ReadIfSet<T> {}

impl<T: std::fmt::Debug> ReadIfSet<T> {
    pub fn get(&self) -> Option<&T> {
        if self.set.load(Ordering::SeqCst) {
            unsafe { self.inner.get().as_ref().unwrap().as_ref() }
            // Some(Ref::map(self.inner.borrow(), |x| x.as_ref().unwrap()))
        } else {
            None
        }
    }

    pub fn set_with_if_unset(&self, f: impl FnOnce() -> T) {
        match self.set_with(f) {
            Ok(()) => (),
            Err(_) => (),
        }
    }

    pub fn set_with(&self, f: impl FnOnce() -> T) -> Result<()> {
        if self.set.load(Ordering::SeqCst) {
            return Err(anyhow!("ReadIfSet can only be set once!"));
        }
        unsafe { self.inner.get().write(Some(f())) };
        self.set.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn set(&self, val: T) -> Result<()> {
        self.set_with(id(val))
    }
}

fn id<T>(t: T) -> impl FnOnce() -> T {
    move || t
}
