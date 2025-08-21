use crate::express::{expect, is_ctrl_word, ok_box, parse_expression};
use crate::node::Node;
use crate::node::Node::ArrayExpression;
use crate::parser::Parser;

pub fn build_array(parser: &mut Parser) -> Result<Box<Node>, String> {
    let mut elements = vec![];

    expect(parser, "[")?;
    loop {
        if is_ctrl_word(&parser.current, "]") {
            break;
        } else if is_ctrl_word(&parser.current, ",") {
            parser.next()?;
            continue;
        }
        let item = parse_expression(parser, 2)?;
        elements.push(*item);
    }
    expect(parser, "]")?;
    ok_box(ArrayExpression { elements })
}

#[cfg(test)]
mod test_array {
    use crate::parser::Parser;
    use crate::token::Token;

    #[test]
    fn test_empty() {
        let mut parser = Parser::new("a = []".to_string()).unwrap();
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_object() {
        let mut parser = Parser::new("a = [1,2,3]".to_string()).unwrap();
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }

    #[test]
    fn test_object_call() {
        let mut parser = Parser::new("a = [1,2,3, [4,5]]".to_string()).unwrap();
        let ast = parser.parse();
        println!("{ast:#?}");
        assert_eq!(parser.current, Token::EOF)
    }
}
