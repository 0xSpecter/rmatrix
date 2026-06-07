use std::fs::File;
use std::io::Write;

// due to using the terminal for rendering, normal println's won't work, since you just write over
// them. therefor a logger to save logging messages and dump contents of logfile on crash nessacary
pub struct Logger {
    file: File,
    path: String,
}

impl Logger {
    pub fn new() -> Self {
        let path = String::from("log.txt");
        let file = File::create(&path).unwrap();
        Self { file, path }
    }

    pub fn log(&mut self, msg: &str) {
        writeln!(self.file, "{}", msg).unwrap();
    }

    pub fn dump(&self) {
        let content = std::fs::read_to_string(&self.path).unwrap();
        print!("{content}");
    }
}

// dumps content of log.txt after chrash and terminal reset
impl Drop for Logger {
    fn drop(&mut self) {
        self.dump();
    }
}
