fn main() {
    for day in 1..=12 {
        print_day_intro(day);
        print_gifts(day);
        println!();
    }
}

fn print_day_intro(day: u32) {
    let day_name = match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    };

    println!(
        "On the {} day of Christmas my true love sent to me:", day_name
    );
}

fn print_gifts(day: u32) {
    for gifts in (1..=day).rev() {
        match gift {
            12 => println!("Twelve drummers drumming,"),
            11 => println!("Eleven pipers piping,"),
            10 => println!("Ten lords a-leaping,"),
            9 => println!("Nine ladies dancing,"),
            8 => println!("Eight maids a-milking,"),
            7 => println!("Seven swans a-swimming,"),
            6 => println!("Six geese a-laying,"),
            5 => println!("Five gold rings,"),
            4 => println!("Four calling birds,"),
            3 => println!("Three french hens,"),
            2 => println!("Two turtle doves,"),
            1 => {
                if day == 1 {
                    println!("A partridge in a peer tree.");
                } else {
                    println!("And a partridge in a pear tree.")
                }
            }
            _ => {}
        }
    }
}
