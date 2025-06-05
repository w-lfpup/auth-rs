use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConnectionPoolParams {
    db_path: PathBuf,
    max_connection_count: usize,
}

pub struct ConnectionPool {
    params: ConnectionPoolParams,
    connections: Mutex<Vec<Connection>>,
}

impl ConnectionPool {
    pub fn from(params: ConnectionPoolParams) -> Result<ConnectionPool, String> {
        if params.max_connection_count == 0 {
            return Err("max connections cannot be 0".to_string());
        }

        Ok(ConnectionPool {
            params: params,
            connections: Mutex::new(Vec::new()),
        })
    }

    pub fn get_connection(&mut self) -> Result<Connection, String> {
        let mut connections = match self.connections.lock() {
            Ok(connections) => connections,
            Err(e) => return Err(e.to_string()),
        };

        if let Some(conn) = connections.pop() {
            return Ok(conn);
        }

        match Connection::open(&self.params.db_path) {
            Ok(cn) => Ok(cn),
            Err(e) => return Err(e.to_string()),
        }
    }

    pub fn set_connection(&mut self, conn: Connection) -> Result<(), String> {
        let mut connections = match self.connections.lock() {
            Ok(connections) => connections,
            Err(e) => return Err(e.to_string()),
        };

        if connections.len() < self.params.max_connection_count {
            connections.push(conn);
        }

        Ok(())
    }
}
