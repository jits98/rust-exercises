use std::io;
fn main() {
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

fn choose_an_action() {
    let balance = initial_balance();
    println!("Choose an action:");
    println!("1. Deposit\n2. Withdraw\n3. Check Balance");
    
    loop {
        let mut input = String::new();
        
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");
    
    let amount = math_on_amount();
    
         match input.trim().parse() {
                Ok(1) => {
                    let sum =  balance + amount;
                    println!("Deposit Successful\nCurrent balance: {}", sum);
                    break;
                },
                Ok(2) => {
                    let sum =  balance - amount;
                    println!("Withdrawal Successful\nCurrent balance: {}", sum);
                    break;
                },
                Ok(3) => {
                    println!("Current Balance: {}", balance);
                    break;
                },
                _ => println!("Please enter correct option"),
                
        }
    }
}

fn math_on_amount() -> u32 {
    println!("Enter amount");

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