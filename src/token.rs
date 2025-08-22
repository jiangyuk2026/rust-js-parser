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
            Token::Comment(_) => write!(f, "Comment"),
            Token::String(_) => write!(f, "String"),
            Token::EOF => write!(f, "EOF"),

            Token::Var => write!(f, "var"),
            Token::Let => write!(f, "let"),
            Token::Const => write!(f, "const"),
            Token::Undefined => write!(f, "undefined"),
            Token::Null => write!(f, "null"),
            Token::Await => write!(f, "await"),
            Token::Async => write!(f, "async"),
            Token::Function => write!(f, "function"),
            Token::With => write!(f, "with"),
            Token::Delete => write!(f, "delete"),
            Token::If => write!(f, "if"),
            Token::Switch => write!(f, "switch"),
            Token::Case => write!(f, "case"),
            Token::Default => write!(f, "default"),
            Token::Break => write!(f, "break"),
            Token::Continue => write!(f, "continue"),
            Token::For => write!(f, "for"),
            Token::While => write!(f, "while"),
            Token::Return => write!(f, "return"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Else => write!(f, "else"),
            Token::In => write!(f, "in"),
            Token::Do => write!(f, "do"),
            Token::Try => write!(f, "try"),
            Token::Catch => write!(f, "catch"),
            Token::Finally => write!(f, "finally"),
            Token::Throw => write!(f, "throw"),
            Token::New => write!(f, "new"),
            Token::This => write!(f, "this"),
            Token::Instanceof => write!(f, "instanceof"),
            Token::Typeof => write!(f, "typeof"),
            Token::Class => write!(f, "class"),
            Token::Void => write!(f, "void"),
            Token::Yield => write!(f, "yield"),
            Token::Debugger => write!(f, "debugger"),
            _ => {
                write!(f, "token")
            }
        }
    }
}

pub fn is_keyword(token: &Token) -> bool {
    match token {
        Token::Var
        | Token::Let
        | Token::Const
        | Token::True
        | Token::False
        | Token::Undefined
        | Token::Null
        | Token::Await
        | Token::Async
        | Token::Function
        | Token::With
        | Token::Delete
        | Token::If
        | Token::Else
        | Token::Switch
        | Token::Case
        | Token::Default
        | Token::Break
        | Token::Continue
        | Token::For
        | Token::In
        | Token::Do
        | Token::While
        | Token::Return
        | Token::Try
        | Token::Catch
        | Token::Finally
        | Token::Throw
        | Token::New
        | Token::This
        | Token::Instanceof
        | Token::Typeof
        | Token::Class
        | Token::Void
        | Token::Yield
        | Token::Debugger => true,
        _ => false,
    }
}
