#![deny(unsafe_code)]

use crate::sync::{thread, Arc, Condvar, Mutex};
use crate::thread::JoinHandle;

const SHARED_SIZE: usize = 10;

#[cfg(feature = "loom")]
pub mod sync {
    pub use loom::sync::{Arc, Condvar, Mutex};
    pub use loom::thread;
}

#[cfg(not(feature = "loom"))]
pub mod sync {
    pub use std::sync::{Arc, Condvar, Mutex};
    pub use std::thread;
}

pub type Task = fn(i64);

pub struct ThreadPool {
    handles: Vec<JoinHandle<()>>,
    shared: Arc<Shared>,
}

impl ThreadPool {
    /// Create a pool with `worker_count` workers.
    ///
    /// # Panics
    ///
    /// Should panic when `worker_count == 0`.
    pub fn new(worker_count: usize, task: Task) -> Self {
        if worker_count == 0 {
            panic!("Insufficient Vespene gas!");
        }

        let shared = Arc::new(Shared::new(SHARED_SIZE));

        let handles: Vec<JoinHandle<()>> = (0..worker_count)
            .map(|_| {
                let local_shared = Arc::clone(&shared);

                // move - передача владения
                thread::spawn(move || {
                    do_task(local_shared, task);
                })
            })
            .collect();

        Self { handles, shared }
    }

    /// Add one number to the work queue.
    pub fn execute(&self, num: i64) {
        let mut state = self.shared.state.lock().expect("panic");
        state.queue.push(num);
        drop(state);
        self.shared.is_working.notify_one();
    }

    /// Finish all queued work and stop all workers.
    pub fn shutdown(self) {
        let mut state = self.shared.state.lock().expect("panic");
        state.is_shutting_down = true;
        drop(state);
        self.shared.is_working.notify_all();
        self.handles
            .into_iter()
            .for_each(|h| h.join().expect("panic"));
    }
}

struct Shared {
    state: Mutex<State>,
    is_working: Condvar,
}

impl Shared {
    fn new(size: usize) -> Self {
        let shared = Mutex::new(State::new(size));
        let is_working = Condvar::new();
        Self {
            state: shared,
            is_working,
        }
    }
}

struct State {
    queue: Vec<i64>,
    is_shutting_down: bool,
}

impl State {
    fn new(size: usize) -> Self {
        Self {
            queue: Vec::with_capacity(size),
            is_shutting_down: false,
        }
    }
}

fn do_task(sh: Arc<Shared>, task: Task) {
    loop {
        let mut state = sh.state.lock().expect("state panic");

        while state.queue.is_empty() && !state.is_shutting_down {
            state = sh.is_working.wait(state).expect("panic");
        }

        let item = {
            if let Some(item) = state.queue.pop() {
                Some(item)
            } else if state.is_shutting_down {
                None
            } else {
                continue;
            }
        };

        match item {
            Some(item) => task(item),
            None => break,
        }
    }
}
