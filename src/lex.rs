use crate::lex::Token::Comment;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub struct Loc {
    pub start: Position,
    pub end: Position,
}

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
    Delete,
    If,
    Else,
    Switch,
    Case,
    Default,
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
    Comment(String),
    TemplateStr(String),
    Regex(String, String),
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
            Token::Delete => write!(f, "Delete"),
            Token::If => write!(f, "If"),
            Token::Switch => write!(f, "Switch"),
            Token::Case => write!(f, "Case"),
            Token::Default => write!(f, "Default"),
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
    line: usize,
    column: usize,
}

impl Lex {
    pub fn new(input: String) -> Self {
        Lex {
            input,
            pos: 0,
            line: 1,
            column: 1,
        }
    }
    pub fn next(&mut self) -> (Token, Loc) {
        let str = &self.input;
        if self.pos > str.len() {
            panic!("end of source");
        }
        loop {
            let c = str.chars().nth(self.pos);
            match c {
                Some(c) => match c {
                    ' ' => {
                        self.column += 1;
                    }
                    '\t' => {
                        println!("\\t find");
                        self.column += 1;
                    }
                    '\r' => {}
                    '\n' => {
                        self.line += 1;
                        self.column = 1;
                    }
                    _ => {
                        break;
                    }
                },
                None => break,
            }
            self.pos += 1;
        }
        let start = Position {
            line: self.line,
            column: self.column,
        };
        let mut result;
        loop {
            let c = str.chars().nth(self.pos);
            match c {
                Some(c) => match c {
                    '"' | '\'' => {
                        result = self.read_string();
                        break;
                    }
                    '=' | '+' | '-' | '*' | '/' | '%' | '>' | '<' | '|' | '?' | ':' | '!' | '&' => {
                        result = self.read_operation();
                        break;
                    }
                    ';' | '(' | ')' | '{' | '}' | '.' | ',' | '[' | ']' => {
                        self.pos += 1;
                        self.column += 1;
                        result = Token::Control(c.to_string());
                        break;
                    }
                    '_' | 'a'..='z' | 'A'..='Z' => {
                        result = self.read_word();
                        break;
                    }
                    '0'..='9' => {
                        result = self.read_digit();
                        break;
                    }
                    '`' => {
                        result = self.read_template_str();
                        break;
                    }
                    _ => panic!("Unrecognized character {c}"),
                },
                None => {
                    result = Token::EOF;
                    break;
                }
            }
        }
        let end = Position {
            line: self.line,
            column: self.column,
        };
        (result, Loc { start, end })
    }

