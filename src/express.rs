use crate::exp::array_exp::build_array;
use crate::exp::arrow_function_exp::build_possible_arrow_function;
use crate::exp::function_exp::build_function;
use crate::exp::object_exp::build_object;
use crate::node::Node::{
    BooleanLiteral, Identity, NewExpression, NullLiteral, RegExpLiteral, SequenceExpression,
    TemplateElement, TemplateLiteral, ThisExpression, UnaryExpression,
};
use crate::node::{Extra, Node};
use crate::parser::Parser;
use crate::token::{Token, is_keyword};

pub fn parse_expression(parser: &mut Parser, min_level: u8) -> Result<Box<Node>, String> {
    let mut left: Box<Node>;
    if parser.is_identity_keyword && is_keyword(&parser.current) {
        left = Box::new(Identity {
            name: parser.current.to_string(),
        });
        parser.next()?;
    } else if parser.current == Token::Function {
        left = build_function(parser, false)?;
    } else if let Token::Control(s) = &parser.current {
        let operator = s.to_string();
        let l = get_level(&parser.current)?;
        match s.as_str() {
            "++" | "--" => {
                parser.next()?;
                left = Box::new(Node::UpdateExpression {
                    operator,
                    prefix: true,
                    argument: parse_expression(parser, l + 1)?,
                });
            }
            "+" | "-" | "!" | "typeof" | "~" => {
                parser.regex_allowed = true;
                parser.next()?;
                left = Box::new(UnaryExpression {
                    operator,
                    prefix: true,
                    argument: parse_expression(parser, l + 1)?,
                });
            }
            "(" => {
                left = build_possible_arrow_function(parser)?;
            }
            "[" => {
                left = build_array(parser)?;
            }
            "{" => left = build_object(parser)?,
            _ => return Err("expect control,".to_string()),
        }
    } else if parser.current == Token::Typeof {
        parser.regex_allowed = true;
        parser.next()?;
        left = Box::new(UnaryExpression {
            argument: parse_expression(parser, 14)?,
            operator: "typeof".to_string(),
            prefix: true,
        })
    } else if parser.current == Token::Delete {
        parser.regex_allowed = true;
        parser.next()?;
        left = Box::new(UnaryExpression {
            argument: parse_expression(parser, 14)?,
            operator: "delete".to_string(),
            prefix: true,
        })
    } else if parser.current == Token::True {
        parser.next()?;
        left = Box::new(BooleanLiteral { value: true });
    } else if parser.current == Token::False {
        parser.next()?;
        left = Box::new(BooleanLiteral { value: false });
    } else if parser.current == Token::This {
        parser.next()?;
        left = Box::new(ThisExpression {});
    } else if parser.current == Token::Null {
        parser.next()?;
        left = Box::new(NullLiteral {});
    } else if parser.current == Token::Undefined {
        parser.next()?;
        left = Box::new(Identity {
            name: "undefined".to_string(),
        });
    } else if let Token::Regex(pattern, flags) = &parser.current {
        left = Box::new(RegExpLiteral {
            pattern: pattern.to_string(),
            flags: flags.to_string(),
        });
        parser.next()?;
    } else if let Token::TemplateStr(s) = &parser.current {
        left = Box::new(TemplateLiteral {
            expressions: vec![],
            quasis: vec![TemplateElement {
                value: s.to_string(),
            }],
        });
        parser.next()?;
    } else if parser.current == Token::New {
        parser.next()?;
        let callee = parse_expression(parser, 18)?;
        let mut arguments = vec![];
        if is_ctrl_word(&parser.current, "(") {
            parser.next()?;
            loop {
                if is_ctrl_word(&parser.current, ")") {
                    break;
                }
                if is_ctrl_word(&parser.current, ",") {
                    parser.next()?;
                }
                arguments.push(*parse_expression(parser, 2)?)
            }
            expect(parser, ")")?;
        }
        left = Box::new(NewExpression { callee, arguments });
    } else if let Token::Variable(s) = &parser.current {
        left = Box::new(Identity {
            name: s.to_string(),
        });
        parser.next()?;
    } else if let Token::Digit(d) = &parser.current {
        left = Box::new(Node::NumericLiteral {
            value: d.to_string(),
        });
        parser.next()?;
    } else if let Token::String(d) = &parser.current {
        left = Box::new(Node::StringLiteral {
            value: d.to_string(),
        });
        parser.next()?;
    } else {
        return Err(format!(
            "unsupported parse_express start {}",
            &parser.current
        ));
    }
    parser.is_identity_keyword = false;
    loop {
        let operator = parser.current.clone();
        match &operator {
            Token::Control(s) => match s.as_str() {
                ";" | ":" | ")" | "]" | "}" => break,
                _ => {}
            },
            Token::EOF => break,
            Token::Variable(_) => {
                if parser.is_same_line() {
                    return Err("syntax error:".to_string());
                }
                break;
            }
            Token::Digit(_) => {
                if parser.is_same_line() {
                    return Err("syntax error:".to_string());
                }
                break;
            }
            Token::Instanceof => {}
            Token::In => {
                if parser.in_for_init {
                    break;
                }
            }
            Token::Void => {}
            _ => break,
        }
        let l = get_level(&parser.current)?;
        if l < min_level {
            break;
        }

        match &operator {
            Token::Control(s) => match s.as_str() {
                "," => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    let right = parse_expression(parser, l + 1)?;
                    if let SequenceExpression { expressions, .. } = *left {
                        let mut exp = vec![];
                        exp.extend(expressions);
                        exp.push(*right);
                        left = Box::new(SequenceExpression {
                            expressions: exp,
                            extra: Extra::Parenthesized,
                        })
                    } else {
                        left = Box::new(SequenceExpression {
                            expressions: vec![*left, *right],
                            extra: Extra::Parenthesized,
                        })
                    }
                }
                "=" | "+=" | "-=" | "*=" | "/=" | "%=" | ">>=" | "<<=" | "|=" | "&=" => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    let right = parse_expression(parser, l)?;
                    left = Box::new(Node::AssignmentExpression {
                        operator: s.to_string(),
                        left,
                        right,
                    })
                }
                "=>" => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    let right;
                    if is_ctrl_word(&parser.current, "{") {
                        right = Parser::parse_block(parser)?
                    } else {
                        right = parse_expression(parser, 2)?;
                    }
                    left = Box::new(Node::ArrowFunctionExpression {
                        params: vec![*left],
                        body: right,
                    })
                }
                "." => {
                    parser.next()?;
                    parser.is_identity_keyword = true;
                    let right = parse_expression(parser, l + 1)?;
                    left = Box::new(Node::MemberExpression {
                        computed: false,
                        object: left,
                        property: right,
                    })
                }
                "+" | "-" | "*" | "/" | "%" | ">" | "<" | ">=" | "<=" | "==" | "===" | "!="
                | "&" | "|" | "<<" | ">>" | "!==" => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    let right = parse_expression(parser, l + 1)?;
                    left = Box::new(Node::BinaryExpression {
                        operator: s.to_string(),
                        left,
                        right,
                        extra: Extra::Parenthesized,
                    })
                }
                "&&" | "||" => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    let right = parse_expression(parser, l + 1)?;
                    left = Box::new(Node::LogicalExpression {
                        operator: s.to_string(),
                        left,
                        right,
                    })
                }
                "++" | "--" => {
                    parser.next()?;
                    return ok_box(Node::UpdateExpression {
                        operator: s.to_string(),
                        prefix: false,
                        argument: left,
                    });
                }
                "?" => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    let consequent = parse_expression(parser, l)?;
                    expect(parser, ":")?;
                    let alternate = parse_expression(parser, l)?;
                    return ok_box(Node::ConditionalExpression {
                        test: left,
                        consequent,
                        alternate,
                    });
                }
                "(" => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    let mut arguments: Vec<Node> = vec![];
                    loop {
                        let next = &parser.current;
                        if is_ctrl_word(&next, ")") {
                            parser.next()?;
                            break;
                        }
                        let express = parse_expression(parser, 2)?;
                        arguments.push(*express);
                        let current = &parser.current.clone();
                        if is_ctrl_word(&current, ",") {
                            parser.next()?;
                        }
                        if is_ctrl_word(&current, ")") {
                            parser.next()?;
                            break;
                        }
                    }
                    left = Box::new(Node::CallExpression {
                        callee: left,
                        arguments,
                    });
                }
                "[" => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    let right = parse_expression(parser, 0)?;
                    expect(parser, "]")?;
                    left = Box::new(Node::MemberExpression {
                        computed: true,
                        object: left,
                        property: right,
                    });
                }
                _ => {
                    return Err(format!("unsupported operator {:?}", &operator));
                }
            },
            Token::Instanceof | Token::In => {
                let operator = if operator == Token::Instanceof {
                    "instanceof"
                } else {
                    "in"
                };
                parser.regex_allowed = true;
                parser.next()?;
                let right = parse_expression(parser, l + 1)?;
                left = Box::new(Node::BinaryExpression {
                    operator: operator.to_string(),
                    left,
                    right,
                    extra: Extra::Parenthesized,
                })
            }
            _ => {
                break;
            }
        }
    }
    Ok(left)
}

