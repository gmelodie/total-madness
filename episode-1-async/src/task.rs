use futures::task::noop_waker;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Task {
    name: String,
    done: bool,
    pub future: Pin<Box<dyn Future<Output = String>>>,
}

impl Task {
    pub fn new(name: String, future: impl Future<Output = String> + 'static) -> Self {
        Self {
            name,
            done: false,
            future: Box::pin(future),
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn poll(&mut self) -> String {
        let binding = noop_waker();
        let mut cx = Context::from_waker(&binding);

        match self.future.as_mut().poll(&mut cx) {
            Poll::Ready(output) => {
                self.done = true;
                output
            }
            Poll::Pending => "Task not finished".to_string(),
        }
    }
}
