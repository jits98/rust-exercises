use std::io;

fn main() {
    println!("In Which temperature unit do you want to convert in 'fahrenheit' or 'celsius' write 'f' for fahrenheit and 'c' for celsius?");

    let mut temp_unit = String::new();
    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp_unit)
        .expect("Failed to read input");

    println!("Please write the number you want to convert:");
    
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input");

    let temp: i32 = temp.trim().parse().expect("Not a valid number");

    let mut result:f64 = 0.0;

    let temp_unit = temp_unit.trim();

    if temp_unit == "f" || temp_unit == "F" {
        result += (temp * 9/5) as f64 + 32.0
    } else if temp_unit == "c" || temp_unit == "C" {
        result += (temp - 32) as f64 * 5.0/9.0  
    } else {
        println!("Please enter correct unit.");
    }
    
    println!("Here is your temperature unit: {}", temp_unit);
    println!("Here is your temperature number: {}", temp);

    println!("Here is your converted answer: {}", result);

}
