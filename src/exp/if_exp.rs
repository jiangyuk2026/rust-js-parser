use crate::express::{expect, expect_keyword, is_ctrl_word, ok_box, parse_expression};
use crate::lex::Token;
use crate::node::Node;
use crate::node::Node::{BlockStatement, EmptyStatement, IfStatement};
use crate::parser::Parser;

pub fn build_if(parser: &mut Parser) -> Result<Box<Node>, String> {
    let test: Box<Node>;
    let consequent: Box<Node>;
    let alternate: Option<Box<Node>>;

    expect_keyword(&parser.current, Token::If)?;
    parser.next()?;
    expect(parser, "(")?;
    test = parse_expression(parser, 0)?;
    expect(parser, ")")?;

    if is_ctrl_word(&parser.current, "{") {
        consequent = Parser::parse_block(parser)?;
    } else if is_ctrl_word(&parser.current, ";") {
        consequent = Box::new(EmptyStatement {});
        parser.next()?;
    } else {
        return Err("if syntax error".to_string());
    }

    if parser.current == Token::Else {
        parser.next()?;
        if is_ctrl_word(&parser.current, "{") {
            alternate = Some(Parser::parse_block(parser)?);
        } else {
            alternate = Some(Box::new(EmptyStatement {}));
        }
    } else {
        alternate = None;
    }
    ok_box(IfStatement {
        test,
        consequent,
        alternate,
    })
}

#[cfg(test)]
mod test_if_statement {
    use crate::lex::Token;
    use crate::parser::Parser;

    #[test]
    fn test_if() {
        let mut parser = Parser::new("if (a) {}".to_string()).unwrap();
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }


    #[test]
    fn test_if_else() {
        let mut parser = Parser::new("if (a) {} else {let b = 1;}".to_string()).unwrap();
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }
}
