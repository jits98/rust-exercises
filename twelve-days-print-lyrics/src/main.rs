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
