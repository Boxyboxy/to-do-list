mod task;
pub use task::Task;
use std::{ process, sync::atomic::{ AtomicU64, self } };

static UNIQUE_ID: AtomicU64 = AtomicU64::new(1);

fn display_todo(todo_list: &Vec<Task>) {
    if todo_list.len() < 1 {
        println!("Empty todo list");
        return;
    }
    println!("Your todo list:");
    for item in todo_list {
        println!("id: {}, name: {}, done: {}", item.id, item.task, item.done_status);
    }
}

fn add_new_task(todo_list: &mut Vec<Task>, task_string: &str) {
    let id_no = UNIQUE_ID.fetch_add(1, atomic::Ordering::SeqCst);

    let task: Task = Task {
        task: task_string.into(), // passes ownership of string to task struct
        done_status: false,
        id: id_no,
    };

    todo_list.push(task);

    println!("{} added to the todo list: ", task_string);
}

fn remove_task(todo_list: &mut Vec<Task>, id_no: u64) {
    todo_list.retain(|task| task.id != id_no);
}

fn get_task(todo_list: &mut Vec<Task>, task_id: u64) -> Result<&mut Task, &str> {
    for task in todo_list {
        if task.id == task_id {
            return Ok(task);
        } else {
            continue;
        }
    }

    return Err("Task not found in todo list");
}

pub fn run(args: Vec<&str>, todo: &mut Vec<Task>) {
    parse_arguments(args, todo);
}

fn display_help() {
    let help: &str =
        "
        Welcome to the todo_list application. 
        structure of query: 
            command [arguments] 

        supported commands: 
            add - Adds a new task to the todo list. Please input your task as a string in the 2nd argument. The task string should NOT be space separated. 
            Command usage: >add task_string

            show - Displays the todo list 
            Command usage: >show

            delete - Deletes a task from the todo list, please provide the task_id as the 2nd argument. 
            Command usage: >delete task_id

            update - Updates the name of a task. Please input the task id as the 2nd argument followed by the new task string as the 3rd argument. 
            Command usage: >update task_id new_task_string 

            done - Updates the task as done. Please input the task id as the 2nd argument.     
            Command usage: >done task_id 

            exit - Exits the program. 
            Command usage: >exit

            help - Displays instructions on how to use this tool. 
            Command usage: >help 
        
        arguments: 
            task_id: Unique id assigned to each task. 

            task_string: String representing the task provided by the user. ";

    println!("{}", help);
}

fn parse_arguments(args: Vec<&str>, todo_list: &mut Vec<Task>) {
    let command = args[0];

    match command {
        "add" => {
            if let Some(value) = args.get(1) {
                let new_task = *value;
                add_new_task(todo_list, new_task);
                display_todo(todo_list);
            } else {
                println!("please provide a new name for the task");
            }
        }

        "show" => {
            display_todo(todo_list);
        }

        "delete" => {
            match &args[1].parse::<u64>() {
                Ok(value) => {
                    remove_task(todo_list, *value);
                    display_todo(todo_list);
                }

                Err(message) => {
                    println!("{}", message.to_string());
                }
            }
        }

        "update" => {
            // possibility 1: id parsing error
            match &args[1].parse::<u64>() {
                Ok(value) => {
                    // possibility 2: task getting error
                    if let Ok(task) = get_task(todo_list, *value) {
                        // possibility 3: no third argument provided.
                        if let Some(value) = args.get(2) {
                            let new_task = *value;
                            task.update_task(new_task.into());
                        } else {
                            println!("no new task provided");
                        }
                    } else {
                        println!("task not found in todo list");
                    }
                }

                Err(message) => {
                    print!("{}", message);
                }
            }
        }

        "done" => {
            match &args[1].parse::<u64>() {
                Ok(value) => {
                    if let Ok(task) = get_task(todo_list, *value) {
                        task.update_status();
                    } else {
                        println!("task id not found in list");
                    }
                }
                Err(message) => {
                    println!("{}", message.to_string());
                }
            }
        }

        "exit" => {
            process::exit(0);
        }

        "help" | _ => {
            display_help();
        }
    }
}
