use std::io;
fn main() {
    initial_balance();
    choose_an_action();
}

fn initial_balance() -> u32 {
    println!("Enter initial balance:");

    loop {

        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
    
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number"),
        }
    }
}

fn choose_an_action() -> u32 {
    println!("Choose an action:");
    println!("1. Deposit\n2. Withdraw\n3. Check Balance");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");

        match input.trim().parse() {
                1 => println!("Deposit successful"),
                2 => println!("Withdrawal successful"),
                3 => println!("Balance check"),
                _ => println!("Please enter correct option"),
        }
    }
}