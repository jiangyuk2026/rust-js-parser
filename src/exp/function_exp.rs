use crate::express::{expect, expect_keyword, is_ctrl_word, ok_box, parse_expression};
use crate::node::Node;
use crate::node::{
    ArrayPattern, AssignmentPattern, FunctionDeclaration, FunctionExpression, Identity,
    ObjectPattern, ObjectProperty,
};
use crate::parser::Parser;
use crate::token::Token;

pub fn build_function(parser: &mut Parser, is_declaration: bool) -> Result<Box<dyn Node>, String> {
    let id: Option<Box<dyn Node>>;
    let mut params;
    let body: Box<dyn Node>;

    let start_loc = parser.loc.clone();
    expect_keyword(&parser.current, Token::Function)?;
    parser.next()?;

    if let Token::Variable(s) = &*parser.current {
        id = Some(Box::new(Identity::new(
            s.to_string(),
            parser.loc.clone(),
            parser.loc.clone(),
        )));
        parser.next()?;
    } else if is_declaration {
        return Err("Expected function name".to_string());
    } else {
        id = None;
    }
    params = handle_function_params(parser)?;
    body = Parser::parse_block(parser)?;
    if is_declaration {
        return Ok(Box::new(FunctionDeclaration::new(
            id.unwrap(),
            params,
            body,
            start_loc,
            parser.last_loc.clone(),
        )));
    }
    Ok(Box::new(FunctionExpression::new(
        id,
        params,
        body,
        start_loc,
        parser.last_loc.clone(),
    )))
}

pub fn handle_function_params(parser: &mut Parser) -> Result<Vec<Box<dyn Node>>, String> {
    let mut params: Vec<Box<dyn Node>> = vec![];

    let start_loc = parser.loc.clone();
    expect(parser, "(")?;
    loop {
        if is_ctrl_word(&parser.current, ")") {
            break;
        } else if is_ctrl_word(&parser.current, ",") {
            parser.regex_allowed = true;
            parser.next()?;
            continue;
        } else if let Token::Variable(s) = &*parser.current {
            let param = Box::new(Identity::new(
                s.to_string(),
                parser.loc.clone(),
                parser.loc.clone(),
            ));
            parser.next()?;
            if is_ctrl_word(&parser.current, "=") {
                parser.regex_allowed = true;
                parser.next()?;
                let default_value = parse_expression(parser, 2)?;
                params.push(Box::new(AssignmentPattern::new(
                    param,
                    default_value,
                    parser.loc.clone(),
                    parser.loc.clone(),
                )));
            } else {
                params.push(param);
            }
        } else if is_ctrl_word(&parser.current, "{") {
            params.push(handle_object(parser)?);
        } else if is_ctrl_word(&parser.current, "[") {
            params.push(handle_array(parser)?);
        }
    }

    expect(parser, ")")?;
    Ok(params)
}

