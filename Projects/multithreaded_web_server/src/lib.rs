use std::{error::Error, fmt, sync::{mpsc, Arc, Mutex}, thread};

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate
}

#[derive(Debug)]
pub struct PoolCreationError;

impl fmt::Display for PoolCreationError {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        write!(format, "Error creating pool")
    }
}

impl Error for PoolCreationError {}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        Worker {
            id,
            thread: Some(thread::spawn(move || {
                loop {
                    let message = receiver.lock().unwrap().recv().unwrap();

                    match message {
                        Message::NewJob(job) => {
                            println!("Worker {} got a job; executing.", id);

                            job.call_box();
                        },
                        Message::Terminate => {
                            println!("Worker {} was told to terminate.", id);

                            break;
                        }
                    }
                }
            }))
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size < 1 {
            return Err(PoolCreationError);
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Ok(ThreadPool {
            workers,
            sender
        })
    }

    pub fn execute<F>(&self, f: F) 
        where 
            F: FnOnce() + Send + 'static 
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}