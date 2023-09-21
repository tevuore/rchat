// TODO this is test without sub modules, is good enough?

use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub trait DebugLogger {
    fn debug(&self, msg: &dyn Debug);
}

pub struct EmptyDebugLogger;

impl DebugLogger for EmptyDebugLogger {
    fn debug(&self, msg: &dyn Debug) {
        // do nothing
    }
}

pub struct StdoutDebugLogger;

impl DebugLogger for StdoutDebugLogger {
    fn debug(&self, msg: &dyn Debug) {
        println!("{:?}", msg);
    }
}

pub struct FileDebugLogger {
    file_path: PathBuf,
}

impl DebugLogger for FileDebugLogger {
    fn debug(&self, msg: &dyn Debug) {
        self._write_to_file(&format!("{:?}", msg));
    }
}

impl FileDebugLogger {
    pub fn new(file: &PathBuf) -> Self {
        FileDebugLogger {
            file_path: file.clone(),
        }
    }

    fn _write_to_file(&self, content: &String) -> std::io::Result<()> {
        let mut file = File::create(&self.file_path)?;
        match file.write_all(content.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                println!("Error writing to file: {}", e);
            }
        }
        Ok(())
    }
}
