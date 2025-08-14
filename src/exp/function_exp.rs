use crate::express::{expect, expect_keyword, is_ctrl_word, ok_box, parse_expression};
use crate::lex::Token;
use crate::node::Node;
use crate::node::Node::{
    AssignmentPattern, BlockStatement, FunctionDeclaration, FunctionExpression, Identity,
};
use crate::parser::Parser;

pub fn build_function(parser: &mut Parser, is_declaration: bool) -> Result<Box<Node>, String> {
    let id: Option<Box<Node>>;
    let mut params: Vec<Node> = vec![];
    let body: Box<Node>;

    expect_keyword(&parser.current, Token::Function)?;
    parser.next();

    if let Token::Variable(s) = &parser.current {
        id = Some(Box::new(Identity {
            name: s.to_string(),
        }));
        parser.next();
    } else if is_declaration {
        return Err("Expected function name".to_string());
    } else {
        id = None;
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
                    let default_value = parse_expression(parser, 2)?;
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
    body = Parser::parse_block(parser)?;
    if is_declaration {
        return ok_box(FunctionDeclaration {
            id: id.unwrap(),
            params,
            body,
        });
    }
    ok_box(FunctionExpression { id, params, body })
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
    fn function_expression_with_name() {
        let mut parser = Parser::new("let a = function a() {}".to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn function_expression_without_name() {
        let mut parser = Parser::new("let a = function () {}".to_string());
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
