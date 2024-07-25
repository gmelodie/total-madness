use futures::pending;
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ToothbrushLock {
    locked: UnsafeCell<AtomicBool>,
}

impl ToothbrushLock {
    pub fn new() -> Self {
        Self {
            locked: UnsafeCell::new(AtomicBool::new(false)),
        }
    }

    pub async fn lock(&self) -> ToothbrushGuard {
        unsafe {
            pending!();
            while (&*self.locked.get()).load(Ordering::SeqCst) == true {
                pending!();
            }
            (&*self.locked.get()).store(true, Ordering::SeqCst);
            ToothbrushGuard::new(self)
        }
    }

    fn unlock(&self) {
        unsafe {
            (&*self.locked.get()).store(false, Ordering::SeqCst);
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
