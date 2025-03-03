use std::{fs, io, usize};

#[derive(Debug)] enum Status{
    Pending,
    Completed
}

enum Operation{
    Add,
    Remove,
    Update,
    List
}

fn main() {


    let mut todo_list: Vec<(String, Status)> = Vec::new();
    load_todos_from_database(&mut todo_list);
    loop{
        print_choices();
        let mut line = String::from("");
        io::stdin().read_line(&mut line).expect("Enter some input");
        let choice: Result<i32, _> = line.trim().parse();
        let current_choice:Operation; 
        match choice {
            Ok(value) => {
                current_choice = match value{
                    1 => Operation::Add,
                    2 => Operation::Remove,
                    3 => Operation::Update,
                    4 => Operation::List,
                    5 => {
                        save_todos_in_database(&mut todo_list);
                        break;
                    },
                    _ => {
                        println!("-------- Select only from the choices available-------\n");
                        continue;
                    }
                }; 
                file_operations(current_choice, &mut todo_list);

            }
            Err(err) => {
                println!("-------- The error is {} --------\n", err);
                continue;
            }
        }
        println!("");
    }
}



fn print_choices(){
    println!("Please enter your choice");
    println!("1. Add a TODO");
    println!("2. Remove a TODO");
    println!("3. Update a TODO");
    println!("4. List all the TODOs");
    println!("5. Exit the program");
}

fn what_is_the_status() -> Status{
    loop {
        println!("\nPlease enter the status of this TODO : ");
        println!("1.Pending");
        println!("2.Completed");
        let mut line = String::from("");
        io::stdin().read_line(&mut line).expect("Enter some input");
        let choice: Result<i32, _> = line.trim().parse();
        match choice {
            Ok(value) => {
                match value{
                    1 => return Status::Pending,
                    2 => return Status::Completed,
                    _ => println!("-------- Select only from the choices available-------\n")
                } 
            }
            Err(err) => 
            {
                println!("-------- The error is {} --------\n", err);
                continue;
            }
        }
    }
}



fn file_operations(choice: Operation, todo_list: &mut Vec<(String, Status)>) {

    match choice {

        // adding a todo
        Operation::Add => {
            //take the description from the user
            let description: String = what_is_the_description();

            // take the status from the user 
            let choice:Status = what_is_the_status();

            todo_list.push((description,choice));

        },


        // remove a todo
        Operation::Remove => {
            let position:usize= what_todo(todo_list.len(), todo_list);
            let removed_element:(String, Status) = todo_list.remove(position - 1);
            println!("The todo removed is : \nDescription: {}\nStatus: {:?}\n", removed_element.0, removed_element.1);
        },


        // updating a todo
        Operation::Update => {
            let position:usize= what_todo(todo_list.len(), todo_list);

            // know what to change( description or status ) 
            let property_to_change:usize = what_to_change();

            if property_to_change == 1 {
                let description:String = what_is_the_description();
                todo_list[position - 1].0 = description;
            }
            if property_to_change == 2 {
                let choice:Status = what_is_the_status(); 
                todo_list[position - 1].1 = choice;
            }
        },


        // listing all the todos
        Operation::List => {
            for i in  0..todo_list.len(){
                print!("{}. ", i+1);
                println!("Description: {}", todo_list[i].0);
                println!("Status: {:?}\n", todo_list[i].1);
            }
        }

    } 
}

fn what_to_change() -> usize{
    loop {
        println!("please enter number of the property of the todo you want to change\n");
        println!("1.description");
        println!("2.status");
        let mut line = String::from("");
        io::stdin().read_line(&mut line).expect("enter some input");
        let choice: Result<usize, _> = line.trim().parse();
        match choice {
            Ok(value) => {
                match value{
                    1 => return 1,
                    2 => return 2,
                    _ => println!("-------- select only from the choices available-------\n")
                } 
            }
            Err(err) => 
            {
                println!("-------- the error is {} --------\n", err);
                continue;
            }
        }
    }
}

fn what_is_the_description() -> String {
    loop {
        println!("\nPlease enter the Description of the TODO : ");
        let mut line = String::from("");
        io::stdin().read_line(&mut line).expect("enter some input");
        let choice: Result<String, _> = line.trim().parse();
        match choice {
            Ok(value) => return value,
            Err(err) => 
            {
                println!("-------- the error is {} --------\n", err);
                continue;
            }
        }
    }

}

// what is the todo specified ( number of the todo i.e, position in the vector)
fn what_todo(todo_len: usize, todo_list: &mut Vec<(String, Status)>) -> usize{

    loop {
        file_operations(Operation::List, todo_list);
        println!("*Please enter the number of the todo you want to select\n");
        let mut line = String::from("");
        io::stdin().read_line(&mut line).expect("Enter some input");
        let choice: Result<usize, _> = line.trim().parse();
        match choice {
            Ok(value) => {
                if value <= todo_len as usize{
                    return value;
                }
            }
            Err(err) => 
            {
                println!("-------- The error is {} --------\n", err);
                continue;
            }
        }
    }

}

fn load_todos_from_database(todo_list: &mut Vec<(String, Status)>){
    let content = fs::read_to_string("./todos.txt");

    match content{
        Ok(value) => {
            for line in value.lines(){
                let words: Vec<&str> = line.split_whitespace().collect();

                if !words.is_empty(){
                    let last_word = words[words.len() - 1];

                    let description:String = if words.len() > 1{
                        words[..words.len() - 1].join(" ")
                    }else{
                        String::from("")
                    };

                    let status:Status;
                    if last_word == "Pending"{
                        status = Status::Pending;
                    }
                    else if last_word == "Completed"{
                        status = Status::Completed;
                    }
                    else{
                        status = Status::Pending;
                    }

                    todo_list.push((description, status));
                }
            }
        },
        Err(err) => 
        {
            println!("-------- The error occured while reading the databse is {} --------\n", err);
        }
    }
}

fn save_todos_in_database(todo_list: &mut Vec<(String, Status)>){
    let content = todo_list.iter()
        .map(|(desc, stat)| format!("{} {:?}\n", desc, stat))
        .collect::<String>();

    // write to the file
    let result = fs::write("./todos.txt", content);
    match result {
        Ok(_) => println!("The todo added successfully"),
        Err(err) => println!("The error occured while writing to the database is {}", err)
    }
}
