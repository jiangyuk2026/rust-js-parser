use crate::exp::function_exp::build_function;
use crate::exp::object_exp::build_object;
use crate::lex::Token;
use crate::node::Node::SequenceExpression;
use crate::node::{Extra, Node};
use crate::parser::Parser;

pub fn parse_expression(parser: &mut Parser, min_level: u8) -> Result<Box<Node>, String> {
    let word = parser.current.clone();
    if let Token::Control(s) = word {
        let l = get_level(&parser.current)?;
        match s.as_str() {
            "++" => {
                parser.next();
                return Ok(Box::new(Node::UpdateExpression {
                    operator: s.to_string(),
                    prefix: true,
                    argument: parse_expression(parser, l + 1)?,
                }));
            }
            "+" | "-" | "!" | "typeof" => {
                parser.next();
                return Ok(Box::new(Node::UnaryExpression {
                    operator: s.to_string(),
                    prefix: true,
                    argument: parse_expression(parser, l + 1)?,
                }));
            }
            "(" => {}
            "{" => return build_object(parser),
            _ => return Err("expect control,".to_string()),
        };
    }
    if parser.current == Token::Function {
        return Ok(build_function(parser, false)?);
    }
    let mut left: Box<Node>;

    if is_ctrl_word(&parser.current, "(") {
        parser.next();
        if is_ctrl_word(&parser.current, ")") {
            parser.next();
            if is_ctrl_word(&parser.current, "=>") {
                left = Box::new(SequenceExpression {
                    expressions: vec![],
                    extra: Extra::None,
                });
            } else {
                return Err("expect expression, but found ()".to_string());
            }
        } else if is_ctrl_word(&parser.current, "(") {
            let express = parse_expression(parser, 1)?;
            if !is_ctrl_word(&parser.current, ")") {
                return Err("expect )".to_string());
            }
            parser.next();
            return Ok(express);
        } else {
            let express = parse_expression(parser, 1)?;
            if !is_ctrl_word(&parser.current, ")") {
                return Err("expect )".to_string());
            }
            parser.next();
            left = express;
        }
    } else if let Token::Variable(s) = &parser.current {
        left = Box::new(Node::Identity {
            name: s.to_string(),
        });
        parser.next();
    } else if let Token::Digit(d) = &parser.current {
        left = Box::new(Node::NumericLiteral {
            value: d.to_string(),
        });
        parser.next();
    } else if let Token::String(d) = &parser.current {
        left = Box::new(Node::StringLiteral {
            value: d.to_string(),
        });
        parser.next();
    } else {
        return Err(format!(
            "unsupported parse_express start {}",
            &parser.current
        ));
    }

    loop {
        let operator = parser.current.clone();
        match &operator {
            Token::Control(s) => match s.as_str() {
                ";" | ":" | ")" | "]" | "}" => break,
                _ => {}
            },
            Token::EOF => break,
            Token::Variable(_) => return Err("syntax error:".to_string()),
            Token::Digit(_) => return Err("syntax error:".to_string()),
            _ => break,
        }
        let l = get_level(&parser.current)?;
        if l < min_level {
            break;
        }

        match &operator {
            Token::Control(s) => match s.as_str() {
                "," => {
                    parser.next();
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
                "=" => {
                    parser.next();
                    let right = parse_expression(parser, l)?;
                    left = Box::new(Node::AssignmentExpression {
                        operator: s.to_string(),
                        left,
                        right,
                    })
                }
                "=>" => {
                    parser.next();
                    let params;
                    let right;
                    if is_ctrl_word(&parser.current, "{") {
                        right = Parser::parse_block(parser)?
                    } else {
                        right = parse_expression(parser, 2)?;
                    }
                    if let SequenceExpression { expressions, .. } = *left {
                        params = expressions;
                    } else {
                        params = vec![*left];
                    }
                    left = Box::new(Node::ArrowFunctionExpression {
                        params,
                        body: right,
                    })
                }
                "." => {
                    parser.next();
                    let right = parse_expression(parser, l + 1)?;
                    left = Box::new(Node::MemberExpression {
                        computed: false,
                        object: left,
                        property: right,
                    })
                }
                "+" | "-" | "*" | "/" | "%" | ">" | "<" | ">=" | "<=" => {
                    parser.next();
                    let right = parse_expression(parser, l + 1)?;
                    left = Box::new(Node::BinaryExpression {
                        operator: s.to_string(),
                        left,
                        right,
                        extra: Extra::Parenthesized,
                    })
                }
                "++" | "--" => {
                    parser.next();
                    return ok_box(Node::UpdateExpression {
                        operator: s.to_string(),
                        prefix: false,
                        argument: left,
                    });
                }
                "?" => {
                    parser.next();
                    let consequent = parse_expression(parser, l)?;
                    if !is_ctrl_word(&parser.current, ":") {
                        return Err("expect :".to_string());
                    }
                    parser.next();
                    let alternate = parse_expression(parser, l + 1)?;
                    return ok_box(Node::ConditionalExpression {
                        test: left,
                        consequent,
                        alternate,
                    });
                }
                "(" => {
                    parser.next();
                    let mut arguments: Vec<Node> = vec![];
                    loop {
                        let next = &parser.current;
                        if is_ctrl_word(&next, ")") {
                            parser.next();
                            break;
                        }
                        let express = parse_expression(parser, 1)?;
                        arguments.push(*express);
                        let current = &parser.current.clone();
                        if is_ctrl_word(&current, ",") {
                            parser.next();
                        }
                        if is_ctrl_word(&current, ")") {
                            parser.next();
                            break;
                        }
                    }
                    return ok_box(Node::CallExpression {
                        callee: left,
                        arguments,
                    });
                }
                "[" => {
                    parser.next();
                    let right = parse_expression(parser, l)?;
                    expect(&parser.current, "]")?;
                    parser.next();
                    return ok_box(Node::MemberExpression {
                        computed: true,
                        object: left,
                        property: right,
                    });
                }
                _ => {
                    return Err(format!("unsupported operator {:?}", &operator));
                }
            },
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
            "new" | "." | "[" | "(" | "?." | "{" => 17,
            "++" | "--" => 15,
            "!" | "~" | "typeof" | "await" | "delete" => 14,
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
            "?" | "=" | "+=" | "-=" | "*=" | "/=" | "%=" | "=>" => 2,
            "," => 1,
            _ => return Err(format!("get level err {token}")),
        },
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

pub fn is_ctrl(word: &Token) -> bool {
    match word {
        Token::Control(_) => true,
        _ => false,
    }
}

pub fn skip_empty(parser: &mut Parser) -> Token {
    loop {
        match &parser.current {
            Token::Control(next) => match next.as_str() {
                "\r" | "\n" | " " | "\t" => {}
                _ => break,
            },
            _ => break,
        }
        parser.next();
    }
    parser.current.clone()
}

pub fn expect(word: &Token, s: &str) -> Result<(), String> {
    match word {
        Token::Control(next) => {
            if next != s {
                return Err(format!("expect {s}"));
            }
        }
        _ => return Err(format!("expect {s}")),
    }
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
