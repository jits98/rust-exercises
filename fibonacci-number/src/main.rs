
// Module import for taking input from the user
use std::io;

fn main() {
    println!("This is fibonacci number generator project.");

    println!("Please enter the first number from which the fibonacci series will start:");
    
    // taking fibonacci series starting number from the user
    let mut fibonacci_first_number = String::new();

    // reading the user input 
    io::stdin()
        .read_line(&mut fibonacci_first_number)
        .expect("Failed to read the line");

    println!("Please enter the second number for the fibonacci series:");
    
    // taking fibonacci series second number from the user
    
    let mut fibonacci_second_number = String::new(); 

    // reading the user input 
    io::stdin()
        .read_line(&mut fibonacci_second_number)
        .expect("Failed to read the line");  
    
    println!("Please enter the position number which you want you want to get the fibonacci number:");

    // Asking user for the position for which he/she want the fibonacci number
    let mut num_position = String::new();

    // reading the user input 
    io::stdin()
        .read_line(&mut num_position)
        .expect("Failed to read number.");

    num_position = num_position.trim().parse().expect("Not a valid integer");

    println!("You want the fibonacci number for the the position of: {}", num_position);


}
