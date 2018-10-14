use std::io;
/*
 *    Write a programme that prints out a triangle from largest to smallest; 
 *    user inputs the largest number. Eg:
 *    *****
 *    ****
 *    ***
 *    **
 *    *
 */

fn main() {

    println!("Enter the number of stars to start your decreasing triangle: ");
    let mut num = String::new();
    
    io::stdin().read_line(&mut num).expect("Failed to read line.");

    let mut num: u32 = num.trim().parse().expect("Please type a number.");
    
    num += 1;

    // First loops 'num' number of rows
    // Then prints 'num' number of *
    for _n in (1u32..num).rev() {
        num -= 1;
        for _m in (0u32..num) {
            print!("*");
        }
        print!("\n");
    }
}
