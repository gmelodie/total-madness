use std::sync::atomic::{AtomicBool, Ordering};

struct ToothbrushLock {
    locked: AtomicBool,
}

impl ToothbrushLock {
    fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    fn lock(&mut self) {
        while self.locked.load(Ordering::SeqCst) {}
        self.locked.store(true, Ordering::SeqCst);
    }

    fn try_lock(&mut self) -> bool {
        let expected = false;
        let new = true;
        match self
            .locked
            .compare_exchange(expected, new, Ordering::SeqCst, Ordering::SeqCst)
        {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    fn unlock(&mut self) {
        self.locked.store(false, Ordering::SeqCst);
    }
}

fn main() {
    let mut tb_lock = ToothbrushLock::new();
    tb_lock.lock();
    // brush teeth
    if tb_lock.try_lock() { // nathan locks toothbrush
         // nathan brushes teeth
    }
    tb_lock.unlock();
}
