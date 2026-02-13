use std::io;


fn main() {
    println!("Enter a sentence:");
    let sentence = read_sentence();
    analyze_sentence(&sentence);

}

fn read_sentence() -> String {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");

        let trimmed = input.trim();

        if trimmed.is_empty() {
            println!("Please write a sentence.")
        } else {
            return trimmed.to_string();
        }
    }
}

fn analyze_sentence(sentence: &str) {
    let char_count = sentence.chars().count();
    println!("Number of characters: {}", char_count);

    let mut words = sentence.split_whitespace();

    let first_word = words.next();
    let last_word = sentence.split_whitespace().last();
    let word_count = sentence.split_whitespace().count();

    println!("Number of words: {}", word_count);
    if let Some(word) = first_word {
        println!("First word: {}", word);
    } 

    if let Some(word) = last_word {
        println!("Last word: {}", word);
    }
}

fn sentence_input() -> String {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");

        let trimmed = input.trim();

        if trimmed.is_empty() {
            println!("Please write a sentence.");
        } else {
            println!("Number of characters: {}", trimmed.len());
            
            let words: Vec<&str> = trimmed.split_whitespace().collect();
            println!("Number of words: {}", words.len());
            println!("First word: {}", words[0]);
            let last_word = trimmed.split_whitespace().last().unwrap();
            println!("Last word: {}", last_word);
            return trimmed.to_string();    
        }   
    }
}

fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];
    
    for day in 0..12 {
        println!("\n0n the {} day of Christmas my true love sent to me:", days[day]);
    
        for gift in (0..=day).rev() {
            if day > 0 && gift == 0 {
                println!("and {}", gifts[gift]);
            } else {
                println!("{}", gifts[gift]);
            }
        }
    }
}











use std::io;

fn main() {
    println!("Enter how many Fibonacci numbers you want:");

    let n = read_number();

    print_fibonacci(n);
}

fn print_fibonacci(n: u32) {
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for _ in 0..n {
        print!("{}", a);
        let next = a + b;
        a = b;
        b = next
    }
    println!();
}

fn read_number() -> u32 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number"),
        }
    }
}
use std::io;

fn main() {
    println!("Temperature Converter");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("Choose an option:");

    let choice = read_number();

    match choice {
        1 => {
            println!("Enter temperature in Celsius:");
            let c = read_float();
            let f = celsius_to_fahrenheit(c);
            println!("{} °C = {:.2} °F", c, f);
        }

        2 => {
            println!("Enter temperature in Fahrenheit:");
            let f = read_float();
            let c = fahrenheit_to_celsius(f);
            println!("{} °F ={:.2} °C", f, c);
        }

        _ => println!("Invalid choice"),
    }

}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn read_number() -> u32 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Enter a valid number"),
        }
    }
}

fn read_float() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Enter a valid number"),
        }
    }
}


use rand::Rng;
fn main() {
    println!("Guess the number (1 to 100)");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");

        let guess = read_number();

        if guess < secret_number {
            println!("Too small!");
        } else if guess > secret_number {
            println!("Too big!");
        } else {
            println!("Correct! You guessed it.");
            break;
        }
    }
}

fn read_number() -> u32 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number"),
        }
    }
}

use std::io;

fn main() {
    expense_tracker();
}

fn read_number() -> u32 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number"),
        }
    }
}

fn expense_tracker() {
  println!("Enter 5 expenses one by one:");

  let mut total: u32 = 0;

  for _ in 0..5 {
    let expense = read_number();
    total += expense;
  }
   
    let average_expense = total/ 5;
    println!("Total Expense: {}", total);
    println!("Average Expense: {}", average_expense);
}



CLI Expense Splitter
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
Odd and Even
fn main() {
    odd_even(2);
    odd_even(4);
    odd_even(3);
    odd_even(56);
}

fn odd_even (num: u32) {
   if num % 2 == 0 {
    println!("The given number {num} is even");
   } else {
    println!("The given number {num} is odd");
   }
} 

Print numbers divisible by 3
fn main () {
    print_numbers();
}

fn print_numbers() {
    println!("Please enter the number till which you want the divisibles of 3's.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");

    let input_number = input.trim().parse::<i32>().expect("Not a valid Integer");   

   for i in 1..=input_number {
    if i % 3 == 0 {
        println!("{}", i);
    } 
   }
}