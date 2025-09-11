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

    let start_loc = parser.loc.clone();
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
            key = Box::new(Identity::new(
                parser.current.to_string(),
                start_loc.clone(),
                parser.last_loc.clone(),
            ))
        } else {
            match &*parser.current {
                Token::Variable(s) => {
                    key = Box::new(Identity::new(
                        s.to_string(),
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    ));
                }
                Token::String(s, is_single_quoted) => {
                    key = Box::new(StringLiteral::new(
                        s.to_string(),
                        *is_single_quoted,
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    ));
                }
                Token::Digit(s) => {
                    key = Box::new(NumericLiteral::new(
                        s.to_string(),
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    ));
                }
                _ => {
                    return Err("object property type error".to_string());
                }
            }
        }
        parser.next()?;
        if is_ctrl_word(&parser.current, ",") {
            properties.push(Box::new(ObjectProperty::new(
                key.clone(),
                key,
                start_loc.clone(),
                parser.last_loc.clone(),
            )));
        } else if is_ctrl_word(&parser.current, "(") {
            let params = handle_function_params(parser)?;
            let body = Parser::parse_block(parser)?;
            properties.push(Box::new(ObjectMethod::new(
                key,
                params,
                body,
                start_loc.clone(),
                parser.last_loc.clone(),
            )))
        } else if is_ctrl_word(&parser.current, ":") {
            parser.regex_allowed = true;
            parser.next()?;
            properties.push(Box::new(ObjectProperty::new(
                key,
                parse_expression(parser, 2)?,
                start_loc.clone(),
                parser.last_loc.clone(),
            )));
        }
    }

    expect(parser, "}")?;
    Ok(Box::new(ObjectExpression::new(
        properties,
        start_loc.clone(),
        parser.last_loc.clone(),
    )))
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
