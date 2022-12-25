#![warn(clippy::all, clippy::pedantic)]

use::std::io::stdin;



fn main() {
    let mut your_name = String::new();

    
    println!("Hello, what's your name?");
    
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    println!("Hello, {}", your_name)
}
