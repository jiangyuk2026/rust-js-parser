use crate::exp::declaration_exp::build_let;
use crate::exp::for_exp::build_for;
use crate::exp::function_exp::build_function;
use crate::exp::if_exp::build_if;
use crate::exp::switch_exp::build_switch;
use crate::exp::try_exp::build_try;
use crate::express::{expect, is_ctrl_word, parse_expression};
use crate::lex::{Lex, Loc, Token};
use crate::node::Node;
use crate::node::Node::{BlockStatement, BreakStatement, ReturnStatement, ThrowStatement};
use std::mem::swap;

#[derive(PartialEq, Debug)]
pub enum IsArrowFunction {
    Impossible,
    Maybe,
    Must,
}

pub struct Parser {
    pub current: Token,
    pub is_arrow_function: IsArrowFunction,
    pub list: Vec<Token>,
    pub loc: Loc,
    pub last_loc: Loc,
    lex: Lex,
}

impl Parser {
    pub fn new(input: String) -> Result<Parser, String> {
        let mut lex = Lex::new(input.to_string());
        let mut current;
        let mut loc;
        loop {
            (current, loc) = lex.next()?;
            if current != Token::LF || current == Token::EOF {
                break;
            }
        }

        let parser = Parser {
            current: current.clone(),
            loc: loc.clone(),
            last_loc: loc,
            is_arrow_function: IsArrowFunction::Maybe,
            list: vec![current],
            lex,
        };

        Ok(parser)
    }

    pub fn next(&mut self) -> Result<(), String> {
        (self.current, self.last_loc) = self.lex.next()?;
        self.list.push(self.current.clone());
        swap(&mut self.loc, &mut self.last_loc);
        Ok(())
    }

    pub fn is_same_line(&self) -> bool {
        self.last_loc.end.line == self.loc.start.line
    }

    pub fn parse_statement_list(parser: &mut Parser) -> Result<Vec<Node>, String> {
        let mut ast = vec![];
        loop {
            match &parser.current {
                Token::EOF => break,
                Token::LF => parser.next()?,
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
                    if !parser.is_same_line() || parser.current == Token::EOF {
                        ast.push(ReturnStatement { argument: None })
                    } else if is_ctrl_word(&parser.current, "}")
                        || is_ctrl_word(&parser.current, ";")
                    {
                        ast.push(ReturnStatement { argument: None })
                    } else {
                        ast.push(ReturnStatement {
                            argument: Some(parse_expression(parser, 0)?),
                        })
                    }
                }
                Token::Break => {
                    parser.next();
                    ast.push(BreakStatement { label: None })
                }
                Token::Throw => {
                    parser.next();
                    if !parser.is_same_line() || parser.current == Token::EOF {
                        return Err("expression expected".to_string());
                    }
                    if is_ctrl_word(&parser.current, "}") || is_ctrl_word(&parser.current, ";") {
                        return Err("Unexpected token".to_string());
                    }
                    ast.push(ThrowStatement {
                        argument: parse_expression(parser, 0)?,
                    })
                }
                _ => {
                    ast.push(*parse_expression(parser, 0)?);
                }
            }
        }
        Ok(ast)
    }

    pub fn parser_statement(parser: &mut Parser) -> Result<Box<Node>, String> {
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
        let mut parser = Parser::new(" \n let \n a \n = \n b\n ;".to_string()).unwrap();

        assert_eq!(Token::Let, parser.current);
        parser.next().unwrap();
        assert_eq!(Token::Variable("a".to_string()), parser.current);
    }

    #[test]
    fn test_return() {
        let mut parser = Parser::new("return 1+2;".to_string()).unwrap();
        let ast = parser.parse();
        if let Err(e) = ast {
            eprintln!("e: {:?}", e)
        }
    }
}
