use std::{any::type_name, io};

struct Person {
    name: String,
    number: u32,
}
fn main() {
    choose_an_option();
    let mut people: Vec<Person> = Vec::new();
    add_contact(&mut people);
    println!("Total contacts: {}", people.len());
}

fn choose_an_option() {
    println!("1. Add Contact\n2. View All Contacts\n3. Search Contact\n4. Exit ");

    loop {
        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
    
    
        match input.trim().parse() {
            Ok(1) => println!("1"),
            Ok(2) => println!("2"),
            Ok(3) => println!("3"),
            _ => println!("Please enter a correct option number"),
        }
    }   
}

fn add_contact(people: &mut Vec<Person>) {
    println!("Enter name:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read the input");

    let name = name.trim().to_string();
    println!("Enter phone:");

    let number: u32 = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");

        match input.trim().parse() {
        Ok(num) => break num,
        Err(_) => {
            println!("Please enter correct number");
            continue;
    }
        }
    };

    people.push(Person {name, number});

    println!("Contact added!");
}

// fn view_contact() {

// }