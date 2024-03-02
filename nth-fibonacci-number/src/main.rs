use std::io;

fn main() {
    loop {
        println!("Type the nth position of the Fibonacci sequence you want:");

        let mut index: String = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = match index.trim().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };

        if index > 93 {
            println!("Position will likely cause overflow.");
            continue;
        }

        let number: usize = get_nth_fibonacci_value(index);

        println!("The Fibonacci number in position {index} is {number}.");
        break;
    }
}

fn get_nth_fibonacci_value(index: usize) -> usize {
    if index == 0 {
        return 0;
    }
    if index == 1 {
        return 1;
    }

    let mut last_number: usize = 0;
    let mut current_number: usize = 1;

    let mut current_index: usize = 2;

    while current_index <= index {
        let new_number: usize = last_number + current_number;

        last_number = current_number;
        current_number = new_number;

        current_index += 1;
    }

    current_number
}
