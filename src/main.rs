mod token;
mod repl;
mod lexer;
mod utils;

fn main() {
    println!("==============================Starting REPL==============================");
    println!(r"
    Welcome to the nipl repl...
    Type in valid commands and process them.
    Quit the terminal by entering a new line without any commands, or type 'quit'.
    ");
    println!("========================All rights reserved. 2023=========================");

    repl::repl::start();
}