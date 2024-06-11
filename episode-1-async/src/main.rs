struct Task {
    name: String,
    done: bool,
    run_counter: usize,
}

impl Task {
    fn new(name: String) -> Self {
        Self {
            name, // this is equivalent to `name: name`
            done: false,
            run_counter: 0,
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn run(&mut self) {
        if self.run_counter == 100 {
            self.done = true;
            return;
        }
        self.run_counter += 1;
        println!("Hi from task: {}", self.name);
    }
}

struct Executor {
    tasks: Vec<Task>,
}

impl Executor {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }
}

fn main() {
    let mut executor = Executor::new();

    executor.tasks.push(Task::new("task1".to_string())); // add task 1
    executor.tasks.push(Task::new("task2".to_string())); // add task 2
    executor.tasks.push(Task::new("task3".to_string())); // add task 3

    while executor.tasks.len() != 0 {
        for task in executor.tasks.iter_mut() {
            task.run();
        }
        // clean up tasks that are done
        executor.tasks.retain(|task| !task.is_done());
    }
}
