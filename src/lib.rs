use colored::*;
use rusqlite::Connection;
use std::process;

pub struct Todo {
    pub conn: Connection,
    pub todo_path: String,
}

pub struct TodoEntity {
    pub id: usize,
    pub title: String,
    pub done: bool,
}

impl Todo {
    pub fn new() -> Result<Self, String> {
        // let todo_path: String = match env::var("TODO_PATH") {
        //     Ok(t) => t,
        //     Err(_) => {
        //         let home = env::var("HOME").unwrap();

        //         // Look for a legacy TODO file path
        //         let legacy_todo = format!("{}/TODO", &home);
        //         match Path::new(&legacy_todo).exists() {
        //             true => legacy_todo,
        //             false => format!("{}/.todo", &home),
        //         }
        //     }
        // };

        let db = rusqlite::Connection::open("todo.db").unwrap();
        db.execute(
            "CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, done INTEGER DEFAULT 0)",
            (),
        )
        .unwrap();

        // Returns todo
        Ok(Self {
            conn: db,
            todo_path: "todo.db".to_string(),
        })
    }

    // Prints every todo saved
    pub fn list(&self) {
        let conn = &self.conn;
        let mut stmt = conn.prepare("SELECT id, title, done FROM todos").unwrap();

        let todos = stmt
            .query_map([], |row| {
                Ok(TodoEntity {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    done: row.get(2)?,
                })
            })
            .unwrap()
            .map(|r| r.unwrap()).collect::<Vec<TodoEntity>>();

        for (number, task) in todos.iter().enumerate() {
            let index = (number + 1).to_string().bold();
            if task.done {
                println!("{index} {}", task.title.strikethrough());
            } else {
                println!("{index} {}", task.title);
            }
        }
        if todos.len() == 0 {
            println!("{}", "No tasks to show.".red());
            println!("{}", "You can add one with todo add <tasks>.".bold());
        }
    }

    // This one is for yall, dmenu chads <3
    pub fn raw(&self, arg: &[String]) {
        if arg.len() > 1 {
            eprintln!("todo raw takes only 1 argument, not {}", arg.len())
        } else if arg.is_empty() {
            eprintln!("todo raw takes 1 argument (done/todo)");
        } else {
            let mut stmt = self
                .conn
                .prepare("SELECT id, title, done FROM todos")
                .unwrap();
            let todos = stmt
                .query_map([], |row| {
                    Ok(TodoEntity {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        done: row.get(2)?,
                    })
                })
                .unwrap()
                .map(|r| r.unwrap());

            for task in todos {
                if arg[0] == "todo" && !task.done {
                    println!("[ ] {}", task.title);
                } else if arg[0] == "done" && task.done {
                    println!("[*] {}", task.title);
                }
            }
        }
    }
    // Adds a new todo
    pub fn add(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo add takes at least 1 argument");
            process::exit(1);
        }

        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }

            self.conn
                .execute("INSERT INTO todos (title) VALUES (?)", (arg,))
                .unwrap();
        }
    }

    // Removes a task
    pub fn remove(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo rm takes at least 1 argument");
            process::exit(1);
        }

        let mut stmt = self
            .conn
            .prepare("SELECT id, title, done FROM todos")
            .unwrap();
        let mut todos = stmt
            .query_map([], |row| {
                Ok(TodoEntity {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    done: row.get(2)?,
                })
            })
            .unwrap()
            .map(|r| r.unwrap());

        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }
            let ind: usize = match arg.parse() {
                Ok(i) => i,
                Err(_) => {
                    eprintln!("Invalid index: {}", arg);
                    continue;
                }
            };
            let todo = todos.nth(ind - 1).unwrap();
            self.conn
                .execute("DELETE FROM todos WHERE id = ?", (todo.id,))
                .unwrap();
        }
    }
    // Clear todo by removing todo file
    pub fn reset(&self) {
        eprint!("Not supported yet");
    }
    pub fn restore(&self) {
        eprint!("Not supported yet");
    }

    // Sorts done tasks
    pub fn sort(&self) {
        eprint!("Not supported yet");
        // // Creates a new empty string
        // let newtodo: String;

        // let mut todo = String::new();
        // let mut done = String::new();

        // for line in self.todo.iter() {
        //     if line.len() > 5 {
        //         if &line[..4] == "[ ] " {
        //             let line = format!("{}\n", line);
        //             todo.push_str(&line);
        //         } else if &line[..4] == "[*] " {
        //             let line = format!("{}\n", line);
        //             done.push_str(&line);
        //         }
        //     }
        // }

        // newtodo = format!("{}{}", &todo, &done);
        // // Opens the TODO file with a permission to:
        // let mut todofile = OpenOptions::new()
        //     .write(true) // a) write
        //     .truncate(true) // b) truncrate
        //     .open(&self.todo_path)
        //     .expect("Couldn't open the todo file");

        // // Writes contents of a newtodo variable into the TODO file
        // todofile
        //     .write_all(newtodo.as_bytes())
        //     .expect("Error while trying to save the todofile");
    }

    pub fn done(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("todo done takes at least 1 argument");
            process::exit(1);
        }

        let mut stmt = self
            .conn
            .prepare("SELECT id, title, done FROM todos")
            .unwrap();
        let mut todos = stmt
            .query_map([], |row| {
                Ok(TodoEntity {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    done: row.get(2)?,
                })
            })
            .unwrap()
            .map(|r| r.unwrap());

        for i in args {
            let index: usize = i.parse().unwrap();
            let todo = todos.nth(index - 1).unwrap();
            if todo.done {
                self.conn
                    .execute("UPDATE todos SET done = 0 WHERE id = ?", (todo.id,))
                    .unwrap();
            } else {
                self.conn
                    .execute("UPDATE todos SET done = 1 WHERE id = ?", (todo.id,))
                    .unwrap();
            }
        }
    }
}

const TODO_HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - add [TASK/s]
        adds new task/s
        Example: todo add \"buy carrots\"
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX]
        removes a task
        Example: todo rm 4
    - reset
        deletes all tasks
    - restore 
        restore recent backup after reset
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort
    - raw [todo/done]
        prints nothing but done/incompleted tasks in plain text, useful for scripting
        Example: todo raw done
";
pub fn help() {
    // For readability
    println!("{}", TODO_HELP);
}
