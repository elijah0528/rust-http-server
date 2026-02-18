use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

// FnOnce is a closure that can be called once
// Send means its safe to send to another thread
// Static means no borrowed references
// Box<dyn...> is a dynamically sized box since closures are difference sizes - heap allocate
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    _workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (tx, rx) = mpsc::channel::<Job>();

        // Wrap the reciever in a Arc<Mutex>
        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            let rx = Arc::clone(&rx);

            let handle =  thread::spawn(move || {
                loop {
                    let job = rx.lock().unwrap().recv().unwrap();
                    job();
                }
            });

            workers.push(handle);
        }

        Self { _workers: workers, sender: tx }
    }

    pub fn execute<F: FnOnce() + Send + 'static>(&self, f: F) {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}