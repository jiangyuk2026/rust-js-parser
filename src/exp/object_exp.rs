use crate::express::{expect, is_ctrl_word, ok_box, parse_expression};
use crate::lex::Token;
use crate::node::Node;
use crate::node::Node::{NumericLiteral, ObjectExpression, ObjectProperty, StringLiteral};
use crate::parser::Parser;

pub fn build_object(parser: &mut Parser) -> Result<Box<Node>, String> {
    let mut properties = vec![];

    expect(&parser.current, "{")?;
    parser.next();

    loop {
        if is_ctrl_word(&parser.current, "}") {
            break;
        } else if is_ctrl_word(&parser.current, ",") {
            parser.next();
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
                key = StringLiteral {
                    value: s.to_string(),
                };
            }
            Token::Digit(s) => {
                key = NumericLiteral {
                    value: s.to_string(),
                };
            }
            _ => {
                return Err("object property type error".to_string());
            }
        }
        parser.next();
        if is_ctrl_word(&parser.current, ",") {
            properties.push(ObjectProperty {
                key: Box::new(key.clone()),
                value: Box::new(key),
            });
            continue;
        } else if is_ctrl_word(&parser.current, "}") {
            continue;
        }
        expect(&parser.current, ":")?;
        parser.next();
        properties.push(ObjectProperty {
            key: Box::new(key),
            value: parse_expression(parser, 2)?,
        });
    }

    expect(&parser.current, "}")?;
    parser.next();

    ok_box(ObjectExpression { properties })
}

#[cfg(test)]
mod test_object {
    use crate::lex::Token;
    use crate::parser::Parser;

    #[test]
    fn test_empty() {
        let mut parser = Parser::new("a = {}".to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_object() {
        let mut parser = Parser::new("a = {b: 1,c:2}".to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_object_simple() {
        let mut parser = Parser::new("a = {b,c}".to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_object_call() {
        let mut parser = Parser::new("a = {b: 1,c:d({})}".to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_object_deep() {
        let mut parser = Parser::new("a = {b: 1,c: {d: 2}}".to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }
}
