fn extract_word(sentence: &str) -> &str { 
    let words: Vec<&str> = sentence.split_whitespace().collect(); 
    words[0] // Extracting the first word 
} 
 
fn main() { 
    let sentence = String::from("Rust is fast and safe."); 
     
    let word = extract_word(&sentence); 
    println!("Extracted word: {}", word); 
} 
