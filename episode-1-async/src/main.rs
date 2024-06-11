struct Task {
    name: String,
    done: bool,
    run_counter: usize,
    func: fn(usize), // new: f is a variable of type function pointer
}

impl Task {
    // f is a pointer to a fn that takes a usize as first (and only) argument and returns nothing
    fn new(name: String, f: fn(usize)) -> Self {
        Self {
            name,
            done: false,
            run_counter: 0,
            func: f, // new!
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

        (self.func)(1); // new!
    }
}

fn brush_teeth(times: usize) {
    for i in 0..times {
        println!("Brushing teeth {}", i);
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

    executor
        .tasks
        .push(Task::new("task1".to_string(), brush_teeth)); // add task 1
    executor
        .tasks
        .push(Task::new("task2".to_string(), brush_teeth)); // add task 2
    executor
        .tasks
        .push(Task::new("task3".to_string(), brush_teeth)); // add task 3

    while executor.tasks.len() != 0 {
        for task in executor.tasks.iter_mut() {
            task.run();
        }
        // clean up tasks that are done
        executor.tasks.retain(|task| !task.is_done());
    }
}
