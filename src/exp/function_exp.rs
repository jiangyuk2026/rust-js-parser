use crate::express::{expect, expect_keyword, is_ctrl_word, ok_box, parse_expression};
use crate::lex::Token;
use crate::node::Node;
use crate::node::Node::{AssignmentPattern, BlockStatement, FunctionDeclaration, Identity};
use crate::parser::Parser;

pub fn build_function(parser: &mut Parser) -> Result<Box<Node>, String> {
    let id: Box<Node>;
    let mut params: Vec<Node> = vec![];
    let body: Vec<Node>;

    expect_keyword(&parser.current, Token::Function)?;
    parser.next();

    if let Token::Variable(s) = &parser.current {
        id = Box::new(Identity {
            name: s.to_string(),
        });
        parser.next();
    } else {
        return Err("Expected function name".to_string());
    }

    expect(&parser.current, "(")?;
    parser.next();

    if !is_ctrl_word(&parser.current, ")") {
        loop {
            if let Token::Variable(s) = &parser.current {
                let param = Identity {
                    name: s.to_string(),
                };
                parser.next();
                if is_ctrl_word(&parser.current, "=") {
                    parser.next();
                    let default_value = parse_expression(parser, 1)?;
                    params.push(AssignmentPattern {
                        left: Box::new(param),
                        right: default_value,
                    });
                    if is_ctrl_word(&parser.current, ",") {
                        parser.next();
                        continue;
                    } else if is_ctrl_word(&parser.current, ")") {
                        break;
                    }
                } else if is_ctrl_word(&parser.current, ",") {
                    params.push(param);
                    parser.next();
                    continue;
                } else if is_ctrl_word(&parser.current, ")") {
                    params.push(param);
                    break;
                } else {
                    return Err("function param syntax error".to_string());
                }
            } else {
                return Err("Expected function param".to_string());
            }
        }
    }

    expect(&parser.current, ")")?;
    parser.next();
    expect(&parser.current, "{")?;
    parser.next();

    if is_ctrl_word(&parser.current, "}") {
        body = vec![];
    } else {
        body = Parser::parse_statement_list(parser)?;
    }

    expect(&parser.current, "}")?;
    parser.next();

    ok_box(FunctionDeclaration {
        id,
        params,
        body: Box::new(BlockStatement { body }),
    })
}

#[cfg(test)]
mod test {
    use crate::lex::Token;
    use crate::parser::Parser;

    #[test]
    fn test_function() {
        let mut parser = Parser::new("function a() {}".to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_function_param() {
        let mut parser = Parser::new("function a(b=1) {}".to_string());
        let ast = parser.parse();
        if ast.is_err() {
            println!("{:#?}", ast.err().unwrap());
        }
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_function_param2() {
        let mut parser = Parser::new("function a(b=1, c) {}".to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_function_body() -> Result<(), String> {
        let mut parser = Parser::new("function a(b=1, c) {let z = 1}".to_string());
        let ast = parser.parse()?;
        assert_eq!(parser.current, Token::EOF);
        Ok(())
    }
}
