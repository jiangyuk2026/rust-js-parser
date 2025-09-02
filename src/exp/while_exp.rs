use crate::express::{expect, expect_keyword, ok_box, parse_expression};
use crate::node::Node;
use crate::node::{DoWhileStatement, WhileStatement};
use crate::parser::Parser;
use crate::token::Token;

pub fn build_while(parser: &mut Parser) -> Result<Box<dyn Node>, String> {
    let test: Box<dyn Node>;
    let body: Box<dyn Node>;

    expect_keyword(&parser.current, Token::While)?;
    parser.next()?;
    parser.regex_allowed = true;
    expect(parser, "(")?;
    test = parse_expression(parser, 0)?;
    expect(parser, ")")?;
    body = parser.build_maybe_empty_body()?;
    Ok(Box::new(WhileStatement { test, body }))
}

pub fn build_do_while(parser: &mut Parser) -> Result<Box<dyn Node>, String> {
    let body: Box<dyn Node>;
    let test: Box<dyn Node>;
    expect_keyword(&parser.current, Token::Do)?;
    parser.next()?;
    body = Parser::parse_block(parser)?;
    expect_keyword(&parser.current, Token::While)?;
    parser.next()?;
    parser.regex_allowed = true;
    expect(parser, "(")?;
    test = parse_expression(parser, 0)?;
    expect(parser, ")")?;
    Ok(Box::new(DoWhileStatement { body, test }))
}

#[cfg(test)]
mod test_while_statement {
    use crate::parser::Parser;
    use crate::token::Token;

    #[test]
    fn test_while() {
        let mut parser = Parser::new("while (a) {}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn test_while_empty() {
        let mut parser = Parser::new("while (1==1);".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn test_do_while() {
        let mut parser = Parser::new("do{}while(1)".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }
}
