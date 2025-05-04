use std::path::PathBuf;
use rusqlite::{Connection};

pub struct Connector {
    db_path: PathBuf,
    max_read_connections: usize, 
    max_write_connections: usize,
    read_connections: Vec<Connection>,
    write_connections: Vec<Connection>,
}

impl Connector {
    pub fn from(
        db_path: &PathBuf,
        max_read_connections: usize, 
        max_write_connections: usize,
    ) -> Result<Connector, String> {
        // get duration
        
        Ok(Connector {
            db_path: db_path.clone(),
            max_read_connections: 12, 
            max_write_connections: 12,
            read_connections: Vec::new(),
            write_connections: Vec::new(),
        })
    }

    pub fn get_read_connection(&mut self) -> Result<Connection, String> {
        if let Some(conn) = self.read_connections.pop() {
            return Ok(conn);
        }
        
        match Connection::open(&self.db_path) {
            Ok(cn) => Ok(cn),
            Err(e) => return Err("falled to connect to sqlite db (session)".to_string()),
        }
    }

    pub fn set_read_connection(&mut self, conn: Connection) {
        if self.write_connections.len() < self.max_write_connections {
            self.write_connections.push(conn)
        }
    }

    pub fn get_write_connection(&mut self) -> Result<Connection, String> {
        if let Some(conn) = self.write_connections.pop() {
            return Ok(conn);
        }
        
        match Connection::open(&self.db_path) {
            Ok(cn) => Ok(cn),
            Err(e) => return Err("falled to connect to sqlite db (session)".to_string()),
        }
    }

    pub fn set_write_connection(&mut self, conn: Connection) {
        if self.write_connections.len() < self.max_write_connections {
            self.write_connections.push(conn)
        }
    }
}
