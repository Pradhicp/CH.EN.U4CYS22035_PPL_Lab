fn main() {
    let mut num = 1; // Start from 1

    while num <= 20 {
        if num % 2 != 0 { // Check if the number is odd
            println!("{}", num);
        }
        num += 1; // Increment number
    }
}

