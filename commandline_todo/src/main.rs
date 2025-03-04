use std::{io, usize};

use rusqlite::{Connection, Result};

enum Operation{
    Add,
    Remove,
    Update,
    List
}

fn main() -> Result<()>{

    // Connect to (or create) a database
    let conn = Connection::open("todos.db")?;

    // create the database

    conn.execute("
            CREATE TABLE IF NOT EXISTS todos(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                is_completed BOOLEAN NOT NULL DEFAULT 0
            )
        ",[])?;

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
                        break;
                    },
                    _ => {
                        println!("-------- Select only from the choices available-------\n");
                        continue;
                    }
                }; 
                file_operations(current_choice, &conn)?;

            }
            Err(err) => {
                println!("-------- The error is {} --------\n", err);
                continue;
            }
        }
        println!("");
    }

    Ok(())
}


fn print_choices(){
    println!("Please enter your choice");
    println!("1. Add a TODO");
    println!("2. Remove a TODO");
    println!("3. Update a TODO");
    println!("4. List all the TODOs");
    println!("5. Exit the program");
}




fn file_operations(choice: Operation, conn: &Connection) -> Result<()>{

    match choice {

        // adding a todo
        Operation::Add => {
            //take the description from the user
            let description: String = what_is_the_description();

            // take the status from the user 
            let choice:bool= what_is_the_status();

            conn.execute("INSERT INTO todos (description, is_completed) VALUES (?1,?2)", rusqlite::params![description, choice])?;
            Ok(())

        },


        // remove a todo
        Operation::Remove => {
            let position:usize= what_todo(conn);
            conn.execute("DELETE FROM todos WHERE id=?1", [position])?;
            Ok(())
        },


        // updating a todo
        Operation::Update => {
            let position:usize= what_todo(conn);

            // know what to change( description or status ) 
            let property_to_change:usize = what_to_change();

            if property_to_change == 1 {
                let description:String = what_is_the_description();
                conn.execute("UPDATE todos SET description=?1 WHERE id=?2", rusqlite::params![description, position])?;
            }
            if property_to_change == 2 {
                let choice:bool= what_is_the_status(); 
                conn.execute("UPDATE todos SET is_completed=?1 WHERE id=?2", rusqlite::params![choice, position])?;
            }
            Ok(())
        },


        // listing all the todos
        Operation::List => {
            return list_all_todos(conn);
        }

    } 
}





fn list_all_todos(conn: &Connection) -> Result<()>{
    let mut query = conn.prepare("SELECT * FROM todos")?;

    let todos = query.query_map([], |row|{
        Ok((
                row.get::<_,i32>(0)?,
                row.get::<_,String>(1)?,
                row.get::<_,bool>(2)?,
        ))
    })?;

    //print result of the query
    for todo in todos{
        let (id, desc, is_completed) = todo?;
        println!("id: {} DESCRIPTION: {} isCompleted: {}", id, desc, is_completed);
    }

    Ok(())

}

// what is the todo specified ( number of the todo i.e, position in the vector)
fn what_todo(conn: &Connection) -> usize{

    loop {
        list_all_todos(conn).expect("There is an error listing todos");
        println!("*Please enter the id of the todo you want to select\n");
        let mut line = String::from("");
        io::stdin().read_line(&mut line).expect("Enter some input");
        let choice: Result<usize, _> = line.trim().parse();
        match choice {
            Ok(value) => return value as usize,
            Err(err) => 
            {
                println!("-------- The error is {} --------\n", err);
                continue;
            }
        };
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

fn what_is_the_status() -> bool{
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
                    1 => return false,
                    2 => return true,
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
