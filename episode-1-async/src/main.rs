mod executor;
mod task;

use executor::Executor;
use task::Task;

fn main() {
    let mut executor = Executor::new();

    while executor.tasks.len() != 0 {
        for task in executor.tasks.iter_mut() {
            task.poll();
        }
        // clean up tasks that are done
        executor.tasks.retain(|task| !task.is_done());
    }
}
