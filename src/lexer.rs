pub struct Lexer {
    input: String,
    position: usize,
    current_char: Option<char>,
}
#[derive(Debug, PartialEq)]
pub enum Token {
    Define,
    Identifier(String),
    Label(String),
    Number(u8),
    Register(String),
    Instruction(String),
    Comment(String),
    Semicolon,
    Eof,
}
impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            current_char: None,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = self.input[self.position..].chars().next();
        }
        self.position += 1;
    }

    fn peek_char(&self) -> Option<char> {
        if self.position >= self.input.len() {
            None
        } else {
            self.input[self.position..].chars().next()
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_comment(&mut self) -> String {
        let mut comment = String::new();
        while let Some(c) = self.current_char {
            if c == '\n' {
                break;
            }
            comment.push(c);
            self.read_char();
        }
        comment
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' || c == '.' {
                ident.push(c);
                self.read_char();
            } else {
                break;
            }
        }
        ident
    }

    fn read_number(&mut self) -> u8 {
        let mut number = String::new();
        while let Some(c) = self.current_char {
            if c.is_digit(10) {
                number.push(c);
                self.read_char();
            } else {
                break;
            }
        }
        number.parse::<u8>().unwrap_or(0) // Default to 0 if parsing fails
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.current_char {
            Some(';') => {
                self.read_char();
                Token::Semicolon
            }
            Some('#') | Some('/') => {
                let comment = self.read_comment();
                Token::Comment(comment)
            }
            Some(c) if c.is_alphabetic() || c == '.' => {
                let ident = self.read_identifier();
                match ident.as_str() {
                    "ldi" | "jmp" | "add" | "mov" | "nop" | "halt" => Token::Instruction(ident),
                    ".define" => Token::Define,
                    _ => Token::Identifier(ident),
                }
            }
            Some(c) if c.is_digit(10) => {
                let number = self.read_number();
                Token::Number(number)
            }
            Some(c) if c.is_alphabetic() => {
                let ident = self.read_identifier();
                ident
                    .to_string()
                    .strip_prefix("r")
                    .and_then(|s| s.parse::<usize>().ok())
                    .filter(|&n| n < 16)
                    .unwrap_or_else(|| panic!("Invalid register: {}", ident));
                Token::Register(ident)
            }
            None => Token::Eof,
            _ => {
                self.read_char(); // Skip unknown characters
                self.next_token()
            }
        };

        self.skip_whitespace(); // Skip any whitespace after the token
        token
    }
}
