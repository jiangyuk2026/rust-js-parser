use crate::exp::declaration_exp::build_let;
use crate::express::{expect, expect_keyword, is_ctrl_word, ok_box, parse_expression};
use crate::node::Node;
use crate::node::Node::{
    EmptyStatement, ForInStatement, ForStatement, Identity, VariableDeclaration, VariableDeclarator,
};
use crate::parser::{IsForIn, Parser};
use crate::token::Token;

pub fn build_for(parser: &mut Parser) -> Result<Box<Node>, String> {
    let init: Box<Node>;
    let test: Box<Node>;
    let update: Box<Node>;
    expect_keyword(&parser.current, Token::For)?;
    parser.next()?;
    expect(parser, "(")?;

    parser.in_for_init = true;
    parser.is_for_in = IsForIn::Maybe;
    if parser.current == Token::Let
        || parser.current == Token::Var
        || parser.current == Token::Const
    {
        init = build_let(parser)?;
        if parser.current == Token::In {
            parser.is_for_in = IsForIn::Must;
            is_single_variable_without_value(&*init)?;
            parser.regex_allowed = true;
            parser.next()?;
        } else {
            parser.is_for_in = IsForIn::Impossible;
        }
    } else if let Token::Variable(_) = &parser.current {
        init = parse_expression(parser, 0)?;
        if parser.current == Token::In {
            if !matches!(*init, Identity { .. }) {
                return Err("for in: syntax error".to_string());
            }
            parser.regex_allowed = true;
            parser.next()?;
            parser.is_for_in = IsForIn::Must;
        } else {
            parser.is_for_in = IsForIn::Impossible;
        }
    } else {
        parser.is_for_in = IsForIn::Impossible;
        if is_ctrl_word(&parser.current, ";") {
            init = Box::new(EmptyStatement {});
        } else {
            init = parse_expression(parser, 0)?;
        }
    }
    parser.in_for_init = false;

    if parser.is_for_in == IsForIn::Must {
        let right = parse_expression(parser, 0)?;
        expect(parser, ")")?;
        let body = parser.build_maybe_empty_body()?;
        return ok_box(ForInStatement {
            left: init,
            right,
            body,
        });
    }

    expect(parser, ";")?;
    if is_ctrl_word(&parser.current, ";") {
        test = Box::new(EmptyStatement {});
    } else {
        test = parse_expression(parser, 0)?;
    }

    expect(parser, ";")?;
    if is_ctrl_word(&parser.current, ")") {
        update = Box::new(EmptyStatement {});
    } else {
        update = parse_expression(parser, 0)?;
    }
    expect(parser, ")")?;
    ok_box(ForStatement {
        init,
        test,
        update,
        body: parser.build_maybe_empty_body()?,
    })
}

fn is_single_variable_without_value(node: &Node) -> Result<bool, String> {
    if let VariableDeclaration { declarations, .. } = node {
        if declarations.len() != 1 {
            return Err("for in: syntax error, more than one variable".to_string());
        }
        if let VariableDeclarator { init, .. } = &declarations[0] {
            if init.is_some() {
                return Err("for in: syntax error".to_string());
            }
        }
    } else {
        panic!("unhandled variable declaration");
    }
    Ok(true)
}

#[cfg(test)]
mod test {
    use crate::parser::Parser;
    use crate::token::Token;

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
        let mut parser =
            Parser::new("for(let i =1; i < 10;i++){let a = 1;let b= 2;}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_for_in() {
        let mut parser = Parser::new("for(let i in {}) {}".to_string()).unwrap();
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_for_in_err1() {
        let mut parser = Parser::new("for(let i,b in {}) {}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(
            ast,
            Err("for in: syntax error, more than one variable".to_string())
        )
    }

    #[test]
    fn test_for_in_err2() {
        let mut parser = Parser::new("for(let i=1 in {}) {}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(ast, Err("for in: syntax error".to_string()))
    }
}
