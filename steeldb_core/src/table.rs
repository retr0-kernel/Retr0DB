use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum DataType {
    String(String),
    Integer32(i32),
    Float32(f32),
}

pub struct Table {
    pub name: String,
    pub fields: HashMap<String, DataType>,
    pub columns: HashMap<String, Vec<DataType>>,
    pub select_columns: Vec<String>,
}

impl Table {
    pub fn save(&self, mode: SaveMode, format: FileFormat) -> Result<(), TableErrors> {
        // Save logic here
        Ok(())
    }

    pub fn load(table_name: String, select_columns: Vec<String>, format: FileFormat) -> Result<Table, TableErrors> {
        // Load logic here
        Ok(Table {
            name: table_name,
            fields: HashMap::new(),
            columns: HashMap::new(),
            select_columns,
        })
    }
}

pub enum SaveMode {
    Overwrite,
}

pub enum FileFormat {
    SimpleColumnar,
}

pub enum TableErrors {
    TableAlreadyExists,
    WriteError(String),
    ColumnNotFound(String),
}
