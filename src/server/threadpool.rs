use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

// FnOnce is a closure that can be called once
// Send means its safe to send to another thread
// Static means no borrowed references
// Box<dyn...> is a dynamically sized box since closures are difference sizes - heap allocate
type Job = Box<dyn FnOnce() + Send + 'static>