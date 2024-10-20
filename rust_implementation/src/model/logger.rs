use std::{ fs::File, io::{self, Write}, sync::mpsc::{self, Receiver, Sender}, thread};

pub struct HistoryLogger {
    tx: mpsc::Sender<String>,
}

impl HistoryLogger {
    pub fn build(file_path : &str) -> HistoryLogger {
        let (tx, rx) : (Sender<String>, Receiver<String>) = mpsc::channel();
        let worker = Worker::build(String::from(file_path));

        thread::spawn(
            move ||
                loop {
                    let msg = rx.recv();
                    match msg {
                        Ok(mut s) => {
                            s.push('\n');
                            worker.log(&s[..]);
                        },
                        Err(_) => {
                            worker.flush();
                            todo!()
                        },
                    }
                }
        );
        
        HistoryLogger {
            tx,
        }
    }

    pub fn listen(&self, event : String) {
        self.tx.send(event).unwrap();
    }
}

struct Worker {
    file_path: String,
    f : Box<File>,
}

impl Worker {
    fn build(file_path : String) -> Worker {
        let f = File::options().create(true).read(true).write(true).truncate(true).open(&file_path).unwrap();
        Worker {
            file_path,
            f: Box::new(f),
        }
    }
}

impl Log for Worker {
    fn log(&self, s : &str) {
        self.f.as_ref().write(s.as_bytes()).unwrap();
    }

    fn flush(&self) {
        self.f.as_ref().flush().unwrap();
    }
}



pub trait Log {
    fn log(&self, s : &str);
    fn flush(&self);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn log_hello_world() {
        let logger = HistoryLogger::build("hw_test.txt");
        logger.listen("Hello World!".to_string());
    }
}