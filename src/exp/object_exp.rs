use crate::exp::function_exp::handle_function_params;
use crate::express::{expect, is_ctrl_word, ok_box, parse_expression};
use crate::node::Node;
use crate::node::{
    Identity, NumericLiteral, ObjectExpression, ObjectMethod, ObjectProperty, StringLiteral,
};
use crate::parser::Parser;
use crate::token::{Token, is_keyword};

pub fn build_object(parser: &mut Parser) -> Result<Box<dyn Node>, String> {
    let mut properties: Vec<Box<dyn Node>> = vec![];

    expect(parser, "{")?;

    loop {
        if is_ctrl_word(&parser.current, "}") {
            break;
        } else if is_ctrl_word(&parser.current, ",") {
            parser.regex_allowed = true;
            parser.next()?;
            continue;
        }
        let key: Box<dyn Node>;

        if is_keyword(&parser.current) {
            key = Box::new(Identity {
                name: parser.current.to_string(),
            })
        } else {
            match &*parser.current {
                Token::Variable(s) => {
                    key = Box::new(StringLiteral {
                        value: s.to_string(),
                    });
                }
                Token::String(s) => {
                    key = Box::new(StringLiteral {
                        value: s.to_string(),
                    });
                }
                Token::Digit(s) => {
                    key = Box::new(NumericLiteral {
                        value: s.to_string(),
                    });
                }
                _ => {
                    return Err("object property type error".to_string());
                }
            }
        }
        parser.next()?;
        if is_ctrl_word(&parser.current, ",") {
            properties.push(Box::new(ObjectProperty {
                key: key.clone(),
                value: key,
            }));
        } else if is_ctrl_word(&parser.current, "(") {
            let params = handle_function_params(parser)?;
            let body = Parser::parse_block(parser)?;
            properties.push(Box::new(ObjectMethod {
                key,
                params,
                body,
            }))
        } else if is_ctrl_word(&parser.current, ":") {
            parser.regex_allowed = true;
            parser.next()?;
            properties.push(Box::new(ObjectProperty {
                key,
                value: parse_expression(parser, 2)?,
            }));
        }
    }

    expect(parser, "}")?;
    Ok(Box::new(ObjectExpression { properties }))
}

#[cfg(test)]
mod test_object {
    use crate::parser::Parser;
    use crate::token::Token;

    #[test]
    fn test_empty() {
        let mut parser = Parser::new("a = {}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn test_object_keyword() {
        let mut parser = Parser::new("a = {return : 1}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn test_object() {
        let mut parser = Parser::new("a = {b: 1,c:2}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn test_object_simple() {
        let mut parser = Parser::new("a = {b,c}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn test_object_method() {
        let mut parser = Parser::new("a = {b(c){}}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn test_object_call() {
        let mut parser = Parser::new("a = {b: 1,c:d({})}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn test_object_deep() {
        let mut parser = Parser::new("a = {b: 1,c: {d: 2}}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }
}
