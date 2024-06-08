struct ToothbrushLock {
    locked: bool,
}

impl ToothbrushLock {
    fn new() -> Self {
        Self { locked: false }
    }

    fn lock(&mut self) {
        while self.locked == true {}
        self.locked = true;
    }

    fn try_lock(&mut self) -> bool {
        let expected = false;
        let new = true;
        return compare_and_exchange(&mut self.locked, expected, new);
    }

    fn unlock(&mut self) {
        self.locked = false;
    }
}
fn compare_and_exchange(original: &mut bool, expected: bool, new: bool) -> bool {
    if *original == expected {
        *original = new;
        return true;
    }
    return false;
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
