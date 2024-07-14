use db_core::Database;

pub struct Server {
    database: Database,
}

impl Server {
    pub fn new() -> Self {
        Server {
            database: Database::new(),
        }
    }

    pub fn handle_request(&mut self, query: String) -> String {
        match self.database.execute(query) {
            Ok(result) => result,
            Err(e) => format!("Error: {}", e),
        }
    }
}

