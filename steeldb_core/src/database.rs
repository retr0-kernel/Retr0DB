use std::collections::HashMap;
use crate::table::{Table, TableErrors, SaveMode, FileFormat};

pub struct Database {
    pub tables: HashMap<String, Table>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            tables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, query: String) -> Result<String, String> {
        // Simple query execution logic
        Ok("Query executed".to_string())
    }
}
