use std::collections::HashMap;
use crate::table::{Table, DataType, SaveMode, FileFormat};

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
        let tokens: Vec<&str> = query.split_whitespace().collect();
        match tokens[0].to_lowercase().as_str() {
            "create" => self.create_table(tokens),
            "insert" => self.insert_into_table(tokens),
            "select" => self.select_from_table(tokens),
            _ => Err("Unknown command".to_string()),
        }
    }

    fn create_table(&mut self, tokens: Vec<&str>) -> Result<String, String> {
        let table_name = tokens[2];
        self.tables.insert(
            table_name.to_string(),
            Table::new(table_name.to_string())
        );
        Ok(format!("Table {} created", table_name))
    }

    fn insert_into_table(&mut self, tokens: Vec<&str>) -> Result<String, String> {
        let table_name = tokens[2];
        if let Some(table) = self.tables.get_mut(table_name) {
            let columns: Vec<&str> = tokens[4].trim_matches('(').trim_matches(')').split(',').collect();
            let values: Vec<&str> = tokens[6].trim_matches('(').trim_matches(')').split(',').collect();
            for (column, value) in columns.iter().zip(values.iter()) {
                table.insert(column.trim().to_string(), DataType::String(value.trim().to_string()));
            }
            Ok("Insert successful".to_string())
        } else {
            Err(format!("Table {} not found", table_name))
        }
    }

    fn select_from_table(&mut self, tokens: Vec<&str>) -> Result<String, String> {
        let columns: Vec<&str> = tokens[1].split(',').collect();
        let table_name = tokens[3];
        if let Some(table) = self.tables.get(table_name) {
            Ok(table.select(columns))
        } else {
            Err(format!("Table {} not found", table_name))
        }
    }
}
