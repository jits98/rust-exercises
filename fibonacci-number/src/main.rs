// Module import for taking input from the user
use std::io;

fn main() {
    println!("This is fibonacci number generator project.");
    
    println!("Please enter the position you want in Fibonacci series:");

    // Asking user for the position for which he/she want the fibonacci number
    let num_position: usize;
    
    loop {      
        // taking fibonacci series second number from the user
        let mut input = String::new();
        // reading the user input 
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");  

        match input.trim().parse::<usize>() {
            Ok(n) if n > 0 => {   
                num_position = n;
                break;
            }
            _ => {
                println!("Please enter a valid positive number: ");
            }
        }         
    };

    // Fibonacci number Output needs to change the logic here

    if num_position == 1 {
        println!("Fibonacci number at position 1 is 0");
        return;
    } 
    if num_position == 2 {
        println!("Fibonacci number at position 2 is 1");
        return;
    }
   
    

}
