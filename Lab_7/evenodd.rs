use std::io;

fn main() {
    // Create a mutable String to store user input
    let mut input = String::new();

    // Prompt user for a number
    println!("Enter an integer:");
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Convert input string to integer
    let num: i32 = input.trim().parse().expect("Please enter a valid integer");

    // Check if the number is even or odd
    if num % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}

