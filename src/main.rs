use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;

fn main() {
    let mut input_buffer: String = String::with_capacity(8);  // should allocate 8 byte here 
    // let stdin: io::Stdin = io::stdin();

    print!("Please input the desired number.\n\n");
    print!("1st term = ");

    io::stdout().flush().unwrap();  // https://stackoverflow.com/questions/37531903/how-do-i-print-output-without-a-trailing-newline-in-rust

    io::stdin().read_line(&mut input_buffer).expect("Bruh");  // Error here very weird thats why its bruh err message

    let a: isize = input_buffer.trim().parse().expect("Why not number?");

    print!("2nd term = ");

    io::stdout().flush().unwrap();

    input_buffer.clear();  // clear the buffer first
    io::stdin().read_line(&mut input_buffer).expect("Bruh");

    let b: isize = input_buffer.trim().parse().expect("Why not number?");

    print!("nth term = ");

    io::stdout().flush().unwrap();
    
    input_buffer.clear();
    io::stdin().read_line(&mut input_buffer).expect("Bruh");

    let nth: usize = input_buffer.trim().parse().expect("Either not a number or you are asking for the negative nth term...");

    if nth.cmp(&3) == Ordering::Less {
        panic!("Did you just ask something weird?")
    }

    // match nth.cmp(&3) {
    //     Ordering::Less => panic!("Did you just ask something weird?"),
    //     _ => (),
    // };
    
    // fib is a + b + (a + b) + (b + (a + b))
    //        1   2      3           4
    // -2 is because 1st term and 2nd term is already there
    let mut last_term: isize = a;
    let mut current_term: isize = b;
    let mut next_term: isize;
    for current_nth in 0..(nth - 2) {  // in python, this `for _ in range(nth - 2):`.
        // next_term = last_term + current_term;
        next_term = match last_term.checked_add(current_term) {  // check of overflow https://stackoverflow.com/questions/52646755/checking-for-integer-overflow-in-rust
            Some(addition_result) => addition_result,
            None => panic!("Number too big now at {}", current_nth),
        };
        last_term = current_term;
        current_term = next_term;
    }

    print!("\nThe nth term for your Fibo seq. is {}\n", current_term)

}
