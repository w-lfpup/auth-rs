use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Connector {
    db_path: PathBuf,
    max_read_connections: usize,
    max_write_connections: usize,
    read_connections: Mutex<Vec<Connection>>,
    write_connections: Mutex<Vec<Connection>>,
}

impl Connector {
    pub fn from(
        db_path: &PathBuf,
        max_read_connections: usize,
        max_write_connections: usize,
    ) -> Result<Connector, String> {
        let max_read = match max_read_connections {
            0 => 1,
            _ => max_read_connections,
        };

        let max_write = match max_write_connections {
            0 => 1,
            _ => max_write_connections,
        };

        Ok(Connector {
            db_path: db_path.clone(),
            max_read_connections: max_read,
            max_write_connections: max_write,
            read_connections: Mutex::new(Vec::new()),
            write_connections: Mutex::new(Vec::new()),
        })
    }

    pub fn get_read_connection(&mut self) -> Result<Connection, String> {
        let mut read_connections = match self.read_connections.lock() {
            Ok(read_connections) => read_connections,
            Err(e) => return Err(e.to_string()),
        };

        if let Some(conn) = read_connections.pop() {
            return Ok(conn);
        }

        match Connection::open(&self.db_path) {
            Ok(cn) => Ok(cn),
            Err(e) => return Err(e.to_string()),
        }
    }

    pub fn set_read_connection(&mut self, conn: Connection) -> Result<(), String> {
        let mut read_connections = match self.read_connections.lock() {
            Ok(read_connections) => read_connections,
            Err(e) => return Err(e.to_string()),
        };

        if read_connections.len() < self.max_read_connections {
            read_connections.push(conn);
        }

        Ok(())
    }

    pub fn get_write_connection(&mut self) -> Result<Connection, String> {
        let mut write_connections = match self.write_connections.lock() {
            Ok(write_connections) => write_connections,
            Err(e) => return Err(e.to_string()),
        };

        if let Some(conn) = write_connections.pop() {
            return Ok(conn);
        }

        match Connection::open(&self.db_path) {
            Ok(cn) => Ok(cn),
            Err(e) => return Err(e.to_string()),
        }
    }

    pub fn set_write_connection(&mut self, conn: Connection) -> Result<(), String> {
        let mut write_connections = match self.write_connections.lock() {
            Ok(write_connections) => write_connections,
            Err(e) => return Err(e.to_string()),
        };

        if write_connections.len() < self.max_write_connections {
            write_connections.push(conn);
        }

        Ok(())
    }
}
