# Command-Line TODO Application

A simple, interactive command-line TODO application written in Rust. This application allows users to manage their tasks efficiently through a terminal interface.

## Features

- Add new tasks with descriptions and status (Pending/Completed)
- Remove existing tasks
- Update task descriptions and status
- List all tasks
- Persistent storage (currently using text file, planning to migrate to SQLite)
- Graceful exit handling (Ctrl+C support)

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/commandline_todo.git
cd commandline_todo

# Build the project
cargo build --release

# Run the application
cargo run

```

## Usage
The application provides a simple menu-driven interface:

1. Add a TODO - Create a new task with description and status
2. Remove a TODO - Delete an existing task
3. Update a TODO - Change description or status of a task
4. List all TODOs - View all your tasks
5. Exit - Save and quit the application

## Future Improvements
- <input disabled="" type="checkbox"> Migrate storage from text file to SQLite database for better data management
- <input disabled="" type="checkbox"> Enhance Ctrl-C handler implementation for cleaner application exit
- <input disabled="" type="checkbox"> Change input handling for numbers to use string matching (e.g., match on "1" instead of parsing integers)
- <input disabled="" type="checkbox"> Extend the Operation enum with additional functionality (such as filtering or sorting tasks)
- <input disabled="" type="checkbox"> Add due dates for tasks
- <input disabled="" type="checkbox"> Create task categories or tags
- <input disabled="" type="checkbox"> Add priority levels for tasks
- <input disabled="" type="checkbox"> Implement data backup and restore functionality
