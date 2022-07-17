/*
Using a hash map and vectors, create a text interface to allow a user to
add employee names to a department in a company. For example, “Add Sally to Engineering”
or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department
or all people in the company by department, sorted alphabetically.
*/

use std::collections::HashMap;
use std::fmt::Write;
use std::io;

enum Action {
    ADD,
    LIST,
    QUIT,
    BAD,
}

fn print_instructions() {
    println!("Usage:");
    println!(" add [first_name] to [department]");
    println!(" list");
    println!(" quit");
}

fn get_action(input: &String) -> Action {
    let first_token = input.split(" ").next();
    let instruction;

    match first_token {
        None => return Action::QUIT,
        Some(v) => instruction = v,
    }

    match instruction {
        "add" => return Action::ADD,
        "list" => return Action::LIST,
        "quit" => return Action::QUIT,
        _ => return Action::BAD,
    }
}

fn add_name(input: &String, dept_mapping: &mut HashMap<String, String>) -> Result<String, String> {
    let name: String;
    let dept: String;
    let mut tokenized = input.split(" ");

    // Pull and ditch the instruction
    let mut next = tokenized.next();
    if next == None {
        return Err("Missing instruction".to_string());
    }

    // Pull the name
    next = tokenized.next();
    match next {
        None => return Err("Missing name".to_string()),
        Some(v) => name = String::from(v),
    }

    // Enforce "to" syntax
    next = tokenized.next();
    match next {
        None => return Err("Missing 'to'".to_string()),
        Some(v) => {
            if v != "to" {
                return Err("Missing 'to'".to_string());
            }
        }
    }

    // Pull the department
    next = tokenized.next();
    match next {
        None => return Err("Missing department".to_string()),
        Some(v) => dept = String::from(v),
    }

    dept_mapping.insert(name, dept);
    return Ok("Correct syntax".to_string());
}

fn get_list(mappings: &HashMap<String, String>) -> String {
    let mut result = String::from("Name -> Department\n");
    for (name, dept) in mappings {
        let _ignore = write!(result, "{} -> {}\n", name, dept);
    }

    return result;
}

fn main() {
    print_instructions();

    let mut mappings = HashMap::new();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.pop(); // strip newline

        match get_action(&input) {
            Action::ADD => match add_name(&input, &mut mappings) {
                Err(v) => {
                    println!("{}", v);
                    print_instructions();
                }
                Ok(_v) => println!("Successfully added"),
            },
            Action::LIST => println!("{}", get_list(&mappings)),
            Action::BAD => print_instructions(),
            Action::QUIT => break,
        }
    }
}
