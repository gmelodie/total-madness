use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

pub struct Task {
    name: String,
    done: bool,
    future: Pin<Box<dyn Future<Output = String>>>,
    waker: Waker,
}

impl Task {
    pub fn new(name: String, future: impl Future<Output = String> + 'static) -> Self {
        Self {
            name,
            done: false,
            future: Box::pin(future),
            waker: futures::task::noop_waker(),
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn poll(&mut self) -> String {
        let mut cx = Context::from_waker(&self.waker);

        if self.done {
            return "Ready".to_string();
        }

        match self.future.as_mut().poll(&mut cx) {
            Poll::Ready(output) => {
                self.done = true;
                output
            }
            Poll::Pending => "Task not finished".to_string(),
        }
    }
}
