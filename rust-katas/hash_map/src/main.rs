/*
Using a hash map and vectors, create a text interface to allow a user to
add employee names to a department in a company. For example, “Add Sally to Engineering”
or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department
or all people in the company by department, sorted alphabetically.
*/

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
    let tokenized = input.split(" ");
    let mut first_token = input.split(" ").next();
    let mut instruction;

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

fn main() {
    print_instructions();

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.pop(); // strip newline

        match get_action(&input) {
            Action::ADD => println!("add"),
            Action::LIST => println!("list"),
            Action::BAD => print_instructions(),
            Action::QUIT => break,
        }
    }
}
