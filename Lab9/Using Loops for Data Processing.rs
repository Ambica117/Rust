fn main() { 
    let n = 10; // Limit for Fibonacci numbers 
    let mut fibonacci = vec![0, 1]; // Starting values of Fibonacci sequence 
 
    for i in 2..n { 
        let next = fibonacci[i - 1] + fibonacci[i - 2]; 
        fibonacci.push(next); 
    } 
 
    println!("Fibonacci sequence up to {}: {:?}", n, fibonacci); 
} 

