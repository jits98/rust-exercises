use std::io;
// CLI Expense Splitter
fn main() {
    println!("Enter total bill amount:");

    let amount: u32 = loop {
        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input.");
    
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a valid number");
                
            }
        };
    };

    println!("Enter number of people:");
    let number_of_person: u32 = loop {

        let mut number_of_people = String::new();
    
        io::stdin()
            .read_line(&mut number_of_people)
            .expect("Failed to read the input.");
    
        match number_of_people.trim().parse() {
            Ok(0) => 
                println!("Number of people cannot be zero")
            ,
            Ok(num) => break num,
            Err(_) => {
                println!("Please enter a valid number")
        }
     };       
    };

    println!("Each person should pay:");
    let each_person_amount = amount / number_of_person;
    println!("{}", each_person_amount);
}
// Odd and Even
// fn main() {
//     odd_even(2);
//     odd_even(4);
//     odd_even(3);
//     odd_even(56);
// }

// fn odd_even (num: u32) {
//    if num % 2 == 0 {
//     println!("The given number {num} is even");
//    } else {
//     println!("The given number {num} is odd");
//    }
// } 

// Print numbers divisible by 3
// fn main () {
//     print_numbers();
// }

// fn print_numbers() {
//     println!("Please enter the number till which you want the divisibles of 3's.");

//     let mut input = String::new();

//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read the input");

//     let input_number = input.trim().parse::<i32>().expect("Not a valid Integer");   

//    for i in 1..=input_number {
//     if i % 3 == 0 {
//         println!("{}", i);
//     } 
//    }
// }