use db_core::Database;

fn main() {
    let mut repl = Repl::new();
    repl.run();
}

pub struct Repl {
    buffer: String,
    database: Database,
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            buffer: String::new(),
            database: Database::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            println!(">> ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            self.buffer.push_str(&input);
            if self.buffer.contains(";") {
                if self.buffer.trim().to_lowercase() == "exit;" {
                    break;
                }
                match self.database.execute(self.buffer.trim().to_string()) {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("Error: {}", e),
                }
                self.buffer.clear();
            }
        }
    }
}
