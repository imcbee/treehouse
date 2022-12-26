#![warn(clippy::all, clippy::pedantic)]

use ::std::io::stdin;

const VISITOR_LIST: [&str; 3] = ["bert", "steve", "ian"];


fn what_is_your_name() -> String {
    let mut your_name = String::new();
    let mut allow_them_in = false;

    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    for visitor in &VISITOR_LIST {
        if visitor == &your_name {
            allow_them_in = true;
            println!("it is working");
        }
        println!("{}", visitor);
    }
    println!("{}", allow_them_in);

    if allow_them_in {
        println!("Welcome! to the treehouse, {}", your_name);
    }else{
        println!("Sorry, you are not on the list");
    }

    your_name.trim().to_lowercase()
}

fn main() {

    println!("Hello, what's your name?");
    let name = what_is_your_name();

    // for i in 0..10 {
    //     println!("{}", i)
    // }

    println!("Hello, {:?}", name)
    
}
