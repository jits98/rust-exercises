// Module import for taking input from the user
use std::io;

fn main() {
    println!("This is fibonacci number generator project.");

    println!("Please enter the first number from which the fibonacci series will start:");
    
    // taking fibonacci series starting number from the user

    loop {

        let mut fibonacci_first_number = String::new();

        // reading the user input 
        io::stdin()
            .read_line(&mut fibonacci_first_number)
            .expect("Failed to read the line");
    
        match fibonacci_first_number.trim().parse::<i32>() {
            Ok (fibonacci_first_number) => {
                println!("You entered: {} for the first number in the fibonacci series", fibonacci_first_number);
                break;
            }
            Err(_) => {
                println!("Please enter a valid number: ");
            }
        }           
    };

    println!("Please enter the second number for the fibonacci series:");

    loop {      
        // taking fibonacci series second number from the user
        let mut fibonacci_second_number = String::new(); 

        // reading the user input 
        io::stdin()
            .read_line(&mut fibonacci_second_number)
            .expect("Failed to read the line");         
    
        match fibonacci_second_number.trim().parse::<i32> () {
            Ok(fibonacci_second_number) => {
                println!("You entered: {} as the second number for the fibonacci series", fibonacci_second_number);
                break;
            }
            Err(_) => {
                println!("Please enter a valid number: ");
            }
        }         
    };
    
    println!("Please enter the position number which you want you want to get the fibonacci number:");

    // Asking user for the position for which he/she want the fibonacci number

    loop {      
        // taking fibonacci series second number from the user
        let mut num_position = String::new();

        // reading the user input 
        io::stdin()
            .read_line(&mut num_position)
            .expect("Failed to read number.");       
    
        match num_position.trim().parse::<i32> () {
            Ok(num_position) => {
                println!("You entered: {} as position number for the fibonacci number", num_position);
                break;
            }
            Err(_) => {
                println!("Please enter a valid number: ");
            }
        }         
    };

}
