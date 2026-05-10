use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::fmt;

#[derive(Debug)]
pub enum FileActionError {
    NotFound(String),
    PermissionDenied,
    Unknown(String),
}

impl fmt::Display for FileActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileActionError::NotFound(path) => write!(f, "No se encontró el archivo: {}", path),
            FileActionError::PermissionDenied => write!(f, "Permiso denegado"),
            FileActionError::Unknown(msg) => write!(f, "Error desconocido: {}", msg),
        }
    }
}

pub struct FileManager;

impl FileManager {
    // File Reader
    pub fn read_file(path: &str) -> Result<String, Box<FileActionError>> {
        let mut file = File::open(path).map_err(|e| {
            Box::new(match e.kind() {
                std::io::ErrorKind::NotFound => FileActionError::NotFound(path.to_string()),
                std::io::ErrorKind::PermissionDenied => FileActionError::PermissionDenied,
                _ => FileActionError::Unknown(e.to_string()),
            })
        })?;

        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| {
            Box::new(FileActionError::Unknown(e.to_string()))
        })?;

        Ok(content)
    }

    // File Writer
    pub fn write_file(path: &str, data: &str) -> Result<(), Box<FileActionError>> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|_| Box::new(FileActionError::PermissionDenied))?;

        file.write_all(data.as_bytes())
            .map_err(|e| Box::new(FileActionError::Unknown(e.to_string())))?;
        
        Ok(())
    }
}