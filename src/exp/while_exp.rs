use crate::express::{expect, expect_keyword, is_ctrl_word, ok_box, parse_expression};
use crate::node::Node;
use crate::node::Node::{DoWhileStatement, EmptyStatement, IfStatement, WhileStatement};
use crate::parser::Parser;
use crate::token::Token;

pub fn build_while(parser: &mut Parser) -> Result<Box<Node>, String> {
    let test: Box<Node>;
    let body: Box<Node>;

    expect_keyword(&parser.current, Token::While)?;
    parser.next()?;
    parser.regex_allowed = true;
    expect(parser, "(")?;
    test = parse_expression(parser, 0)?;
    expect(parser, ")")?;
    body = parser.build_maybe_empty_body()?;
    ok_box(WhileStatement { test, body })
}

pub fn build_do_while(parser: &mut Parser) -> Result<Box<Node>, String> {
    let body: Box<Node>;
    let test: Box<Node>;
    expect_keyword(&parser.current, Token::Do)?;
    parser.next()?;
    body = Parser::parse_block(parser)?;
    expect_keyword(&parser.current, Token::While)?;
    parser.next()?;
    parser.regex_allowed = true;
    expect(parser, "(")?;
    test = parse_expression(parser, 0)?;
    expect(parser, ")")?;
    ok_box(DoWhileStatement { body, test })
}

#[cfg(test)]
mod test_while_statement {
    use crate::parser::Parser;
    use crate::token::Token;

    #[test]
    fn test_while() {
        let mut parser = Parser::new("while (a) {}".to_string()).unwrap();
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn test_while_empty() {
        let mut parser = Parser::new("while (1==1);".to_string()).unwrap();
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn test_do_while() {
        let mut parser = Parser::new("do{}while(1)".to_string()).unwrap();
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(*parser.current, Token::EOF)
    }
}
