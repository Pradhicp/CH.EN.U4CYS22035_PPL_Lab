use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter the number of Fibonacci terms:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Please enter a valid number");

    let mut fib_sequence = Vec::new();

    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        fib_sequence.push(a);
        let next = a + b;
        a = b;
        b = next;
    }

    println!("Fibonacci Sequence: {:?}", fib_sequence);
}

