use std::io;

fn main() {
    let mut numbers = Vec::new();

    println!("Enter numbers (enter 0 to stop):");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: i32 = input.trim().parse().expect("Please enter a valid number");

        if num == 0 {
            break;
        }
        numbers.push(num);
    }

    println!("Even numbers from the list:");
    
    let mut iter = numbers.into_iter();
    while let Some(num) = iter.next() {
        if num % 2 == 0 {
            println!("{}", num);
        }
    }
}

