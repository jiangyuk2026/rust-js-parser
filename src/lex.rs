use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    LF,
    CR,

    Var,
    Let,
    Const,
    True,
    False,
    Undefined,
    Null,
    Await,
    Async,
    Function,
    With,
    If,
    Else,
    Switch,
    Case,
    Break,
    Continue,
    For,
    In,
    Do,
    While,
    Return,
    Try,
    Catch,
    Finally,
    Throw,
    New,
    This,
    Instanceof,
    Typeof,
    Class,
    Void,
    Yield,
    Debugger,

    Variable(String),
    Digit(String),
    String(String),
    Control(String),
    EOF,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Control(s) => {
                write!(f, "{}", s)
            }
            Token::Variable(s) => {
                write!(f, "{}", s)
            }
            Token::Digit(s) => {
                write!(f, "{}", s)
            }
            Token::String(_) => write!(f, "String"),
            Token::Var => write!(f, "Var"),
            Token::Let => write!(f, "Let"),
            Token::Const => write!(f, "Const"),
            Token::Undefined => write!(f, "Undefined"),
            Token::Null => write!(f, "Null"),
            Token::Await => write!(f, "Await"),
            Token::Async => write!(f, "Async"),
            Token::Function => write!(f, "Function"),
            Token::With => write!(f, "With"),
            Token::If => write!(f, "If"),
            Token::Switch => write!(f, "Switch"),
            Token::Case => write!(f, "Case"),
            Token::Break => write!(f, "Break"),
            Token::Continue => write!(f, "Continue"),
            Token::For => write!(f, "For"),
            Token::While => write!(f, "While"),
            Token::Return => write!(f, "Return"),
            Token::EOF => write!(f, "EOF"),
            Token::True => write!(f, "True"),
            Token::False => write!(f, "False"),
            Token::Else => write!(f, "Else"),
            Token::In => write!(f, "In"),
            Token::Do => write!(f, "Do"),
            Token::Try => write!(f, "Try"),
            Token::Catch => write!(f, "Catch"),
            Token::Finally => write!(f, "Finally"),
            Token::Throw => write!(f, "Throw"),
            Token::New => write!(f, "New"),
            Token::This => write!(f, "This"),
            Token::Instanceof => write!(f, "Instanceof"),
            Token::Typeof => write!(f, "Typeof"),
            Token::Class => write!(f, "Class"),
            Token::Void => write!(f, "Void"),
            Token::Yield => write!(f, "Yield"),
            Token::Debugger => write!(f, "Debugger"),
            _ => {
                write!(f, "token")
            }
        }
    }
}

pub struct Lex {
    input: String,
    pos: usize,
}

impl Lex {
    pub fn new(input: String) -> Self {
        Lex { input, pos: 0 }
    }
    pub fn next(&mut self) -> Token {
        let str = &self.input;
        if self.pos == str.len() {
            return Token::EOF;
        }
        if self.pos > str.len() {
            panic!("end of source");
        }
        loop {
            let c = str.chars().nth(self.pos);
            match c {
                Some(c) => match c {
                    ' ' | '\r' | '\n' => self.pos += 1,
                    '"' | '\'' => {
                        return read_string(&mut self.pos, &str);
                    }
                    '=' | '+' | '-' | '*' | '/' | '%' | '>' | '<' | '|' | '?' | ':' => {
                        return read_operation(&mut self.pos, &str);
                    }
                    ';' | '(' | ')' | '{' | '}' | '.' | '!' | ',' | '[' | ']' => {
                        self.pos += 1;
                        return Token::Control(c.to_string());
                    }
                    '_' | 'a'..='z' | 'A'..='Z' => return read_word(&mut self.pos, &str),
                    '0'..='9' => return read_digit(&mut self.pos, &str),
                    _ => panic!("Unrecognized character {c}"),
                },
                None => return Token::EOF,
            }
        }
    }
}

fn read_word(i: &mut usize, source: &str) -> Token {
    let c = source.chars().nth(*i).unwrap();
    let mut word = String::new();
    word.push(c);
    loop {
        *i = *i + 1;
        let d = source.chars().nth(*i);
        match d {
            Some(d) => match d {
                '_' | 'a'..='z' | 'A'..='Z' | '0'..='9' => word.push(d),
                _ => break,
            },
            None => break,
        }
    }
    match word.as_str() {
        "var" => Token::Var,
        "let" => Token::Let,
        "const" => Token::Const,
        "undefined" => Token::Undefined,
        "null" => Token::Null,
        "await" => Token::Await,
        "async" => Token::Async,
        "function" => Token::Function,
        "with" => Token::With,
        "if" => Token::If,
        "switch" => Token::Switch,
        "case" => Token::Case,
        "break" => Token::Break,
        "continue" => Token::Continue,
        "for" => Token::For,
        "while" => Token::While,
        "return" => Token::Return,
        "true" => Token::True,
        "false" => Token::False,
        "else" => Token::Else,
        "in" => Token::In,
        "do" => Token::Do,
        "try" => Token::Try,
        "catch" => Token::Catch,
        "finally" => Token::Finally,
        "throw" => Token::Throw,
        "new" => Token::New,
        "this" => Token::This,
        "instanceof" => Token::Instanceof,
        "typeof" => Token::Typeof,
        "class" => Token::Class,
        "void" => Token::Void,
        "yield" => Token::Yield,
        "debugger" => Token::Debugger,
        _ => Token::Variable(word),
    }
}

