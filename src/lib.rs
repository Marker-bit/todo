use std::io::{BufReader, Write};
use std::io::prelude::*;
use std::fs::OpenOptions;
use colored::*;

pub struct Todo {
    pub todo: Vec<String>,
}

impl Todo {
    pub fn new () -> Result<Self,String> {

        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open("TODO")
            .expect("Couldn't open the todofile");

        // Creates a new buf reader
        let mut buf_reader = BufReader::new(&todofile);

        // Empty String ready to be filled with TODOs
        let mut contents = String::new();

        // Loads "contents" string with data
        buf_reader.read_to_string(&mut contents).unwrap();

        // Splits contents of TODO file into a todo vector
        let todo = contents.to_string().lines().map(str::to_string).collect();
        
        // Returns todo
        Ok(Self{todo})
    }


    // Prints every todo
    pub fn list (&self) {
        
        // This loop will repeat itself for each taks in TODO file
        for (number, task) in self.todo.iter().enumerate() {
           
            if task.len() > 5 {
            // Converts virgin default number into a chad BOLD string
            let number = (number+1).to_string().bold();

            // Saves the symbol of current task
            let symbol = &task[..4];
            // Saves a task without a symbol
            let task = &task[4..];

            // Checks if the current task is completed or not...
            if symbol == "[*] " {
                // DONE
                
                //If the task is completed, then it prints it with a strikethrough 
                println!("{} {}",number, task.strikethrough()); 
            } else if symbol == "[ ] " {
                // NOT DONE

                //If the task is not completed yet, then it will print it as it is
                println!("{} {}",number , task);
            }

            } 
        }
    }
  

    // This one is for yall, dmenu chads <3
    pub fn raw (&self, arg: &[String]) {
       
        
        if arg.len() > 1 {
            eprintln!("todo raw takes only 1 argument, not {}", arg.len())
        } else if arg.len() < 1 {
            eprintln!("todo raw takes 1 argument (done/todo)");
        } else {
            
        
        
        // This loop will repeat itself for each taks in TODO file
        for task in self.todo.iter() {
           
            if task.len() > 5 {

            // Saves the symbol of current task
            let symbol = &task[..4];
            // Saves a task without a symbol
            let task = &task[4..];

            // Checks if the current task is completed or not...
            if symbol == "[*] " && arg[0] == "done" {
                // DONE
                //If the task is completed, then it prints it with a strikethrough 
                println!("{}", task); 
            } else if symbol == "[ ] " && arg[0] == "todo" {
                // NOT DONE

                //If the task is not completed yet, then it will print it as it is
                println!("{}", task);
            }

            } 
        }
    }
    }
    // Adds a new todo
    pub fn add (&self, args: &[String]) {
        
        // Opens the TODO file with a permission to:
        let mut todofile = OpenOptions::new()
            .create(true) // a) create the file if it does not exist 
            .append(true) // b) append a line to it
            .open("TODO")
            .expect("Couldn't open the todofile");

        let mut newtodo = String::new();
        
        for arg in args {
            let line = format!("[ ] {}\n", arg);
            newtodo.push_str(&line);
        }
        
        // Appends a new task/s to the file
        writeln!(todofile,"{}", newtodo).unwrap();
    }

    // Removes a task
    pub fn remove (&self, args: &[String]) {
        

        // Creates a new empty string
        let mut newtodo = String::new();

   
        
        for (pos, line) in self.todo.iter().enumerate() {
            if args.contains(&(pos+1).to_string()) {
                continue;
            }
           
            let line = format!("{}\n", line);
            newtodo.push_str(&line[..]);
        }
        
        // Opens the TODO file with a permission to:
        let mut todofile = OpenOptions::new()
            .write(true) // a) write
            .truncate(true) // b) truncrate
            .open("TODO")
            .expect("Couldn't open the todo file");
        
        // Writes contents of a newtodo variable into the TODO file 
        todofile.write_all(newtodo.as_bytes())
            .expect("Error while trying to save the todofile");
//        write!(&self.todofile, "{}", newtodo).unwrap();
    }


    // Sorts done tasks
    pub fn sort (&self) {
        

        // Creates a new empty string
        let newtodo: String;

        let mut todo = String::new(); 
        let mut done = String::new(); 

        for line in self.todo.iter() {
            if line.len() > 5 {
                if &line[..4] == "[ ] " {
                    let line = format!("{}\n", line);
                    todo.push_str(&line); 
                } else if &line[..4] == "[*] " {

                    let line = format!("{}\n", line);
                    done.push_str(&line); 
                }
            }
        }
        
        newtodo = format!("{}{}",&todo,&done);
        // Opens the TODO file with a permission to:
        let mut todofile = OpenOptions::new()
            .write(true) // a) write
            .truncate(true) // b) truncrate
            .open("TODO")
            .expect("Couldn't open the todo file");
        
        // Writes contents of a newtodo variable into the TODO file 
        todofile.write_all(newtodo.as_bytes())
            .expect("Error while trying to save the todofile");
    }

    pub fn done (&self, args: &[String]) {
        

        // Creates a new empty string
        let mut newtodo = String::new();

   
        for (pos, line) in self.todo.iter().enumerate() {
            if line.len() > 5 {
                    if  args.contains(&(pos+1).to_string()){

                    if &line[..4] == "[ ] "{
                        let line = format!("[*] {}\n", &line[4..]);
                        newtodo.push_str(&line[..]);
                    } else if &line[..4] == "[*] " {
                        let line = format!("[ ] {}\n", &line[4..]);
                        newtodo.push_str(&line[..]);
                    }
        
                } else {
                    if &line[..4] == "[ ] " || &line[..4] == "[*] " {
                        let line = format!("{}\n", line);
                        newtodo.push_str(&line[..]);
                    }
                } 
            }
        }
        
        // Opens the TODO file with a permission to overwrite it
        let mut f = OpenOptions::new()
            .write(true) 
            .open("TODO")
            .expect("Couldn't open the todofile");
        
        // Writes contents of a newtodo variable into the TODO file 
        f.write_all(newtodo.as_bytes()).expect("Error while trying to save the todofile");
//        write!(&self.todofile, "{}", newtodo).unwrap();
    }

}


pub fn help() {
    println!(
"Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - new [TASK/s] 
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
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort 
    - raw [todo/done]
        prints nothing but done/incompleted tasks in plain text, useful for scripting
        Example: todo raw done
        ");
}
