use crate::express::{expect, is_ctrl_word, ok_box, parse_expression};
use crate::node::Node::{
    ArrayExpression, ArrowFunctionExpression, AssignmentExpression, AssignmentPattern, Identity,
    NumericLiteral, ObjectExpression, ObjectProperty, SequenceExpression, StringLiteral,
};
use crate::node::{Extra, Node};
use crate::parser::{IsArrowFunction, Parser};
use std::cmp::PartialEq;
use crate::token::Token;

pub fn build_possible_arrow_function(parser: &mut Parser) -> Result<Box<Node>, String> {
    let mut params = vec![];
    let body: Box<Node>;

    expect(parser, "(")?;
    parser.is_arrow_function = IsArrowFunction::Maybe;
    loop {
        if is_ctrl_word(&parser.current, ")") {
            break;
        } else if is_ctrl_word(&parser.current, ",") {
            parser.next()?;
            continue;
        } else if is_ctrl_word(&parser.current, "{") {
            params.push(*build_possible_object(parser)?);
        } else if is_ctrl_word(&parser.current, "(") {
            parser.is_arrow_function = IsArrowFunction::Impossible;
            params.push(*parse_expression(parser, 0)?);
        } else if is_ctrl_word(&parser.current, "[") {
            params.push(*build_possible_array(parser)?);
        } else {
            let exp = *parse_expression(parser, 2)?;
            if let Identity { .. } = &exp {
            } else if let AssignmentExpression { operator, .. } = &exp {
                if operator != "=" {
                    parser.is_arrow_function = IsArrowFunction::Impossible;
                }
            } else {
                parser.is_arrow_function = IsArrowFunction::Impossible;
            }
            params.push(exp);
        }
    }

    expect(parser, ")")?;
    if !is_ctrl_word(&parser.current, "=>") {
        return if parser.is_arrow_function == IsArrowFunction::Must {
            Err("syntax error".to_string())
        } else {
            if params.len() == 0 {
                Err("syntax error, ()".to_string())
            } else if params.len() == 1 {
                ok_box(params.remove(0))
            } else {
                ok_box(SequenceExpression {
                    expressions: params,
                    extra: Extra::Parenthesized,
                })
            }
        };
    }
    if parser.is_arrow_function == IsArrowFunction::Impossible {
        return Err("syntax error".to_string());
    }
    parser.next()?;
    if is_ctrl_word(&parser.current, "{") {
        body = Parser::parse_block(parser)?
    } else {
        body = parse_expression(parser, 2)?
    }

    ok_box(ArrowFunctionExpression { params, body })
}

fn build_possible_object(parser: &mut Parser) -> Result<Box<Node>, String> {
    let mut properties = vec![];

    expect(parser, "{")?;
    loop {
        if is_ctrl_word(&parser.current, "}") {
            break;
        }
        if is_ctrl_word(&parser.current, ",") {
            parser.next()?;
            continue;
        }
        let key: Node;

        match &parser.current {
            Token::Variable(s) => {
                key = StringLiteral {
                    value: s.to_string(),
                };
            }
            Token::String(s) => {
                parser.is_arrow_function = IsArrowFunction::Impossible;
                key = StringLiteral {
                    value: s.to_string(),
                };
            }
            Token::Digit(s) => {
                parser.is_arrow_function = IsArrowFunction::Impossible;
                key = NumericLiteral {
                    value: s.to_string(),
                };
            }
            _ => {
                return Err("object property type error".to_string());
            }
        }
        parser.next()?;
        if is_ctrl_word(&parser.current, ":") {
            parser.next()?;
            if is_ctrl_word(&parser.current, "{") {
                properties.push(ObjectProperty {
                    key: Box::new(key),
                    value: build_possible_object(parser)?,
                });
            } else if is_ctrl_word(&parser.current, "[") {
                properties.push(ObjectProperty {
                    key: Box::new(key),
                    value: build_possible_array(parser)?,
                });
            } else {
                parser.is_arrow_function = IsArrowFunction::Impossible;
                properties.push(ObjectProperty {
                    key: Box::new(key),
                    value: parse_expression(parser, 2)?,
                });
            }
        } else if is_ctrl_word(&parser.current, "=") {
            parser.is_arrow_function = IsArrowFunction::Must;
            parser.next()?;
            let default_value = parse_expression(parser, 2)?;
            properties.push(AssignmentPattern {
                left: Box::new(key),
                right: default_value,
            })
        }
    }

    expect(parser, "}")?;
    Ok(Box::new(ObjectExpression { properties }))
}

