use std::sync::atomic;
use std::cell;
use crate::prelude::*;
use std::sync::atomic::Ordering;

#[derive(Default)]
pub struct ReadIfSet<T> {
    set: atomic::AtomicBool,
    inner: cell::Cell<Option<T>>,
}

impl ReadIfSet<T> {
    pub fn get(&self) -> Option<&T> {
        if self.set {
            Some(&self.inner.unwrap())
        } else {
            None
        }
    }

    pub fn set_with(&self, f: impl FnOnce() -> T) {
        if self.set.load(Ordering::Relaxed) {
            Err(anyhow!("ReadIfSet can only be set once! {}", self.inner.ok()))
        }
        self.inner.set(Some(f()));
    }

    pub fn set(&self, val: T) -> Result<()> {
        if self.set.load(Ordering::Relaxed) {
            Err(anyhow!("ReadIfSet can only be set once! {}", self.inner.ok()))
        }
        self.inner.set(Some(val));
        Ok(())
    }
}

