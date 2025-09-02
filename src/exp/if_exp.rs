use crate::express::{expect, expect_keyword, parse_expression};
use crate::node::Node;
use crate::node::IfStatement;
use crate::parser::Parser;
use crate::token::Token;

pub fn build_if(parser: &mut Parser) -> Result<Box<dyn Node>, String> {
    let test: Box<dyn Node>;
    let consequent: Box<dyn Node>;
    let alternate: Option<Box<dyn Node>>;

    expect_keyword(&parser.current, Token::If)?;
    parser.next()?;
    parser.regex_allowed = true;
    expect(parser, "(")?;
    test = parse_expression(parser, 0)?;
    expect(parser, ")")?;

    consequent = parser.build_maybe_empty_body()?;

    if *parser.current == Token::Else {
        parser.regex_allowed = true;
        parser.next()?;
        if *parser.current == Token::If {
            alternate = Some(build_if(parser)?);
        } else {
            alternate = Some(parser.build_maybe_empty_body()?);
        }
    } else {
        alternate = None;
    }
    Ok(Box::new(IfStatement {
        test,
        consequent,
        alternate,
    }))
}

#[cfg(test)]
mod test_if_statement {
    use crate::parser::Parser;
    use crate::token::Token;

    #[test]
    fn test_if() {
        let mut parser = Parser::new("if (a) {}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn test_if_else() {
        let mut parser = Parser::new("if (a) {} else {let b = 1;}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn test_if_else_if() {
        let mut parser = Parser::new("if (1) {} else if(2){} else {}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }
}