fn handle_object(parser: &mut Parser) -> Result<Box<dyn Node>, String> {
    if !is_ctrl_word(&parser.current, "{") {
        return Err("function handle_object expect {".to_string());
    }
    let start_loc = parser.loc.clone();
    parser.next()?;
    let mut properties: Vec<Box<dyn Node>> = vec![];
    loop {
        if is_ctrl_word(&parser.current, "}") {
            break;
        } else if let Token::Variable(s) = &*parser.current {
            let name = s.to_string();
            let property_start_loc = parser.loc.clone();
            let key = Box::new(Identity::new(
                name.to_string(),
                parser.loc.clone(),
                parser.loc.clone(),
            ));
            parser.next()?;
            if is_ctrl_word(&parser.current, ":") {
                parser.regex_allowed = true;
                parser.next()?;
                if is_ctrl_word(&parser.current, "{") {
                    let right = handle_object(parser)?;
                    properties.push(Box::new(ObjectProperty::new(
                        key,
                        right,
                        property_start_loc,
                        parser.last_loc.clone(),
                    )))
                } else if is_ctrl_word(&parser.current, "[") {
                    properties.push(Box::new(ObjectProperty::new(
                        key,
                        handle_array(parser)?,
                        property_start_loc.clone(),
                        parser.last_loc.clone(),
                    )))
                } else {
                    return Err("handle_object expect { or [ after :".to_string());
                }
            } else if is_ctrl_word(&parser.current, "=") {
                parser.regex_allowed = true;
                parser.next()?;
                let right = parse_expression(parser, 2)?;
                properties.push(Box::new(ObjectProperty::new(
                    Box::new(Identity::new(
                        name.to_string(),
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    )),
                    Box::new(AssignmentPattern::new(
                        Box::new(Identity::new(
                            name.to_string(),
                            start_loc.clone(),
                            parser.last_loc.clone(),
                        )),
                        right,
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    )),
                    start_loc.clone(),
                    parser.last_loc.clone(),
                )))
            } else if is_ctrl_word(&parser.current, ",") {
                parser.regex_allowed = true;
                parser.next()?;
                properties.push(Box::new(ObjectProperty::new(
                    Box::new(Identity::new(
                        name.to_string(),
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    )),
                    Box::new(Identity::new(
                        name.to_string(),
                        start_loc.clone(),
                        parser.last_loc.clone(),
                    )),
                    start_loc.clone(),
                    parser.last_loc.clone(),
                )))
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
    parser.next()?;
    Ok(Box::new(ObjectPattern::new(
        properties,
        start_loc.clone(),
        parser.last_loc.clone(),
    )))
}

fn handle_array(parser: &mut Parser) -> Result<Box<dyn Node>, String> {
    let mut elements: Vec<Box<dyn Node>> = vec![];
    if !is_ctrl_word(&parser.current, "[") {
        return Err("function handle_array expect [".to_string());
    }
    let start_loc = parser.loc.clone();
    parser.next()?;
    loop {
        if is_ctrl_word(&parser.current, "]") {
            break;
        } else if is_ctrl_word(&parser.current, ",") {
            parser.next()?;
        } else if let Token::Variable(s) = &*parser.current {
            let param_loc_start = parser.loc.clone();
            let name = Box::new(Identity::new(
                s.to_string(),
                parser.loc.clone(),
                parser.loc.clone(),
            ));
            parser.next()?;
            if is_ctrl_word(&parser.current, "=") {
                parser.next()?;
                elements.push(Box::new(AssignmentPattern::new(
                    name,
                    parse_expression(parser, 2)?,
                    param_loc_start,
                    parser.last_loc.clone(),
                )));
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
    parser.next()?;
    Ok(Box::new(ArrayPattern::new(
        elements,
        start_loc.clone(),
        parser.last_loc.clone(),
    )))
}

#[cfg(test)]
mod test {
    use crate::parser::Parser;
    use crate::token::Token;

    #[test]
    fn test_function() {
        let mut parser = Parser::new("function a() {}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn function_expression_with_name() {
        let mut parser = Parser::new("let a = function a() {}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn function_expression_without_name() {
        let mut parser = Parser::new("let a = function () {}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn test_function_param() {
        let mut parser = Parser::new("function a(b=1) {}".to_string()).unwrap();
        let ast = parser.parse();
        if ast.is_err() {
            println!("{:#?}", ast.err().unwrap());
        }
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn test_function_param2() {
        let mut parser = Parser::new("function a(b=1, c) {}".to_string()).unwrap();
        let ast = parser.parse();
        assert_eq!(*parser.current, Token::EOF)
    }

    #[test]
    fn test_function_body() -> Result<(), String> {
        let mut parser = Parser::new("function a(b=1, c) {let z = 1}".to_string()).unwrap();
        let ast = parser.parse()?;
        assert_eq!(*parser.current, Token::EOF);
        Ok(())
    }

    #[test]
    fn test_function_deep() -> Result<(), String> {
        let mut parser = Parser::new("function a({a=2}) {let z = 1}".to_string()).unwrap();
        let ast = parser.parse()?;
        assert_eq!(*parser.current, Token::EOF);
        Ok(())
    }

    #[test]
    fn test_function_deep2() -> Result<(), String> {
        let mut parser = Parser::new("function a({b: {c = 3}}) {}".to_string()).unwrap();
        let ast = parser.parse()?;
        assert_eq!(*parser.current, Token::EOF);
        Ok(())
    }

    #[test]
    fn test_function_array() -> Result<(), String> {
        let mut parser = Parser::new("function a([a,b,c]) {let z = 1}".to_string()).unwrap();
        let ast = parser.parse()?;
        assert_eq!(*parser.current, Token::EOF);
        Ok(())
    }

    #[test]
    fn test_function_array2() -> Result<(), String> {
        let mut parser =
            Parser::new("function a([b = {c: 3}], d) {let z = 1}".to_string()).unwrap();
        let ast = parser.parse()?;
        assert_eq!(*parser.current, Token::EOF);
        Ok(())
    }

    #[test]
    fn test_function_mix() -> Result<(), String> {
        let mut parser =
            Parser::new("function b(c, {d: {e: [f, g, {h = 3}]}}) {}".to_string()).unwrap();
        let ast = parser.parse()?;
        assert_eq!(*parser.current, Token::EOF);
        Ok(())
    }

    #[test]
    fn test_function_array3() -> Result<(), String> {
        let mut parser = Parser::new("function a2([[b,c,d]]) {}".to_string()).unwrap();
        let ast = parser.parse()?;
        assert_eq!(*parser.current, Token::EOF);
        Ok(())
    }
}
