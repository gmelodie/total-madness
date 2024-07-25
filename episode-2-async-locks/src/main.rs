mod atomic_lock;
mod executor;
#[allow(dead_code)]
mod lock;
mod task;

use std::sync::Arc;

use atomic_lock::ToothbrushLock;
use executor::Executor;
// use lock::ToothbrushLock;
use task::Task;

async fn brush_teeth(brush_lock: Arc<ToothbrushLock>, times: usize) -> String {
    for i in 0..times {
        brush_lock.lock().await;
        println!("Brushing teeth {}", i);
    }
    return "Done".to_string();
}

fn main() {
    let mut executor = Executor::new();

    let brush_lock = Arc::new(ToothbrushLock::new());

    let task_gabe = Task::new("gabe".to_string(), brush_teeth(brush_lock.clone(), 20));
    let task_nat = Task::new("nat".to_string(), brush_teeth(brush_lock.clone(), 30));
    let task_fefe = Task::new("fefe".to_string(), brush_teeth(brush_lock.clone(), 100));

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
