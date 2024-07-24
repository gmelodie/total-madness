use crate::Task;

pub struct Executor {
    pub tasks: Vec<Task>,
}

impl Executor {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }
}
