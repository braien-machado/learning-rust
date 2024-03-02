use std::io;

fn main() {
    println!("Type the nth position of the Fibonacci sequence you want. (Positions beyond 93th will cause overflow)");

    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Position should be an unsigned integer value.");

    let number = get_nth_fibonacci_value(index);

    println!("The Fibonacci number in position {index} is {number}.")
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
        let new_number = last_number + current_number;

        last_number = current_number;
        current_number = new_number;

        current_index += 1;
    }

    current_number
}
