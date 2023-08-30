use std::io;
use std::io::Write;
use crate::lexer::lexer::Lexer;
use crate::token::token::TokenType;

const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        print!("{PROMPT}");
        // don't wait for a new line, flush the present print job to the output
        io::stdout().flush().unwrap();

        let mut input = String::new();
        // read the input line and panic if error in reading
        io::stdin().read_line(&mut input).expect("failed to read input");

        // using the ENTER key as the terminator
        if input == "\n" { return; }

        // start a lexer
        let mut lex = Lexer::new(input);
        // read the next token
        let mut tok = lex.next_token();
        // while there are tokens to read
        while tok.kind != TokenType::EOF {
            println!("{tok:?}");
            // read the next token
            tok = lex.next_token();
        }
    }
}