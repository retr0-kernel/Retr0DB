# Retr0DB

[Blog](https://medium.com/@krizzsrivastava/retr0db-building-a-database-in-rust-b223e2b98cbd)

Retr0DB is a lightweight, Rust-based custom database designed for educational purposes and small-scale applications. It features a simple client-server architecture, allowing users to create tables, insert data, and query data through a basic REPL interface.

## Features

- **Lightweight Database Engine**: Core database functionalities implemented in Rust for optimal performance.
- **Client-Server Architecture**: Separate server and client modules communicate over TCP, enabling distributed usage scenarios.
- **REPL Interface**: Interactive command-line interface for executing SQL-like commands directly.
- **Extensible Framework**: Easily extendable codebase for adding new features like new SQL commands or improving the existing query parser.

## Components

- `db_core`: The core library that handles database operations such as table creation, data insertion, and querying.
- `db_server`: A TCP server that listens for commands from clients and uses `db_core` to execute them.
- `db_client`: A simple client that connects to the `db_server` and sends commands for execution.
- `db_parser`: A custom parser built using LALRPOP to interpret a subset of SQL-like commands.

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version recommended)
- Git

### Installation

1. **Clone the Repository**

    ```bash
    git clone https://github.com/yourusername/Retr0DB.git
    cd Retr0DB
    ```

2. **Build the Project**

    To build all components, run:

    ```bash
    cargo build
    ```

3. **Run the Server**

    In one terminal window, start the server:

    ```bash
    cargo run --bin db_server
    ```

4. **Run the Client**

    In another terminal window, start the client:

    ```bash
    cargo run --bin db_client
    ```

### Configuration

Currently, the server listens on `localhost:7878`. This can be configured in the `db_server/src/main.rs` file if needed.

## Usage

After starting both the server and the client, you can use the client's REPL to send commands to the server. Here are some example commands:

- Create a table:
  ```sql
  create table users;

- Insert data into the table:
  ```sql
  insert into users (name, age) values ('Krish', 20);

- Query data:
  ```sql
  select name, age from users;
