use crate::token::token::{Token, TokenType};
use crate::utils;

// Lexer represents the lexer in tokenization
pub struct Lexer {
    pub input: Vec<char>,
    pub position: usize, // current position in the input (points to current char)
    pub read_position: usize, // current reading position in the input (after current char)
    pub current_char: Option<char>, // current char under examination
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Self {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            current_char: None,
        };
        l.read_char(); // point to the first char to read
        l
    }

    /// eat_whitespace skips any whitespace characters in the input string
    pub fn eat_whitespace(&mut self) {
        // while the current char is whitespace, read it and go to the next char
        while let Some(c) = self.current_char {
            match c {
                ' ' | '\t' | '\n' | '\r' => {
                    self.read_char();
                }
                _ => {
                    break;
                }
            }
        }
    }

    /// peek_char returns the next character in the token but DOES NOT advance the read or current position
    pub fn peek_char(&self) -> Option<char> {
        // if there is nothing more to read, return None else return the current character
        match self.read_position < self.input.len() {
            true => Some(self.input[self.read_position]),
            false => None,
        }
    }

    /// read_char reads the next character in the token and advances the read position
    pub fn read_char(&mut self) {
        // get the next character if it exists
        self.current_char = self.peek_char();
        // advance the position and read position
        self.position = self.read_position;
        self.read_position += 1;
    }

    /// read_identifier keeps reading a word until there is no longer a letter
    pub fn read_identifier(&mut self) -> String {
        let current_position = self.position;
        // while there is a letter to read, read it and move the read position
        while utils::is_letter_or_underscore(self.current_char.clone().unwrap()) {
            self.read_char();
        }
        self.input[current_position..self.position]
            .to_vec()
            .iter()
            .collect::<String>()
    }

    /// read_digit keeps reading a word until there is no longer a digit
    pub fn read_digit(&mut self) -> String {
        let current_position = self.position;
        // while there is a digit to read, read it and move the read position
        while utils::is_digit(self.current_char.clone().unwrap()) {
            self.read_char();
        }
        self.input[current_position..self.position]
            .to_vec()
            .iter()
            .collect::<String>()
    }

    /// lookup_identifier looks up an the identifier in the list of keywords
    pub fn lookup_identifier(&self, s: &str) -> TokenType {
        match s {
            "let" => TokenType::Let,
            "fn" => TokenType::Function,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ => TokenType::Ident(s.to_string())
        }
    }

    /// check_two_char_token_eq handles scenarios where the token is potentially a comparison token, e.g. == and !=
    fn check_two_char_token_eq(&mut self, single_kind: TokenType, double_kind: TokenType) -> Token {
        if self.peek_char().unwrap_or(0_u8 as char) == '=' {
            let curr_char = self.current_char.expect("next char should be '='");
            self.read_char(); // set current character to the next character after reading it
            return Token::new(double_kind, format!("{}{}", curr_char, self.current_char.expect("next char should be '='")))
        }
        return Token::new(single_kind, self.current_char.expect("next char should be '='"));
    }

    /// next_token returns the next token in the sequence
    pub fn next_token(&mut self) -> Token {
        // eat any whitespaces before processing the next character
        self.eat_whitespace();

        // if the current_char is None, return a token with the byte 0
        let ch = match self.current_char {
            Some(ch) => ch,
            None => return Token::new(TokenType::EOF, 0_u8 as char),
        };

        let tok = match ch {
            '=' => self.check_two_char_token_eq(TokenType::Assign, TokenType::Eq),
            '!' => self.check_two_char_token_eq(TokenType::Bang, TokenType::NotEq),
            '<' => self.check_two_char_token_eq(TokenType::LT, TokenType::LTE),
            '>' => self.check_two_char_token_eq(TokenType::GT, TokenType::GTE),
            '+' => Token::new(TokenType::Plus, ch),
            '-' => Token::new(TokenType::Minus, ch),
            '/' => Token::new(TokenType::Slash, ch),
            '*' => Token::new(TokenType::Asterisk, ch),
            ';' => Token::new(TokenType::Semicolon, ch),
            '(' => Token::new(TokenType::LParen, ch),
            ')' => Token::new(TokenType::RParen, ch),
            ',' => Token::new(TokenType::Comma, ch),
            '{' => Token::new(TokenType::LBrace, ch),
            '}' => Token::new(TokenType::RBrace, ch),
            _ => {
                // create a default illegal token
                let mut tok = Token::new(TokenType::Illegal, ch);
                // if the current char is a letter, read the whole word as an identifier
                if utils::is_letter_or_underscore(ch) {
                    // set the word as the literal
                    tok.set_literal_str(self.read_identifier());
                    // check if it is a keyword and set appropriately
                    tok.set_kind(self.lookup_identifier(tok.literal.as_str()));
                } else if utils::is_digit(ch) {
                    // set the digit as the literal
                    tok.set_literal_str(self.read_digit());
                    // set the type to be an integer
                    tok.set_kind(TokenType::Int(tok.literal.clone()))
                }
                return tok;
            }
        };

        // advance the read position
        self.read_char();

        tok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = String::from(
            r#"let _five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        10 <= 11;
        10 >= 9;
        "#,
        );

        let test_cases = vec![
            TokenType::Let,
            TokenType::Ident(String::from("_five")),
            TokenType::Assign,
            TokenType::Int(String::from("5")),
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident(String::from("ten")),
            TokenType::Assign,
            TokenType::Int(String::from("10")),
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident(String::from("add")),
            TokenType::Assign,
            TokenType::Function,
            TokenType::LParen,
            TokenType::Ident(String::from("x")),
            TokenType::Comma,
            TokenType::Ident(String::from("y")),
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::Ident(String::from("x")),
            TokenType::Plus,
            TokenType::Ident(String::from("y")),
            TokenType::Semicolon,
            TokenType::RBrace,
            TokenType::Semicolon,
            TokenType::Let,
            TokenType::Ident(String::from("result")),
            TokenType::Assign,
            TokenType::Ident(String::from("add")),
            TokenType::LParen,
            TokenType::Ident(String::from("five")),
            TokenType::Comma,
            TokenType::Ident(String::from("ten")),
            TokenType::RParen,
            TokenType::Semicolon,
            TokenType::Bang,
            TokenType::Minus,
            TokenType::Slash,
            TokenType::Asterisk,
            TokenType::Int(String::from("5")),
            TokenType::Semicolon,
            TokenType::Int(String::from("5")),
            TokenType::LT,
            TokenType::Int(String::from("10")),
            TokenType::GT,
            TokenType::Int(String::from("5")),
            TokenType::Semicolon,
            TokenType::If,
            TokenType::LParen,
            TokenType::Int(String::from("5")),
            TokenType::LT,
            TokenType::Int(String::from("10")),
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::Return,
            TokenType::True,
            TokenType::Semicolon,
            TokenType::RBrace,
            TokenType::Else,
            TokenType::LBrace,
            TokenType::Return,
            TokenType::False,
            TokenType::Semicolon,
            TokenType::RBrace,
            TokenType::Int(String::from("10")),
            TokenType::Eq,
            TokenType::Int(String::from("10")),
            TokenType::Semicolon,
            TokenType::Int(String::from("10")),
            TokenType::NotEq,
            TokenType::Int(String::from("9")),
            TokenType::Semicolon,
            TokenType::Int(String::from("10")),
            TokenType::LTE,
            TokenType::Int(String::from("11")),
            TokenType::Semicolon,
            TokenType::Int(String::from("10")),
            TokenType::GTE,
            TokenType::Int(String::from("9")),
            TokenType::Semicolon,
            TokenType::EOF,
        ];

        let mut l = Lexer::new(input);

        for test_case in test_cases {
            let tok = l.next_token();
            assert_eq!(tok.kind, test_case);
        }
    }
}
