// TODO this is test without sub modules, is good enough?

use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use serde::Serialize;

pub trait DebugLogger {
    fn debug(&self, msg: &dyn Debug);

    fn debug_d(&self, msg: &dyn Display);

    fn enabled(&self) -> bool {
        true
    }
}

pub struct EmptyDebugLogger;

impl DebugLogger for EmptyDebugLogger {
    fn debug(&self, msg: &dyn Debug) {
        // do nothing
    }

    fn debug_d(&self, msg: &dyn Display) {
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

    fn debug_d(&self, msg: &dyn Display) {
        println!("DEBUG: {}", msg);
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

    fn debug_d(&self, msg: &dyn Display) {
        self._write_to_file(&format!("{}", msg));
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

// TODO as generic this can't go to trait as size is then not known during compile time,
//      but is there anyway include this to trait? Or something similar?
pub fn debug_as_json<T>(log: &Box<dyn DebugLogger>, msg: &T)
where
    T: Serialize,
{
    if log.enabled() {
        log.debug_d(&format!(
            "JSON: {}",
            serde_json::to_string_pretty(&msg).unwrap()
        ));
    }
}
