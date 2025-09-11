use crate::express::{expect_keys, is_ctrl_word, parse_expression};
use crate::node::{Identity, Node};
use crate::node::{VariableDeclaration, VariableDeclarator};
use crate::parser::Parser;
use crate::token::Token;

pub fn build_let(parser: &mut Parser) -> Result<Box<dyn Node>, String> {
    let start_loc = parser.loc.clone();
    let kind = expect_keys(&parser.current, &vec![Token::Var, Token::Let, Token::Const])?;
    parser.next()?;
    let mut declarations = vec![];
    declarations.push(build_declarator(parser)?);
    loop {
        let c2 = &*parser.current;
        match c2 {
            Token::Control(s) => match s.as_str() {
                "," => {
                    parser.regex_allowed = true;
                    parser.next()?;
                    declarations.push(build_declarator(parser)?);
                }
                _ => break,
            },
            _ => break,
        }
    }
    if !parser.in_for_init && is_ctrl_word(&parser.current, ";") {
        parser.next()?;
    }
    Ok(Box::new(VariableDeclaration::new(
        kind,
        declarations,
        start_loc,
        parser.loc.clone(),
    )))
}

fn build_declarator(parser: &mut Parser) -> Result<Box<dyn Node>, String> {
    let start_loc = parser.loc.clone();
    let id = &*parser.current;
    if let Token::Variable(s) = id {
        let id = Box::new(Identity::new(s.to_string(), start_loc.clone(), parser.loc.clone()));
        parser.next()?;
        let equal = &parser.current;
        if !is_ctrl_word(equal, "=") {
            return Ok(Box::new(VariableDeclarator::new(
                id,
                None,
                start_loc,
                parser.last_loc.clone(),
            )));
        }
        parser.regex_allowed = true;
        parser.next()?;
        return Ok(Box::new(VariableDeclarator::new(
            id,
            Some(parse_expression(parser, 1)?),
            start_loc,
            parser.last_loc.clone(),
        )));
    }
    Err(format!("expect Variable, find {id}"))
}

#[cfg(test)]
mod test_let {
    use super::*;

    #[test]
    fn test_digit() {
        let mut parser = Parser::new("let a = 1".to_string()).unwrap();

        let result = build_let(&mut parser);
    }

    #[test]
    fn test_string() {
        let mut parser = Parser::new("let a = \"abce\"".to_string()).unwrap();

        let result = build_let(&mut parser);
    }

    #[test]
    fn test_express() {
        let mut parser = Parser::new("let a = 1 + 2".to_string()).unwrap();

        let result = build_let(&mut parser);
    }

    #[test]
    fn test_comma() {
        let mut parser = Parser::new("let a = 3, b = 2".to_string()).unwrap();

        let result = build_let(&mut parser);
    }
}
