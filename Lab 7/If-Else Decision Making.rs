use std::io; 

 

fn main() { 

    // Take input from the user 

    let mut input = String::new(); 

    println!("Enter a number: "); 

    io::stdin().read_line(&mut input).expect("Failed to read line"); 

    let num: i32 = input.trim().parse().expect("Please enter a valid number"); 

 

    // Check if the number is positive, negative, or zero 

    if num > 0 { 

        println!("Positive"); 

    } else if num < 0 { 

        println!("Negative"); 

    } else { 

        println!("Zero"); 

    } 

} 
