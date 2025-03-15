use std::io; 
 
fn main() { 
    let mut numbers = Vec::new(); 
    let mut input = String::new(); 
 
    println!("Enter numbers, one per line. Enter 0 to stop:"); 
 
    loop { 
        io::stdin().read_line(&mut input).expect("Failed to read line"); 
        let num: i32 = input.trim().parse().expect("Please enter a valid number"); 
 
        if num == 0 { 
            break; 
        } 
 
        numbers.push(num); 
        input.clear(); 
    } 
 
    // Print only even numbers using while let 
    let mut iter = numbers.into_iter(); 
    while let Some(num) = iter.next() { 
        if num % 2 == 0 { 
            println!("{}", num); 
        } 
    } 
} 

