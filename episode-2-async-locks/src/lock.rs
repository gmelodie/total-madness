use std::ops::{Deref, DerefMut};

pub struct ToothbrushLock {
    locked: bool,
}

impl ToothbrushLock {
    pub fn new() -> Self {
        Self { locked: false }
    }

    pub fn lock(&self) -> ToothbrushLockGuard {
        while self.locked {} // wait until can lock
        ToothbrushLockGuard::new(self)
    }
}

struct ToothbrushLockGuard<'guard_life> {
    mutex: &'guard_life ToothbrushLock,
}

impl ToothbrushLockGuard<'_> {
    pub fn new(locked: &mut bool) -> Self {
        *locked = true;
        ToothbrushLockGuard { locked }
    }

    pub fn unlock(&mut self) {
        *self.locked = false;
    }
}
