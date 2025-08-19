use crate::exp::array_exp::build_array;
use crate::exp::arrow_function_exp::build_possible_arrow_function;
use crate::exp::function_exp::build_function;
use crate::exp::object_exp::build_object;
use crate::lex::Token;
use crate::node::Node::{
    BooleanLiteral, Identity, NewExpression, NullLiteral, SequenceExpression, TemplateElement,
    TemplateLiteral, ThisExpression, UnaryExpression,
};
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
            "(" => {
                return build_possible_arrow_function(parser);
            }
            "[" => {
                return build_array(parser);
            }
            "{" => return build_object(parser),
            _ => return Err("expect control,".to_string()),
        };
    }

    if parser.current == Token::Function {
        return Ok(build_function(parser, false)?);
    }

    let mut left: Box<Node>;

    if parser.current == Token::Typeof {
        parser.next();
        left = Box::new(UnaryExpression {
            argument: parse_expression(parser, 14)?,
            operator: "typeof".to_string(),
            prefix: true,
        })
    } else if parser.current == Token::True {
        parser.next();
        left = Box::new(BooleanLiteral { value: true });
    } else if parser.current == Token::False {
        parser.next();
        left = Box::new(BooleanLiteral { value: false });
    } else if parser.current == Token::This {
        parser.next();
        left = Box::new(ThisExpression {});
    } else if parser.current == Token::Null {
        parser.next();
        left = Box::new(NullLiteral {});
    } else if parser.current == Token::Undefined {
        parser.next();
        left = Box::new(Identity {
            name: "undefined".to_string(),
        });
    } else if let Token::TemplateStr(s) = &parser.current {
        left = Box::new(TemplateLiteral {
            expressions: vec![],
            quasis: vec![TemplateElement {
                value: s.to_string(),
            }],
        });
        parser.next();
    } else if parser.current == Token::New {
        parser.next();
        let callee = parse_expression(parser, 18)?;
        let mut arguments = vec![];
        if is_ctrl_word(&parser.current, "(") {
            parser.next();
            loop {
                if is_ctrl_word(&parser.current, ")") {
                    break;
                }
                if is_ctrl_word(&parser.current, ",") {
                    parser.next();
                }
                arguments.push(*parse_expression(parser, 2)?)
            }
            expect(&parser.current, ")")?;
            parser.next();
        }
        left = Box::new(NewExpression { callee, arguments });
    } else if let Token::Variable(s) = &parser.current {
        left = Box::new(Identity {
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
            Token::Variable(_) => {
                if parser.last_loc.end.line != parser.loc.start.line {
                    break;
                }
                return Err("syntax error:".to_string());
            }
            Token::Digit(_) => {
                if parser.last_loc.end.line != parser.loc.start.line {
                    break;
                }
                return Err("syntax error:".to_string());
            }
            Token::Instanceof => {}
            Token::In => {}
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
                "=" | "+=" | "-=" | "*=" | "/=" | "%=" | ">>=" | "<<=" | "|=" | "&=" => {
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
                    parser.next();
                    let right = parse_expression(parser, l + 1)?;
                    left = Box::new(Node::MemberExpression {
                        computed: false,
                        object: left,
                        property: right,
                    })
                }
                "+" | "-" | "*" | "/" | "%" | ">" | "<" | ">=" | "<=" | "==" | "===" | "!="
                | "!==" => {
                    parser.next();
                    let right = parse_expression(parser, l + 1)?;
                    left = Box::new(Node::BinaryExpression {
                        operator: s.to_string(),
                        left,
                        right,
                        extra: Extra::Parenthesized,
                    })
                }
                "&&" | "||" => {
                    parser.next();
                    let right = parse_expression(parser, l + 1)?;
                    left = Box::new(Node::LogicalExpression {
                        operator: s.to_string(),
                        left,
                        right,
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
                    left = Box::new(Node::CallExpression {
                        callee: left,
                        arguments,
                    });
                }
                "[" => {
                    parser.next();
                    let right = parse_expression(parser, 0)?;
                    expect(&parser.current, "]")?;
                    parser.next();
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
            Token::Instanceof => {
                parser.next();
                let right = parse_expression(parser, l + 1)?;
                left = Box::new(Node::BinaryExpression {
                    operator: "instanceof".to_string(),
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
                return Err(format!("expect() expect: {s}"));
            }
        }
        _ => return Err(format!("expect() expect:  {s}")),
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
