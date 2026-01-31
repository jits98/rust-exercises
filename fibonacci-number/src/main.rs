// Module import for taking input from the user
use std::io;

fn main() {
    println!("This is fibonacci number generator project.");

    println!("Please enter the first number from which the fibonacci series will start:");
    
    let fibonacci_first_number : i32;
    loop {
        // taking fibonacci series starting number from the user
        let mut input = String::new();
        // reading the user input 
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line");
    
        match input.trim().parse::<i32>() {
            Ok(num) => {
                println!("You entered: {} for the first number in the fibonacci series", num);
                fibonacci_first_number = num;
                break;
            },
            Err(_) => {
                println!("Please enter a valid number: ");
            }     
        }           
    };

    println!("Please enter the second number for the fibonacci series:");

    let fibonacci_second_number : i32; 
    
    loop {      
        // taking fibonacci series second number from the user
        let mut input2 = String::new();
        // reading the user input 
        io::stdin()
            .read_line(&mut input2)
            .expect("Failed to read the line");         
    
        match input2.trim().parse::<i32> () {
            Ok(num2) => {
                println!("You entered: {} as the second number for the fibonacci series", num2);
                fibonacci_second_number = num2;
                break;
            },
            Err(_) => {
                println!("Please enter a valid number: ");
            }
        }         
    };
    
    println!("Please enter the position number which you want you want to get the fibonacci number:");

    // Asking user for the position for which he/she want the fibonacci number

    let num_position: i32;
    loop {      
        // taking fibonacci series second number from the user
        let mut input3 = String::new();
        // reading the user input 
        io::stdin()
            .read_line(&mut input3)
            .expect("Failed to read number.");       
    
        match input3.trim().parse::<i32> () {
            Ok(num3) => {
                println!("You entered: {} as position number for the fibonacci number", num3);
                num_position = num3;
                break;
            }
            Err(_) => {
                println!("Please enter a valid number: ");
            }
        }         
    };

    // Fibonacci number Output needs to change the logic here
    let mut fibonacci_number = 0; 
    while num_position <= num_position {
        fibonacci_number += fibonacci_first_number + fibonacci_second_number;
        println!("{}", fibonacci_number);
        
    }
    

}