pub fn ok_box(node: Node) -> Result<Box<Node>, String> {
    Ok(Box::new(node))
}

pub fn box_(node: Node) -> Box<Node> {
    Box::new(node)
}

fn get_level(token: &Token) -> Result<u8, String> {
    let d = match token {
        Token::Control(s) => match s.as_str() {
            "." | "[" | "(" | "?." | "{" => 17,
            "++" | "--" => 15,
            "!" | "~" => 14,
            "**" => 13,
            "*" | "/" | "%" => 12,
            "+" | "-" => 11,
            "<<" | ">>" => 10,
            ">" | ">=" | "<" | "<=" => 9,
            "==" | "!=" | "!==" | "===" => 8,
            "&" => 7,
            "^" => 6,
            "|" => 5,
            "&&" => 4,
            "??" | "||" => 3,
            "?" | "=" | "+=" | "-=" | "*=" | "/=" | "%=" | ">>=" | "<<=" | "|=" | "&=" | "=>" => 2,
            "," => 1,
            _ => return Err(format!("get level err {token}")),
        },
        Token::Instanceof => 9,
        Token::In => 9,
        Token::Typeof => 14,
        Token::Void => 14,
        Token::Delete => 14,
        Token::Await => 14,
        Token::New => 17,
        _ => return Err(format!("get level err {token}")),
    };
    Ok(d)
}

pub fn is_ctrl_word(word: &Token, str: &str) -> bool {
    match word {
        Token::Control(s) => {
            if s == str {
                return true;
            }
            false
        }
        _ => false,
    }
}

pub fn expect(parser: &mut Parser, s: &str) -> Result<(), String> {
    match &parser.current {
        Token::Control(next) => {
            if next != s {
                return Err(format!("expect() expect: {s}"));
            }
        }
        _ => return Err(format!("expect() expect:  {s}")),
    }
    parser.next()?;
    Ok(())
}

pub fn expect_keyword(word: &Token, token: Token) -> Result<(), String> {
    if *word == token {
        return Ok(());
    }
    Err(format!("expect keyword {token}"))
}

pub fn expect_keys(word: &Token, list: &Vec<Token>) -> Result<Token, String> {
    for s in list {
        if s == word {
            return Ok(s.clone());
        }
    }
    Err(format!("expect {list:?}"))
}