    fn read_word(&mut self) -> Token {
        let c = self.input.chars().nth(self.pos).unwrap();
        let mut word = String::new();
        word.push(c);
        loop {
            self.pos += 1;
            self.column += 1;
            let d = self.input.chars().nth(self.pos);
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
            "delete" => Token::Delete,
            "if" => Token::If,
            "switch" => Token::Switch,
            "case" => Token::Case,
            "default" => Token::Default,
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

    fn read_string(&mut self) -> Token {
        let s = self.input.chars().nth(self.pos).unwrap();
        let mut word = String::new();
        let mut escaped = false;
        loop {
            self.pos += 1;
            self.column += 1;
            let c = self.input.chars().nth(self.pos);
            match c {
                Some(c) => match c {
                    '"' | '\'' => {
                        if c == s {
                            if escaped {
                                escaped = false;
                            } else {
                                self.pos += 1;
                                self.column += 1;
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
                            panic!("string format error, unsupported \\n, line: {}", self.line)
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

    fn read_operation(&mut self) -> Token {
        let c = self.input.chars().nth(self.pos).unwrap();
        let mut word = String::new();
        word.push(c);
        loop {
            self.pos += 1;
            self.column += 1;
            if word == "//" {
                return self.read_comment();
            }
            let d = self.input.chars().nth(self.pos);
            match d {
                Some(d) => match d {
                    '=' | '+' | '-' | '*' | '/' | '%' | '>' | '<' | '|' | '?' | ':' | '&' => {
                        if word == "/" && d != '/' {
                            return self.read_regex();
                        }
                        word.push(d);
                    }
                    _ => {
                        if word == "/" {
                            return self.read_regex();
                        }
                        break;
                    }
                },
                None => break,
            }
        }

        Token::Control(word)
    }

    fn read_regex(&mut self) -> Token {
        let mut escaped = false;
        let mut flags_start = false;
        let c = self.input.chars().nth(self.pos).unwrap();
        if c == '/' {
            panic!("expect regex, but find //")
        }
        let mut word = String::new();
        let mut flags = String::new();
        word.push(c);
        loop {
            self.pos += 1;
            self.column += 1;
            let d = self.input.chars().nth(self.pos);
            match d {
                Some(d) => match d {
                    '/' => {
                        if flags_start {
                            panic!("expect regex flags, but found /");
                        }
                        if escaped {
                            escaped = false;
                            word.push(d);
                        } else {
                            flags_start = true
                        }
                    }
                    '_' | 'a'..='z' | '0'..='9' => match d {
                        'i' | 'g' | 'm' | 's' | 'u' | 'y' => {
                            if flags_start {
                                if flags.contains(d) {
                                    panic!("repeated regex flags");
                                }
                                flags.push(d);
                            }
                        }
                        _ => {
                            if flags_start {
                                panic!("regex expect newline or semicolon");
                            } else {
                                word.push(d);
                            }
                        }
                    },
                    _ => {
                        if flags_start {
                            break;
                        }
                        word.push(d);
                    }
                },
                None => {
                    break;
                }
            }
        }
        if !flags_start {
            panic!("regex syntax error");
        }
        Token::Regex(word, flags)
    }

    fn read_comment(&mut self) -> Token {
        let mut word = String::new();
        loop {
            let c = self.input.chars().nth(self.pos);
            match c {
                Some(c) => match c {
                    '\r' | '\n' => {
                        break;
                    }
                    s => {
                        word.push(s);
                    }
                },
                None => {
                    break;
                }
            }
            self.pos += 1;
            self.column += 1;
        }
        Comment(word)
    }

    fn read_newline(&mut self) -> Token {
        loop {
            let c = self.input.chars().nth(self.pos);
            match c {
                Some(c) => match c {
                    '\r' | ' ' | '\t' => {
                        self.column += 1;
                    }
                    '\n' => {
                        self.line += 1;
                        self.column = 1;
                    }
                    _ => break,
                },
                _ => break,
            }
            self.pos += 1;
            self.column += 1;
        }
        Token::LF
    }

    fn read_digit(&mut self) -> Token {
        let c = self.input.chars().nth(self.pos).unwrap();
        let mut word = String::new();
        let mut exponential = false;
        word.push(c);
        loop {
            self.pos += 1;
            self.column += 1;
            let c = self.input.chars().nth(self.pos);
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
    fn read_template_str(&mut self) -> Token {
        let mut word = String::new();
        loop {
            self.pos += 1;
            self.column += 1;
            let c = self.input.chars().nth(self.pos);
            match c {
                Some(c) => match c {
                    '`' => {
                        self.pos += 1;
                        self.column += 1;
                        break;
                    }
                    _ => {
                        word.push(c);
                    }
                },
                None => panic!("template string error"),
            }
        }
        Token::TemplateStr(word)
    }
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
        assert_eq!(lex.next().0, Token::For);
    }

    #[test]
    fn test_digit_exponential() {
        let input = "1e3";
        let mut lex = Lex::new(input.to_string());
        assert_eq!(lex.next().0, Token::Digit("1e3".to_string()));
    }

    #[test]
    fn test_string_single() {
        let input = "'abcdefjie'";
        let mut lex = Lex::new(input.to_string());
        assert_eq!(lex.next().0, Token::String("abcdefjie".to_string()));
    }

    #[test]
    fn test_string_single_newline() {
        let input = "'abcdefjie\\nxx'";
        let mut lex = Lex::new(input.to_string());
        assert_eq!(lex.next().0, Token::String("abcdefjie\nxx".to_string()));
    }

    #[test]
    fn test_string_double() {
        let input = "\"abcde\\\"fjie\"";
        let mut lex = Lex::new(input.to_string());
        assert_eq!(lex.next().0, Token::String("abcde\"fjie".to_string()));
    }

    #[test]
    fn test_comment() {
        let input = "//abcd\n//dddd";
        let mut lex = Lex::new(input.to_string());
        assert_eq!(lex.next().0, Token::Comment("abcd".to_string()));
        assert_eq!(lex.next().0, Token::Comment("dddd".to_string()));
    }

    #[test]
    fn test_regex() {
        let input = "/abc/";
        let mut lex = Lex::new(input.to_string());
        assert_eq!(
            lex.next().0,
            Token::Regex("abc".to_string(), "".to_string())
        );
    }

    #[test]
    fn test_regex_flags() {
        let input = "/abc/ig";
        let mut lex = Lex::new(input.to_string());
        assert_eq!(
            lex.next().0,
            Token::Regex("abc".to_string(), "ig".to_string())
        );
    }

    #[test]
    fn test_regex_escape() {
        let input = "/abc\\r\\n/ig";
        let mut lex = Lex::new(input.to_string());
        assert_eq!(
            lex.next().0,
            Token::Regex("abc\\r\\n".to_string(), "ig".to_string())
        );
    }

    #[test]
    fn test_lex() {
        let input = " \n\n\nlet\n\n\n a\n\n\n =\n\n\n 1\n\n\n + \n\n\n2\n\n\n";
        let mut lex = Lex::new(input.to_string());

        assert_eq!(lex.next().0, Token::Let);
        assert_eq!(lex.next().0, Token::Variable("a".to_string()));
        assert_eq!(lex.next().0, Token::Control("=".to_string()));
        assert_eq!(lex.next().0, Token::Digit("1".to_string()));
        assert_eq!(lex.next().0, Token::Control("+".to_string()));
        assert_eq!(lex.next().0, Token::Digit("2".to_string()));
        assert_eq!(lex.next().0, Token::EOF);
    }
}
