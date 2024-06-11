struct Task {
    name: String,
    done: bool,
    max_runs: usize,
    run_counter: usize,
    func: fn(usize),
}

impl Task {
    fn new(name: String, max_runs: usize, f: fn(usize)) -> Self {
        // f is a pointer to a fn that takes a usize as first (and only) argument and returns nothing
        Self {
            name,
            done: false,
            max_runs,
            run_counter: 0,
            func: f,
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn run(&mut self) {
        if self.run_counter == self.max_runs {
            self.done = true;
            return;
        }
        self.run_counter += 1;

        (self.func)(1);
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

    let gabe = Task::new("gabe".to_string(), 20, brush_teeth);
    let nathan = Task::new("nathan".to_string(), 30, brush_teeth);
    let fefe = Task::new("fefe".to_string(), 100, brush_teeth);

    executor.tasks.push(gabe);
    executor.tasks.push(nathan);
    executor.tasks.push(fefe);

    while executor.tasks.len() != 0 {
        for task in executor.tasks.iter_mut() {
            task.run();
        }
        // clean up tasks that are done
        executor.tasks.retain(|task| !task.is_done());
    }
}
