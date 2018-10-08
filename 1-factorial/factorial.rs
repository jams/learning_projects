use std::io;
use std::process;

// Write a programme which finds the factorial of a 
// number entered by the user. (check for all conditions).
fn main() {
    let mut input = String::new();
    let mut count = 0u64;
    let mut total = 1u64;

    println!("Enter the an integer [1, 12] whose factorial you want to calculate.");
        
    io::stdin().read_line(&mut input)
        .expect("Failed to read line.");
    
    let n: u64 =  input.trim().parse()
        .expect("N must be a number");
   
    if n <= 0 || n >= 13 {
        println!("Please enter a integer within subset [1, 12]. Exiting...");
        process::exit(1);
    }

    loop {
        total = total * (n - count);    
        count += 1;
        if count >= n-1 {
            break;
        }
    }

    println!("The result of {}! is {}.", n, total);

}
