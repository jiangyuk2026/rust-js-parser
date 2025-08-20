use crate::exp::declaration_exp::build_let;
use crate::express::{expect, expect_keyword, is_ctrl_word, ok_box, parse_expression};
use crate::lex::Token;
use crate::node::Node;
use crate::node::Node::{EmptyStatement, ForStatement};
use crate::parser::Parser;

pub fn build_for(parser: &mut Parser) -> Result<Box<Node>, String> {
    let init: Box<Node>;
    let test: Box<Node>;
    let update: Box<Node>;
    let body: Box<Node>;
    expect_keyword(&parser.current, Token::For)?;
    parser.next();
    expect(&parser.current, "(")?;
    parser.next();
    if parser.current == Token::Let {
        init = build_let(parser)?;
    } else if is_ctrl_word(&parser.current, ";") {
        init = Box::new(EmptyStatement {});
    } else {
        init = parse_expression(parser, 0)?;
    }

    expect(&parser.current, ";")?;
    parser.next();
    if is_ctrl_word(&parser.current, ";") {
        test = Box::new(EmptyStatement {});
    } else {
        test = parse_expression(parser, 0)?;
    }

    expect(&parser.current, ";")?;
    parser.next();
    if is_ctrl_word(&parser.current, ")") {
        update = Box::new(EmptyStatement {});
    } else {
        update = parse_expression(parser, 0)?;
    }

    expect(&parser.current, ")")?;
    parser.next();
    if is_ctrl_word(&parser.current, "{") {
        body = Parser::parse_block(parser)?;
    } else if is_ctrl_word(&parser.current, ";") {
        body = Box::new(EmptyStatement {});
        parser.next();
    } else {
        return Err("for body error".to_string());
    }
    ok_box(ForStatement {
        init,
        test,
        update,
        body,
    })
}

#[cfg(test)]
mod test {
    use crate::lex::Token;
    use crate::parser::Parser;

    #[test]
    fn test_for() {
        let mut parser = Parser::new("for(let i =1; i < 10;i++) {}".to_string()).unwrap();
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_for_empty() {
        let mut parser = Parser::new("for(let i =1; i < 10;i++);".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_for_empty2() {
        let mut parser = Parser::new("for(let i =1; i < 10;i++);".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn for_body() {
        let mut parser = Parser::new("for(let i =1; i < 10;i++){let a = 1;let b= 2;}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(parser.current, Token::EOF)
    }
}
