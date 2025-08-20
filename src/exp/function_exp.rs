use crate::express::{expect, expect_keyword, is_ctrl_word, ok_box, parse_expression};
use crate::lex::Token;
use crate::node::Node;
use crate::node::Node::{
    ArrayPattern, AssignmentPattern, FunctionDeclaration, FunctionExpression, Identity,
    ObjectPattern, ObjectProperty,
};
use crate::parser::Parser;

pub fn build_function(parser: &mut Parser, is_declaration: bool) -> Result<Box<Node>, String> {
    let id: Option<Box<Node>>;
    let mut params;
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
    params = handle_function_params(parser)?;
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

pub fn handle_function_params(parser: &mut Parser) -> Result<Vec<Node>, String> {
    let mut params: Vec<Node> = vec![];

    expect(&parser.current, "(")?;
    parser.next();
    loop {
        if is_ctrl_word(&parser.current, ")") {
            break;
        } else if is_ctrl_word(&parser.current, ",") {
            parser.next();
            continue;
        } else if let Token::Variable(s) = &parser.current {
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
            } else {
                params.push(param);
            }
        } else if is_ctrl_word(&parser.current, "{") {
            params.push(handle_object(parser)?);
        } else if is_ctrl_word(&parser.current, "[") {
            params.push(handle_array(parser)?);
        }
    }

    expect(&parser.current, ")")?;
    parser.next();
    Ok(params)
}

fn handle_object(parser: &mut Parser) -> Result<Node, String> {
    if !is_ctrl_word(&parser.current, "{") {
        return Err("function handle_object expect {".to_string());
    }
    parser.next();
    let mut properties = vec![];
    loop {
        if is_ctrl_word(&parser.current, "}") {
            break;
        } else if let Token::Variable(s) = &parser.current {
            let name = s.to_string();
            parser.next();
            if is_ctrl_word(&parser.current, ":") {
                parser.next();
                if is_ctrl_word(&parser.current, "{") {
                    let right = handle_object(parser)?;
                    properties.push(ObjectProperty {
                        key: Box::new(Identity {
                            name: name.to_string(),
                        }),
                        value: Box::new(right),
                    })
                } else if is_ctrl_word(&parser.current, "[") {
                    properties.push(ObjectProperty {
                        key: Box::new(Identity {
                            name: name.to_string(),
                        }),
                        value: Box::new(handle_array(parser)?),
                    })
                } else {
                    return Err("handle_object expect { or [ after :".to_string());
                }
            } else if is_ctrl_word(&parser.current, "=") {
                parser.next();
                let right = parse_expression(parser, 2)?;
                properties.push(ObjectProperty {
                    key: Box::new(Identity {
                        name: name.to_string(),
                    }),
                    value: Box::new(AssignmentPattern {
                        left: Box::new(Identity {
                            name: name.to_string(),
                        }),
                        right,
                    }),
                })
            } else if is_ctrl_word(&parser.current, ",") {
                parser.next();
                properties.push(ObjectProperty {
                    key: Box::new(Identity {
                        name: name.to_string(),
                    }),
                    value: Box::new(Identity {
                        name: name.to_string(),
                    }),
                })
            } else {
                return Err("handle_object syntax error".to_string());
            }
        } else {
            return Err("handle_object expect variable".to_string());
        }
    }
    if !is_ctrl_word(&parser.current, "}") {
        return Err("function param expect }".to_string());
    }
    parser.next();
    Ok(ObjectPattern { properties })
}

fn handle_array(parser: &mut Parser) -> Result<Node, String> {
    let mut elements = vec![];
    if !is_ctrl_word(&parser.current, "[") {
        return Err("function handle_array expect [".to_string());
    }
    parser.next();
    loop {
        if is_ctrl_word(&parser.current, "]") {
            break;
        } else if is_ctrl_word(&parser.current, ",") {
            parser.next();
        } else if let Token::Variable(s) = &parser.current {
            let name = Identity {
                name: s.to_string(),
            };
            parser.next();
            if is_ctrl_word(&parser.current, "=") {
                parser.next();
                elements.push(AssignmentPattern {
                    left: Box::new(name),
                    right: parse_expression(parser, 2)?,
                });
            } else {
                elements.push(name);
            }
        } else if is_ctrl_word(&parser.current, "{") {
            elements.push(handle_object(parser)?);
        } else if is_ctrl_word(&parser.current, "[") {
            elements.push(handle_array(parser)?);
        } else {
            return Err("handle_array syntax error".to_string());
        }
    }
    if !is_ctrl_word(&parser.current, "]") {
        return Err("function handle_array expect ]".to_string());
    }
    parser.next();
    Ok(ArrayPattern { elements })
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

    #[test]
    fn test_function_deep() -> Result<(), String> {
        let mut parser = Parser::new("function a({a=2}) {let z = 1}".to_string());
        let ast = parser.parse()?;
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF);
        Ok(())
    }

    #[test]
    fn test_function_deep2() -> Result<(), String> {
        let mut parser = Parser::new("function a({b: {c = 3}}) {}".to_string());
        let ast = parser.parse()?;
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF);
        Ok(())
    }

    #[test]
    fn test_function_array() -> Result<(), String> {
        let mut parser = Parser::new("function a([a,b,c]) {let z = 1}".to_string());
        let ast = parser.parse()?;
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF);
        Ok(())
    }

    #[test]
    fn test_function_array2() -> Result<(), String> {
        let mut parser = Parser::new("function a([b = {c: 3}], d) {let z = 1}".to_string());
        let ast = parser.parse()?;
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF);
        Ok(())
    }

    #[test]
    fn test_function_mix() -> Result<(), String> {
        let mut parser = Parser::new("function b(c, {d: {e: [f, g, {h = 3}]}}) {}".to_string());
        let ast = parser.parse()?;
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF);
        Ok(())
    }

    #[test]
    fn test_function_array3() -> Result<(), String> {
        let mut parser = Parser::new("function a2([[b,c,d]]) {}".to_string());
        let ast = parser.parse()?;
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF);
        Ok(())
    }
}
