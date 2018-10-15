use std::io;

/*
 * Prints out a triangle from smallest to largest, skipping even rows. 
 * user inputs bottom number. Eg:
 * *
 * ***
 * ******
 */
fn main() {
    println!("Enter the number of stars to end your increasing triangle with.");    
    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("Failed to read line.");

    let mut num: u32 = num.trim().parse().expect("Please enter a number");

    let mut count: u32 = 1;

    for _n in 1u32..(num+1) {
        num +=  1;
        if _n % 2 !=0 {
            for _i in 0u32..count {
                print!("*");
            }
            print!("\n");
        }
        count += 1;
    }
}
