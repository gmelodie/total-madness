use futures::pending;
use std::cell::UnsafeCell;

pub struct ToothbrushLock {
    locked: UnsafeCell<bool>,
}

impl ToothbrushLock {
    pub fn new() -> Self {
        Self {
            locked: UnsafeCell::new(false),
        }
    }

    pub async fn lock(&self) -> ToothbrushGuard {
        unsafe {
            pending!();
            while *self.locked.get() == true {
                pending!();
            }
            *self.locked.get() = true;
            ToothbrushGuard::new(self)
        }
    }

    fn unlock(&self) {
        unsafe {
            *self.locked.get() = false;
        }
    }
}

pub struct ToothbrushGuard<'guard_life> {
    mutex: &'guard_life ToothbrushLock,
}
impl<'guard_life> ToothbrushGuard<'guard_life> {
    pub fn new(mutex: &'guard_life ToothbrushLock) -> Self {
        ToothbrushGuard { mutex }
    }
}
impl Drop for ToothbrushGuard<'_> {
    fn drop(&mut self) {
        self.mutex.unlock();
    }
}
