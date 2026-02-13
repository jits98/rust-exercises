use std::io;

fn main() {
    choose_an_option();
}

fn choose_an_option() {
    println!("1. Add Contact\n2. View All Contacts\n3. Search Contact\n4. Exit ");

    loop {
        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
    
    
        match input.trim().parse() {
            Ok(1) => {
                add_contact();
                break;
            }
            Ok(2) => println!("2"),
            Ok(3) => println!("3"),
            _ => println!("Please enter a correct option number"),
        }
    }   
}

fn add_contact() {
    println!("Enter name:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read the input");

    println!("Enter phone:");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read the input");
    println!("Contact added!");
}