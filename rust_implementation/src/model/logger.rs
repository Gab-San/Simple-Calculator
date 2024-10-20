use std::{ fs::File, io::Write, sync::mpsc::{self, Receiver, Sender}, thread};

pub struct HistoryLogger {
    file_path : String,
    tx: mpsc::Sender<String>,
}

impl HistoryLogger {
    pub fn build(file_path : &str) -> HistoryLogger {
        let file_path = String::from(file_path);

        let (tx, rx) : (Sender<String>, Receiver<String>) = mpsc::channel();
        let worker = FileLogger::build(&file_path);

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
            file_path,
            tx,
        }
    }

    pub fn store(&self, expression : &str) {
        self.tx.send(expression.to_string()).unwrap();
    }

    pub fn file_path(&self) -> &String {
        &self.file_path
    }
}

struct FileLogger {
    f : Box<File>,
}

impl FileLogger {
    fn build(file_path : &str) -> FileLogger {
        let f = File::options().create(true).read(true).write(true).truncate(true).open(file_path).unwrap();
        FileLogger {
            f: Box::new(f),
        }
    }
}

impl Log for FileLogger {
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
        logger.store("Hello World!");
    }
}