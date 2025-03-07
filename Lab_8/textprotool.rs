use std::io;

fn extract_word(sentence: &str, start: usize, end: usize) -> String {
    sentence[start..end].to_string()  
}
fn main() {
    let mut input = String::new();
    println!("Enter a sentence:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    input = input.trim().to_string(); 
    if input.len() < 4 {
        println!("Sentence too short to extract a word!");
        return;
    }
    let extracted_word = extract_word(&input, 0, 4);
    println!("Extracted word: {}", extracted_word);
    input.push_str(" This is a new addition!");

    println!("Modified sentence: {}", input);
    println!("Extracted word still valid: {}", extracted_word);
}

