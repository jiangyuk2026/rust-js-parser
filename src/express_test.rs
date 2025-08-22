#[cfg(test)]
mod test {
    use crate::node::Node::*;
    use crate::parser::Parser;
    use crate::token::Token;

    #[test]
    fn test_dot() {
        let mut parser = Parser::new("a.b.c".to_string()).unwrap();
        let ast = parser.parse();
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_array() {
        let mut parser = Parser::new("a = [1,2,3]".to_string()).unwrap();
        let ast = parser.parse();
        println!("{:?}", ast);
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_equal() {
        let mut parser = Parser::new("a = b = c".to_string()).unwrap();
        let ast = parser.parse();
        println!("{:#?}", ast);
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_brackets() {
        let mut parser = Parser::new("a  = ((1))".to_string()).unwrap();
        let ast = parser.parse();
        println!("{:#?}", ast);
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_comma1() {
        let mut parser = Parser::new("a,b".to_string()).unwrap();
        let ast = parser.parse();
        println!("{:#?}", ast);
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_comma2() {
        let mut parser = Parser::new("a,b,c".to_string()).unwrap();
        let ast = parser.parse();
        println!("{:#?}", ast);
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_array2() -> Result<(), String> {
        let mut parser = Parser::new("a[b[2]]".to_string())?;
        let ast = parser.parse()?;
        assert_eq!(parser.current, Token::EOF);
        Ok(())
    }

    #[test]
    fn test_typeof_typeof() -> Result<(), String> {
        let mut parser = Parser::new("typeof typeof a".to_string())?;
        let ast = parser.parse()?;
        println!("{:#?}", ast);
        assert_eq!(parser.current, Token::EOF);
        Ok(())
    }

    #[test]
    fn test_in_in() -> Result<(), String> {
        let mut parser = Parser::new("a in a in a".to_string())?;
        let ast = parser.parse()?;
        println!("{:#?}", ast);
        assert_eq!(parser.current, Token::EOF);
        Ok(())
    }

    #[test]
    fn test_question() {
        let mut parser = Parser::new("a = b ? c ? d : e : f".to_string()).unwrap();
        let ast = parser.parse();
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_prefix_operator() {
        let mut parser = Parser::new("!a + b".to_string()).unwrap();
        let ast = parser.parse();
        assert!(ast.is_ok());
        println!("{:#?}", ast);
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_operator() {
        let mut parser = Parser::new("c = a + +b + d++".to_string()).unwrap();
        let ast = parser.parse();
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_call() {
        let mut parser = Parser::new("c = a ? b(d,e,f) : 2+3".to_string()).unwrap();
        let ast = parser.parse();
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_call_2() {
        let mut parser = Parser::new("a(b.c())".to_string()).unwrap();
        let ast = parser.parse();
        println!("{:#?}", ast);
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_new() {
        let mut parser = Parser::new("new A".to_string()).unwrap();
        let ast = parser.parse();
        println!("{:#?}", ast);
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_void() {
        let mut parser = Parser::new("void 0".to_string()).unwrap();
        let ast = parser.parse();
        println!("{:#?}", ast);
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_new_param() {
        let mut parser = Parser::new("new A(1,2,3)".to_string()).unwrap();
        let ast = parser.parse();
        println!("{:#?}", ast);
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_new_and_call() {
        let mut parser = Parser::new("new A(1,2,3)()".to_string()).unwrap();
        let ast = parser.parse();
        println!("{:#?}", ast);
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_arrow_function() {
        let mut parser = Parser::new("let a = ()=> {}".to_string()).unwrap();
        let ast = parser.parse();
        println!("{:#?}", ast);
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn test_arrow_function_with_params() {
        let mut parser = Parser::new("let a = (a= 1, b)=> {}".to_string()).unwrap();
        let ast = parser.parse();
        println!("{:#?}", ast);
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn member_call_mix() {
        let mut parser =
            Parser::new("recast.print(node).code.substring(0, limit).replace('', '')".to_string())
                .unwrap();
        let ast = parser.parse();
        println!("{:#?}", ast);
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }

    #[test]
    fn index_member_call() {
        let mut parser = Parser::new("a[b].c(d)".to_string()).unwrap();
        let ast = parser.parse();
        println!("{:#?}", ast);
        assert!(ast.is_ok());
        assert_eq!(parser.current, Token::EOF);
    }
}