fn build_possible_array(parser: &mut Parser) -> Result<Box<Node>, String> {
    let mut elements: Vec<Node> = vec![];
    parser.next()?;
    loop {
        if is_ctrl_word(&parser.current, "]") {
            break;
        } else if is_ctrl_word(&parser.current, ",") {
            parser.next()?;
            continue;
        } else if is_ctrl_word(&parser.current, "{") {
            elements.push(*build_possible_object(parser)?)
        } else if is_ctrl_word(&parser.current, "[") {
            elements.push(*build_possible_array(parser)?)
        } else {
            elements.push(*parse_expression(parser, 2)?)
        }
    }
    expect(parser, "]")?;
    Ok(Box::new(ArrayExpression { elements }))
}

fn convert_params(properties: Vec<Node>) {
    let mut result = vec![];
    for property in properties {
        if let ObjectProperty { key, value } = property {
            result.push(ObjectProperty { key, value })
        }
    }
}

#[cfg(test)]
mod test_arrow_function {
    use crate::exp::arrow_function_exp::{
        IsArrowFunction, build_possible_array, build_possible_arrow_function, build_possible_object,
    };
    use crate::parser::Parser;
    use crate::token::Token;

    #[test]
    fn arrow_function_param() -> Result<(), String> {
        let mut parser = Parser::new("{a=1}".to_string())?;
        let r = build_possible_object(&mut parser)?;
        assert_eq!(parser.is_arrow_function, IsArrowFunction::Must);
        println!("{:#?}", r);
        Ok(())
    }

    #[test]
    fn object() -> Result<(), String> {
        let mut parser = Parser::new("{a: {b: {c:1}}}".to_string())?;
        let r = build_possible_object(&mut parser)?;
        assert_eq!(parser.is_arrow_function, IsArrowFunction::Impossible);
        println!("{:#?}", r);
        Ok(())
    }

    #[test]
    fn arrow_function_or_object() -> Result<(), String> {
        let mut parser = Parser::new("{a: {b: {c=1}}}".to_string())?;
        let r = build_possible_object(&mut parser)?;
        assert_eq!(parser.is_arrow_function, IsArrowFunction::Must);
        println!("{:#?}", r);
        Ok(())
    }

    #[test]
    fn arrow_function_param_array() -> Result<(), String> {
        let mut parser = Parser::new("([{},a,b])".to_string())?;
        let r = build_possible_arrow_function(&mut parser)?;
        assert_eq!(parser.is_arrow_function, IsArrowFunction::Maybe);
        println!("{:#?}", r);
        Ok(())
    }

    #[test]
    fn arrow_function_param_array_object() -> Result<(), String> {
        let mut parser = Parser::new("[{x: 1},a,b]".to_string())?;
        let r = build_possible_array(&mut parser)?;
        assert_eq!(parser.is_arrow_function, IsArrowFunction::Impossible);
        println!("{:#?}", r);
        Ok(())
    }

    #[test]
    fn test_arrow_function_without_brackets() {
        let mut parser = Parser::new("let a = b => {}".to_string()).unwrap();
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_arrow_function() {
        let mut parser = Parser::new("let a = (b,c,d) => {}".to_string()).unwrap();
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_arrow_function2() {
        let mut parser = Parser::new("let a = ([a,b,c]) => {}".to_string()).unwrap();
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_arrow_function3() {
        let mut parser = Parser::new("let a = ({a: {b: {c=1}}}) => {}".to_string()).unwrap();
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_arrow_function_error() {
        let mut parser = Parser::new("let a = ({a: 1}) => {}".to_string()).unwrap();
        let ast = parser.parse();
        assert!(ast.is_err());
    }

    #[test]
    fn test_arrow_function_error2() {
        let mut parser = Parser::new("let a = ()".to_string()).unwrap();
        let ast = parser.parse();
        assert!(ast.is_err());
    }
}
