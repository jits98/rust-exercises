// Module import for taking input from the user
use std::io;

fn main() {
    println!("This is fibonacci number generator project.");
    
    println!("Please enter the position number which you want you want to get the fibonacci number:");

    // Asking user for the position for which he/she want the fibonacci number

    let num_position: u32;
    
    loop {      
        // taking fibonacci series second number from the user
        let mut input3 = String::new();
        // reading the user input 
        io::stdin()
            .read_line(&mut input3)
            .expect("Failed to read number.");      
        match input3.trim().parse::<u32> () {
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

    const FIRST_NUMBER: u32 = 0;
        const SECOND_NUMBER: u32 = 1;
       
        let mut vec = Vec::new();

        vec.push(FIRST_NUMBER);
        vec.push(SECOND_NUMBER);
        vec.push(FIRST_NUMBER + SECOND_NUMBER);

        vec.push(SECOND_NUMBER + vec.get(2));
        println!("{:?}", vec);
        println!("{}", num_position);

    // Fibonacci number Output needs to change the logic here
   
    

}
