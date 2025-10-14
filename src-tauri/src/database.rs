use std::sync::{Arc, Mutex};

use rusqlite::Connection;

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

// each vault is a table

impl Database {
    
}
