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
    pub fn read_file(path: &str, chunk_size: usize) -> Result<Vec<String>, Box<FileActionError>> {
        let mut file = File::open(path).map_err(|e| {
            Box::new(match e.kind() {
                std::io::ErrorKind::NotFound => FileActionError::NotFound(path.to_string()),
                std::io::ErrorKind::PermissionDenied => FileActionError::PermissionDenied,
                _ => FileActionError::Unknown(e.to_string()),
            })
        })?;

        let mut chunks = Vec::new();
        // Creamos un buffer intermedio con el tamaño límite especificado
        let mut buffer = vec![0; chunk_size];

        loop {
            let bytes_read = file.read(&mut buffer).map_err(|e| {
                Box::new(FileActionError::Unknown(e.to_string()))
            })?;

            // Si se leyeron 0 bytes, significa que llegamos al final del archivo
            if bytes_read == 0 {
                break;
            }

            // Se convierten los bytes leídos válidos a un String
            let chunk_str = String::from_utf8_lossy(&buffer[..bytes_read]).into_owned();
            chunks.push(chunk_str);
        }

        Ok(chunks)
    }

    pub fn write_file(path: &str, data: &str, chunk_size: usize) -> Result<(), Box<FileActionError>> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|_| Box::new(FileActionError::PermissionDenied))?;

        let bytes = data.as_bytes();
        let mut offset = 0;

        // Iteramos sobre el string en porciones de tamaño 'chunk_size'
        while offset < bytes.len() {
            let end = std::cmp::min(offset + chunk_size, bytes.len());
            let chunk = &bytes[offset..end];

            file.write_all(chunk).map_err(|e| {
                Box::new(FileActionError::Unknown(e.to_string()))
            })?;

            file.flush().map_err(|e| Box::new(FileActionError::Unknown(e.to_string())))?;

            offset = end;
        }
        
        Ok(())
    }
}
