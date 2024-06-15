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

    let future = brush_teeth(10);
    let task1 = Task::new("first_task".to_string(), future);

    executor.tasks.push(task1);

    while executor.tasks.len() != 0 {
        for task in executor.tasks.iter_mut() {
            task.poll();
        }
        println!("Went through all tasks once");

        // clean up tasks that are done
        executor.tasks.retain(|task| !task.is_done());
    }
}
