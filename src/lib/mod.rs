// pub mod commands;
pub mod prelude;
pub mod server;
pub mod Foundation;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crossbeam::{unbounded, Receiver, Sender};

enum Message {
  NewJob(Job),
  Terminate,
}

#[derive(Debug)]
pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[allow(dead_code)]
impl ThreadPool {
  /// Create a new ThreadPool.
  ///
  /// The size is the number of threads in the pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = unbounded();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      // create some threads and store them in the vector
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);

    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

#[derive(Debug)]
struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let message = receiver.lock().unwrap().recv().unwrap();

      match message {
        Message::NewJob(job) => {
          println!("Worker {} got a job; executing.", id);
          job();
        }
        Message::Terminate => {
          println!("Worker {} was told to terminate.", id);
          break;
        }
      }
    });

    Worker {
      id,
      thread: Some(thread),
    }
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    println!("Sending terminate message to all workers.");

    for _ in &mut self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    println!("Shutting down all workers.");

    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);

      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}