fn read_string(i: &mut usize, source: &str) -> Token {
    let s = source.chars().nth(*i).unwrap();
    let mut word = String::new();
    let mut escaped = false;
    loop {
        *i = *i + 1;
        let c = source.chars().nth(*i);
        match c {
            Some(c) => match c {
                '"' | '\'' => {
                    if c == s {
                        if escaped {
                            escaped = false;
                        } else {
                            break;
                        }
                    }
                    word.push(c);
                }
                'r' | 'n' | 't' => {
                    if escaped {
                        escaped = false;
                        if c == 'r' {
                            word.push('\r');
                        }
                        if c == 'n' {
                            word.push('\n');
                        }
                        if c == 't' {
                            word.push('\t');
                        }
                    } else {
                        word.push(c);
                    }
                }
                '\n' => {
                    if escaped {
                        escaped = false;
                        word.push(c);
                    } else {
                        panic!("string format error, unsupported \\n")
                    }
                }
                '\\' => {
                    if escaped {
                        escaped = false;
                        word.push(c);
                    } else {
                        escaped = true;
                    }
                }
                _ => {
                    word.push(c);
                    escaped = false;
                }
            },
            None => break,
        }
    }
    Token::String(word)
}

fn read_operation(i: &mut usize, source: &str) -> Token {
    let c = source.chars().nth(*i).unwrap();
    let mut word = String::new();
    word.push(c);
    loop {
        *i = *i + 1;
        let c = source.chars().nth(*i);
        match c {
            Some(c) => match c {
                '=' | '+' | '-' | '*' | '/' | '%' | '>' | '<' | '|' | '?' | ':' => {
                    word.push(c);
                }
                _ => {
                    break;
                }
            },
            None => break,
        }
    }
    Token::Control(word)
}

fn read_newline(i: &mut usize, source: &str) -> Token {
    loop {
        *i = *i + 1;
        let c = source.chars().nth(*i);
        match c {
            Some(c) => match c {
                '\r' | '\n' | ' ' | '\t' => {}
                _ => break,
            },
            _ => break,
        }
    }
    Token::Control("\n".to_string())
}

fn read_digit(i: &mut usize, source: &str) -> Token {
    let c = source.chars().nth(*i).unwrap();
    let mut word = String::new();
    let mut exponential = false;
    word.push(c);
    loop {
        *i = *i + 1;
        let c = source.chars().nth(*i);
        match c {
            Some(c) => match c {
                '_' => {
                    word.push(c);
                }
                '0'..='9' => {
                    word.push(c);
                    exponential = false;
                }
                'e' => {
                    exponential = true;
                    word.push(c);
                }
                _ => {
                    break;
                }
            },
            None => break,
        }
    }
    if exponential {
        panic!("digit syntax error");
    }
    Token::Digit(word)
}

#[cfg(test)]
mod tests {
    use crate::lex::{Lex, Token};

    #[test]
    fn test_token_display() {
        let a = Token::For;
        println!("{a}")
    }

    #[test]
    fn test_keyword() {
        let input = "for(let i = 1; i < 10;i++)++";
        let mut lex = Lex::new(input.to_string());
        assert_eq!(lex.next(), Token::For);
    }

    #[test]
    fn test_digit_exponential() {
        let input = "1e3";
        let mut lex = Lex::new(input.to_string());
        assert_eq!(lex.next(), Token::Digit("1e3".to_string()));
    }

    #[test]
    fn test_string_single() {
        let input = "'abcdefjie'";
        let mut lex = Lex::new(input.to_string());
        assert_eq!(lex.next(), Token::String("abcdefjie".to_string()));
    }

    #[test]
    fn test_string_single_newline() {
        let input = "'abcdefjie\\nxx'";
        let mut lex = Lex::new(input.to_string());
        assert_eq!(lex.next(), Token::String("abcdefjie\nxx".to_string()));
    }

    #[test]
    fn test_string_double() {
        let input = "\"abcde\\\"fjie\"";
        let mut lex = Lex::new(input.to_string());
        assert_eq!(lex.next(), Token::String("abcde\"fjie".to_string()));
    }
    #[test]
    fn test_lex() {
        let input = " \n\n\nlet\n\n\n a\n\n\n =\n\n\n 1\n\n\n + \n\n\n2\n\n\n";
        let mut lex = Lex::new(input.to_string());

        assert_eq!(lex.next(), Token::Let);
        assert_eq!(lex.next(), Token::Variable("a".to_string()));
        assert_eq!(lex.next(), Token::Control("=".to_string()));
        assert_eq!(lex.next(), Token::Digit("1".to_string()));
        assert_eq!(lex.next(), Token::Control("+".to_string()));
        assert_eq!(lex.next(), Token::Digit("2".to_string()));
        assert_eq!(lex.next(), Token::EOF);
    }
}
