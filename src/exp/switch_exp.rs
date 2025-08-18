use crate::express::{expect, expect_keyword, is_ctrl_word, ok_box, parse_expression};
use crate::lex::Token;
use crate::node::Node;
use crate::node::Node::{SwitchCase, SwitchStatement};
use crate::parser::Parser;

pub fn build_switch(parser: &mut Parser) -> Result<Box<Node>, String> {
    let discriminant: Box<Node>;
    let mut cases: Vec<Node> = vec![];

    expect_keyword(&parser.current, Token::Switch)?;
    parser.next();

    expect(&parser.current, "(")?;
    parser.next();

    discriminant = parse_expression(parser, 0)?;

    expect(&parser.current, ")")?;
    parser.next();

    expect(&parser.current, "{")?;
    parser.next();

    loop {
        if parser.current == Token::Case || parser.current == Token::Default {
            let test: Option<Box<Node>>;
            let mut consequent: Vec<Node>;

            if parser.current == Token::Case {
                parser.next();
                test = Some(parse_expression(parser, 0)?);
            } else {
                test = None;
                parser.next();
            }
            expect(&parser.current, ":")?;
            parser.next();

            if is_ctrl_word(&parser.current, "{") {
                consequent = vec![*Parser::parse_block(parser)?];
            } else {
                consequent = Parser::parse_statement_list(parser)?;
            }
            cases.push(SwitchCase { test, consequent });
        } else {
            break;
        }
    }

    expect(&parser.current, "}")?;
    parser.next();

    ok_box(SwitchStatement {
        discriminant,
        cases,
    })
}

#[cfg(test)]
mod test_switch_statement {
    use crate::lex::Token;
    use crate::parser::Parser;

    #[test]
    fn test_switch() {
        let mut parser = Parser::new("switch (a) {}".to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_case_empty() {
        let str = r#"
        switch (3) {
            case 1:
        }
        "#;
        let mut parser = Parser::new(str.to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_switch_case() {
        let str = r#"
        switch (a) {
            case 2: {
                let b = 2;
            }
            default: {
            }
        }"#;
        let mut parser = Parser::new(str.to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_switch_case_default() {
        let str = r#"
            switch (console.log()) {
                case 1:
                case 3:
                case 2: {

                    console.log(1)
                    console.log(1)
                    console.log(1)
                    console.log(1)
                }
                default:
                    break;
                case 3:
                    console.log(1)
                    console.log(1)
                    console.log(1)
                    console.log(1)
            }

            switch (a) {
                case 1:
                case 2: {
                    let b = 2;
                }
                default: {
                }
            }
        "#;
        let mut parser = Parser::new(str.to_string());
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }
}
