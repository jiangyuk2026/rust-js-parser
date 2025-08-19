use crate::exp::declaration_exp::build_let;
use crate::exp::for_exp::build_for;
use crate::exp::function_exp::build_function;
use crate::exp::if_exp::build_if;
use crate::exp::switch_exp::build_switch;
use crate::exp::try_exp::build_try;
use crate::express::{expect, is_ctrl_word, parse_expression};
use crate::lex::{Lex, Token};
use crate::node::Node;
use crate::node::Node::{BlockStatement, BreakStatement, ReturnStatement};

#[derive(PartialEq, Debug)]
pub enum IsArrowFunction {
    Impossible,
    Maybe,
    Must,
}

pub struct Parser {
    pub current: Token,
    pub lookahead: Token,
    pub is_arrow_function: IsArrowFunction,
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
            is_arrow_function: IsArrowFunction::Maybe,
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
                    _ => ast.push(*parse_expression(parser, 0)?),
                },
                Token::Case | Token::Default => {
                    break;
                }
                Token::Var | Token::Let | Token::Const => {
                    ast.push(*build_let(parser)?);
                }
                Token::For => {
                    ast.push(*build_for(parser)?);
                }
                Token::Function => {
                    ast.push(*build_function(parser, true)?);
                }
                Token::If => {
                    ast.push(*build_if(parser)?);
                }
                Token::Try => {
                    ast.push(*build_try(parser)?);
                }
                Token::Switch => {
                    ast.push(*build_switch(parser)?);
                }
                Token::Return => {
                    parser.next();
                    ast.push(ReturnStatement {
                        argument: parse_expression(parser, 0)?,
                    })
                }
                Token::Break => {
                    parser.next();
                    ast.push(BreakStatement { label: None })
                }
                _ => {
                    ast.push(*parse_expression(parser, 0)?);
                }
            }
        }
        Ok(ast)
    }

    pub fn parser_statement(parser: &mut Parser)-> Result<Box<Node>, String> {
        todo!()
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

    #[test]
    fn test_return() {
        let mut parser = Parser::new("return 1+2;".to_string());
        let ast = parser.parse();
        if let Err(e) = ast {
            eprintln!("e: {:?}", e)
        }
    }
}
