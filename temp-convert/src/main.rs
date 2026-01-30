use std::io;

fn main() {
    println!("This program convert celsius to fahrenheit and vice versa.");

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

    let temp: f64 = temp.trim().parse().expect("Enter a valid number");

    let temp_unit = temp_unit.trim().to_lowercase();

    let result = match temp_unit.as_str() {
        "f" => temp * 9.0/5.0 + 32.0,
        "c" => (temp - 32.0) * 5.0 / 9.0,
        _ => {
            println!("Invalid unit. Use 'f' or 'c'");
            return;
        }
    };

    // if temp_unit == "f" || temp_unit == "F" {
    //     result = (temp as f64 * 9.0/5.0) + 32.0
    // } else if temp_unit == "c" || temp_unit == "C" {
    //     result = (temp as f64 - 32.0) * 5.0/9.0  
    // } else {
    //     println!("Please enter correct unit.");
    // }
    

    println!("Converted temperature: {:.2}", result);

}
