use std::io;
fn main() {
    println!("Enter a positive integer:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");


    let n: u32 = input.trim().parse().expect("Please type a positive integer!");

    let mut m = 0;
    let mut num_n = 1;
    let mut num_n_minu_1 = 0;
    let mut num_n_plu_1 = 1;
    while m < n {
        num_n_plu_1 = num_n + num_n_minu_1; //makes fib output
        num_n_minu_1 = num_n; //moves to next n-1 in seq
        num_n = num_n_plu_1; // moves to next n in seq
        m = m +1; //track position in fib seq
    }
    println!("The {}th number in the fib sequence is: {}", n, num_n_plu_1); 

}
