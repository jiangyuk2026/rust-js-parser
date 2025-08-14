use crate::express::{expect_keyword, is_ctrl_word, ok_box};
use crate::lex::Token;
use crate::node::Node;
use crate::node::Node::{CatchClause, Identity, TryStatement};
use crate::parser::Parser;

pub fn build_try(parser: &mut Parser) -> Result<Box<Node>, String> {
    let block: Box<Node>;
    let handle: Option<Box<Node>>;
    let finalizer: Option<Box<Node>>;

    expect_keyword(&parser.current, Token::Try)?;
    parser.next();

    block = Parser::parse_block(parser)?;

    if parser.current == Token::Catch {
        let param: Option<Box<Node>>;
        let body: Box<Node>;
        parser.next();
        if is_ctrl_word(&parser.current, "(") {
            parser.next();
            if is_ctrl_word(&parser.current, ")") {
                param = None;
                parser.next();
            } else if is_ctrl_word(&parser.current, "{") {
                return Err("catch({}) unsupported now".to_string());
            } else if let Token::Variable(s) = &parser.current {
                param = Some(Box::new(Identity {
                    name: s.to_string(),
                }));
                parser.next();
                if is_ctrl_word(&parser.current, ")") {
                    parser.next();
                } else if is_ctrl_word(&parser.current, ",") {
                    return Err("catch support only one param".to_string());
                }
            } else {
                return Err("catch param error".to_string());
            }
            body = Parser::parse_block(parser)?;
            handle = Some(Box::new(CatchClause { param, body }))
        } else if is_ctrl_word(&parser.current, "{") {
            body = Parser::parse_block(parser)?;
            handle = Some(Box::new(CatchClause { param: None, body }))
        } else {
            return Err("catch syntax error".to_string());
        }
        if parser.current == Token::Finally {
            parser.next();
            finalizer = Some(Parser::parse_block(parser)?);
        } else {
            finalizer = None;
        }
    } else if parser.current == Token::Finally {
        handle = None;
        parser.next();
        finalizer = Some(Parser::parse_block(parser)?);
    } else {
        return Err("expect catch or finally".to_string());
    }

    ok_box(TryStatement {
        block,
        handle,
        finalizer,
    })
}

#[cfg(test)]
mod test_try_statement {
    use crate::lex::Token;
    use crate::parser::Parser;

    #[test]
    fn try_catch_no_param() {
        let mut parser = Parser::new("try {} catch() {}".to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn try_catch() {
        let mut parser = Parser::new("try {} catch(a) {}".to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn try_catch_finally() {
        let mut parser = Parser::new("try {} catch(a) {} finally {}".to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn try_finally() {
        let mut parser = Parser::new("try {} finally {}".to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }
}
