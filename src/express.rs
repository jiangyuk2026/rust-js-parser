use crate::exp::array_exp::build_array;
use crate::exp::arrow_function_exp::build_possible_arrow_function;
use crate::exp::function_exp::build_function;
use crate::exp::object_exp::build_object;
use crate::node::{
    ArrowFunctionExpression, AssignmentExpression, BinaryExpression, BooleanLiteral,
    CallExpression, ConditionalExpression, Identity, LogicalExpression, MemberExpression,
    NewExpression, NullLiteral, NumericLiteral, RegExpLiteral, SequenceExpression, StringLiteral,
    TemplateElement, TemplateLiteral, ThisExpression, UnaryExpression, UpdateExpression,
};
use crate::node::{Extra, Node};
use crate::parser::Parser;
use crate::token::{Token, is_keyword};
use std::rc::Rc;

pub fn parse_expression(parser: &mut Parser, min_level: u8) -> Result<Box<dyn Node>, String> {
    let mut left: Box<dyn Node>;

    let start_loc = parser.loc.clone();
    if parser.is_identity_keyword && is_keyword(&parser.current) {
        left = Box::new(Identity::new(
            parser.current.to_string(),
            start_loc.clone(),
            parser.last_loc.clone(),
        ));
        parser.next()?;
    } else if *parser.current == Token::Function {
        left = build_function(parser, false)?;
    } else if let Token::Control(s) = &*parser.current {
        let operator = s.to_string();
        let l = get_level(&parser.current)?;
        match s.as_str() {
            "++" | "--" => {
                parser.next()?;
                left = Box::new(UpdateExpression::new(
                    operator,
                    true,
                    parse_expression(parser, l + 1)?,
                    start_loc.clone(),
                    parser.last_loc.clone(),
                ));
            }
            "+" | "-" | "!" | "typeof" | "~" => {
                parser.regex_allowed = true;
                parser.next()?;
                left = Box::new(UnaryExpression::new(
                    operator,
                    true,
                    parse_expression(parser, l + 1)?,
                    start_loc.clone(),
                    parser.last_loc.clone(),
                ));
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
    } else if *parser.current == Token::Typeof {
        parser.regex_allowed = true;
        parser.next()?;
        left = Box::new(UnaryExpression::new(
            "typeof".to_string(),
            true,
            parse_expression(parser, 14)?,
            start_loc.clone(),
            parser.last_loc.clone(),
        ))
    } else if *parser.current == Token::Delete {
        parser.regex_allowed = true;
        parser.next()?;
        left = Box::new(UnaryExpression::new(
            "delete".to_string(),
            true,
            parse_expression(parser, 14)?,
            start_loc.clone(),
            parser.last_loc.clone(),
        ))
    } else if *parser.current == Token::True {
        parser.next()?;
        left = Box::new(BooleanLiteral::new(
            true,
            start_loc.clone(),
            parser.last_loc.clone(),
        ));
    } else if *parser.current == Token::False {
        parser.next()?;
        left = Box::new(BooleanLiteral::new(
            false,
            start_loc.clone(),
            parser.last_loc.clone(),
        ));
    } else if *parser.current == Token::This {
        parser.next()?;
        left = Box::new(ThisExpression::new(
            start_loc.clone(),
            parser.last_loc.clone(),
        ));
    } else if *parser.current == Token::Null {
        parser.next()?;
        left = Box::new(NullLiteral::new(start_loc.clone(), parser.last_loc.clone()));
    } else if *parser.current == Token::Undefined {
        parser.next()?;
        left = Box::new(Identity::new(
            "undefined".to_string(),
            start_loc.clone(),
            parser.last_loc.clone(),
        ));
    } else if let Token::Regex(pattern, flags) = &*parser.current {
        left = Box::new(RegExpLiteral::new(
            pattern.to_string(),
            flags.to_string(),
            start_loc.clone(),
            parser.last_loc.clone(),
        ));
        parser.next()?;
    } else if let Token::TemplateStr(s) = &*parser.current {
        left = Box::new(TemplateLiteral::new(
            vec![],
            vec![Box::new(TemplateElement::new(
                s.to_string(),
                start_loc.clone(),
                parser.last_loc.clone(),
            ))],
            start_loc.clone(),
            parser.last_loc.clone(),
        ));
        parser.next()?;
    } else if *parser.current == Token::New {
        parser.next()?;
        let callee = parse_expression(parser, 18)?;
        let mut arguments: Vec<Box<dyn Node>> = vec![];
        if is_ctrl_word(&parser.current, "(") {
            parser.next()?;
            loop {
                if is_ctrl_word(&parser.current, ")") {
                    break;
                }
                if is_ctrl_word(&parser.current, ",") {
                    parser.next()?;
                }
                arguments.push(parse_expression(parser, 2)?)
            }
            expect(parser, ")")?;
        }
        left = Box::new(NewExpression::new(
            callee,
            arguments,
            start_loc.clone(),
            parser.last_loc.clone(),
        ));
    } else if *parser.current == Token::Void {
        parser.regex_allowed = true;
        parser.next()?;
        left = Box::new(UnaryExpression::new(
            "void".to_string(),
            true,
            parse_expression(parser, 17)?,
            start_loc.clone(),
            parser.last_loc.clone(),
        ))
    } else if let Token::Variable(s) = &*parser.current {
        left = Box::new(Identity::new(
            s.to_string(),
            start_loc.clone(),
            parser.last_loc.clone(),
        ));
        parser.next()?;
    } else if let Token::Digit(d) = &*parser.current {
        left = Box::new(NumericLiteral::new(
            d.to_string(),
            start_loc.clone(),
            parser.last_loc.clone(),
        ));
        parser.next()?;
    } else if let Token::String(d, is_single_quoted) = &*parser.current {
        left = Box::new(StringLiteral::new(
            d.to_string(),
            *is_single_quoted,
            start_loc.clone(),
            parser.last_loc.clone(),
        ));
        parser.next()?;
    } else {
        return Err(format!(
            "unsupported parse_express start {}",
            &parser.current
        ));
    }
    parser.is_identity_keyword = false;
    loop {
        let operator = &*Rc::clone(&parser.current);
        match operator {
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

        match &*operator {
            Token::Control(s) => match s.as_str() {
                "," => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    let right = parse_expression(parser, l + 1)?;
                    if let Some(t) = left.as_any().downcast_ref::<SequenceExpression>() {
                        let mut exp: Vec<Box<dyn Node>> = vec![];
                        exp.extend(t.expressions.iter().cloned());
                        exp.push(right);
                        left = Box::new(SequenceExpression::new(
                            exp,
                            start_loc.clone(),
                            parser.last_loc.clone(),
                        ))
                    } else {
                        left = Box::new(SequenceExpression::new(
                            vec![left, right],
                            start_loc.clone(),
                            parser.last_loc.clone(),
                        ))
                    }
                }
                "=" | "+=" | "-=" | "*=" | "/=" | "%=" | ">>=" | "<<=" | "|=" | "&=" => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    let right = parse_expression(parser, l)?;
                    left = Box::new(AssignmentExpression::new(
                        left,
                        s.to_string(),
                        right,
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    ))
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
                    left = Box::new(ArrowFunctionExpression::new(
                        vec![left],
                        right,
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    ))
                }
                "." => {
                    parser.next()?;
                    parser.is_identity_keyword = true;
                    let right = parse_expression(parser, l + 1)?;
                    left = Box::new(MemberExpression::new(
                        right,
                        left,
                        false,
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    ))
                }
                "+" | "-" | "*" | "/" | "%" | ">" | "<" | ">=" | "<=" | "==" | "===" | "!="
                | "&" | "|" | "<<" | ">>" | "!==" | ">>>" => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    let right = parse_expression(parser, l + 1)?;
                    left = Box::new(BinaryExpression::new(
                        left,
                        s.to_string(),
                        right,
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    ))
                }
                "&&" | "||" => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    let right = parse_expression(parser, l + 1)?;
                    left = Box::new(LogicalExpression::new(
                        left,
                        s.to_string(),
                        right,
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    ))
                }
                "++" | "--" => {
                    parser.next()?;
                    left = Box::new(UpdateExpression::new(
                        s.to_string(),
                        false,
                        left,
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    ));
                }
                "?" => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    let consequent = parse_expression(parser, l)?;
                    expect(parser, ":")?;
                    let alternate = parse_expression(parser, l)?;
                    left = Box::new(ConditionalExpression::new(
                        left,
                        consequent,
                        alternate,
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    ));
                }
                "(" => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    let mut arguments: Vec<Box<dyn Node>> = vec![];
                    loop {
                        let next = &parser.current;
                        if is_ctrl_word(&next, ")") {
                            parser.next()?;
                            break;
                        }
                        let express = parse_expression(parser, 2)?;
                        arguments.push(express);
                        let current = &parser.current.clone();
                        if is_ctrl_word(&current, ",") {
                            parser.next()?;
                        }
                        if is_ctrl_word(&current, ")") {
                            parser.next()?;
                            break;
                        }
                    }
                    left = Box::new(CallExpression::new(
                        left,
                        arguments,
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    ));
                }
                "[" => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    let right = parse_expression(parser, 0)?;
                    expect(parser, "]")?;
                    left = Box::new(MemberExpression::new(
                        right,
                        left,
                        true,
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    ));
                }
                _ => {
                    return Err(format!("unsupported operator {:?}", &operator));
                }
            },
            Token::Instanceof | Token::In => {
                let operator = if *operator == Token::Instanceof {
                    "instanceof"
                } else {
                    "in"
                };
                parser.regex_allowed = true;
                parser.next()?;
                let right = parse_expression(parser, l + 1)?;
                left = Box::new(BinaryExpression::new(
                    left,
                    operator.to_string(),
                    right,
                    start_loc.clone(),
                    parser.last_loc.clone(),
                ))
            }
            _ => {
                break;
            }
        }
    }
    Ok(left)
}

pub fn ok_box(node: Box<dyn Node>) -> Result<Box<dyn Node>, String> {
    Ok(node)
}

pub fn box_(node: Box<dyn Node>) -> Box<dyn Node> {
    node
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
            "<<" | ">>" | ">>>" => 10,
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
    match &*parser.current {
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
