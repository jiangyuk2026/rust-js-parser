use crate::exp::declaration_exp::build_let;
use crate::exp::for_exp::build_for;
use crate::exp::function_exp::build_function;
use crate::exp::if_exp::build_if;
use crate::exp::try_exp::build_try;
use crate::express::{expect, is_ctrl_word, parse_expression};
use crate::lex::{Lex, Token};
use crate::node::Node;
use crate::node::Node::BlockStatement;

pub struct Parser {
    pub current: Token,
    pub lookahead: Token,
    pub list: Vec<Token>,
    lex: Lex,
}

impl Parser {
    pub fn new(input: String) -> Parser {
        let mut lex = Lex::new(input.to_string());
        let current = lex.next();
        let parser = Parser {
            current: current.clone(),
            lookahead: lex.next(),
            list: vec![current],
            lex,
        };

        parser
    }

    pub fn next(&mut self) {
        self.current = self.lookahead.clone();
        self.list.push(self.current.clone());
        self.lookahead = self.lex.next();
    }

    pub fn parse_statement_list(parser: &mut Parser) -> Result<Vec<Node>, String> {
        let mut ast = vec![];
        loop {
            match &parser.current {
                Token::EOF => break,
                Token::Control(s) => match s.as_str() {
                    ";" => {
                        parser.next();
                    }
                    "}" => break,
                    _ => ast.push(*parse_expression(parser, 1)?),
                },
                Token::Var | Token::Let | Token::Const => {
                    ast.push(*build_let(parser)?);
                }
                Token::For => {
                    ast.push(*build_for(parser)?);
                }
                Token::Function => {
                    ast.push(*build_function(parser)?);
                }
                Token::If => {
                    ast.push(*build_if(parser)?);
                }
                Token::Try => {
                    ast.push(*build_try(parser)?);
                }
                _ => {
                    ast.push(*parse_expression(parser, 0)?);
                }
            }
        }
        Ok(ast)
    }

    pub fn parse_block(parser: &mut Parser) -> Result<Box<Node>, String> {
        let consequent: Box<Node>;
        if !is_ctrl_word(&parser.current, "{") {
            return Err("handle_block expect {".to_string());
        }
        parser.next();
        consequent = Box::new(BlockStatement {
            body: Parser::parse_statement_list(parser)?,
        });
        expect(&parser.current, "}")?;
        parser.next();
        Ok(consequent)
    }

    pub fn parse(&mut self) -> Result<Vec<Node>, String> {
        Parser::parse_statement_list(self)
    }
}

#[cfg(test)]
mod parser_test {
    use crate::lex::Token;
    use crate::parser::Parser;

    #[test]
    fn test1() {
        let mut parser = Parser::new(" \n let \n a \n = \n b\n ;".to_string());

        assert_eq!(Token::Let, parser.current);
        parser.next();
        assert_eq!(Token::Variable("a".to_string()), parser.current);
    }
}
