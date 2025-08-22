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
