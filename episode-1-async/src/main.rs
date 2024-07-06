mod executor;
mod task;

use executor::Executor;
use futures::pending;
use task::Task;

async fn brush_teeth(times: usize) -> String {
    for i in 0..times {
        println!("Brushing teeth {}", i);
        pending!(); // new!
    }
    return "Done".to_string();
}

fn main() {
    let mut executor = Executor::new();

    let task_gabe = Task::new("gabe".to_string(), brush_teeth(20));
    let task_nat = Task::new("nat".to_string(), brush_teeth(30));
    let task_fefe = Task::new("fefe".to_string(), brush_teeth(100));

    executor.tasks.push(task_gabe);
    executor.tasks.push(task_nat);
    executor.tasks.push(task_fefe);

    while executor.tasks.len() != 0 {
        for task in executor.tasks.iter_mut() {
            print!("{}: ", task.name);
            task.poll();
        }
        println!("--- Went through all tasks once");

        // clean up tasks that are done
        executor.tasks.retain(|task| !task.is_done());
    }
}
