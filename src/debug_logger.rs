// TODO this is test without sub modules, is good enough?

use serde::Serialize;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub trait DebugLogger {
    fn debug(&self, msg: &dyn Debug);

    fn enabled(&self) -> bool {
        true
    }
}

// TODO as generic this can't go to trait as size is then not known during compile time,
//      but is there anyway include this to trait? Or something similar?
pub fn debug_as_json<T>(log: &Box<dyn DebugLogger>, msg: &T)
where
    T: Serialize,
{
    if log.enabled() {
        log.debug(&format!(
            "Request body:\n{}",
            serde_json::to_string_pretty(&msg).unwrap()
        ));
    }
}

pub struct EmptyDebugLogger;

impl DebugLogger for EmptyDebugLogger {
    fn debug(&self, msg: &dyn Debug) {
        // do nothing
    }

    fn enabled(&self) -> bool {
        false
    }
}

pub struct StdoutDebugLogger;

impl DebugLogger for StdoutDebugLogger {
    fn debug(&self, msg: &dyn Debug) {
        println!("DEBUG: {:?}", msg);
    }

    fn enabled(&self) -> bool {
        true
    }
}

pub struct FileDebugLogger {
    file_path: PathBuf,
}

impl DebugLogger for FileDebugLogger {
    fn debug(&self, msg: &dyn Debug) {
        self._write_to_file(&format!("{:?}", msg));
    }

    fn enabled(&self) -> bool {
        true
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
