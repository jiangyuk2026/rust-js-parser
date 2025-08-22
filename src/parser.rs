use crate::exp::declaration_exp::build_let;
use crate::exp::for_exp::build_for;
use crate::exp::function_exp::build_function;
use crate::exp::if_exp::build_if;
use crate::exp::switch_exp::build_switch;
use crate::exp::try_exp::build_try;
use crate::exp::while_exp::{build_do_while, build_while};
use crate::express::{expect, is_ctrl_word, parse_expression};
use crate::lex::{Lex, Loc};
use crate::node::Node;
use crate::node::Node::{
    BlockStatement, BreakStatement, ContinueStatement, EmptyStatement, ReturnStatement,
    ThrowStatement,
};
use crate::token::Token;

#[derive(PartialEq, Debug)]
pub enum IsArrowFunction {
    Impossible,
    Maybe,
    Must,
}

#[derive(PartialEq, Debug)]
pub enum IsForIn {
    Impossible,
    Maybe,
    Must,
}

pub struct Parser {
    pub current: Token,
    pub is_arrow_function: IsArrowFunction,
    pub is_for_in: IsForIn,
    pub in_for_init: bool,
    pub list: Vec<Token>,
    pub loc: Loc,
    pub last_loc_line: usize,
    comment: Option<Token>,
    pub regex_allowed: bool,
    pub is_identity_catch: bool,
    pub is_identity_finally: bool,
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
            comment: None,
            current: current.clone(),
            loc: loc.clone(),
            last_loc_line: 0,
            is_arrow_function: IsArrowFunction::Maybe,
            in_for_init: false,
            is_for_in: IsForIn::Maybe,
            list: vec![current],
            regex_allowed: true,
            is_identity_catch: false,
            is_identity_finally: false,
            lex,
        };

        Ok(parser)
    }

    pub fn next(&mut self) -> Result<(), String> {
        self.lex.regex_allowed = self.regex_allowed;
        self.last_loc_line = self.loc.end.line;
        loop {
            (self.current, self.loc) = self.lex.next()?;
            /*if self.last_loc_line> 3120 {
                println!("token: {}", self.current);
            }*/
            // self.list.insert(0, self.current.clone());
            if !matches!(self.current, Token::Comment(_)) {
                break;
            }
        }
        self.regex_allowed = false;
        Ok(())
    }

    pub fn is_same_line(&self) -> bool {
        self.last_loc_line == self.loc.start.line
    }

    pub fn parse_statement_list(parser: &mut Parser) -> Result<Vec<Node>, String> {
        let mut ast = vec![];
        loop {
            match &parser.current {
                Token::EOF => break,
                Token::Comment(_) => {
                    parser.comment = Some(parser.current.clone());
                    parser.next()?;
                }
                Token::Control(s) => match s.as_str() {
                    ";" => {
                        parser.regex_allowed = true;
                        parser.next()?;
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
                Token::While => {
                    ast.push(*build_while(parser)?);
                }
                Token::Do => {
                    ast.push(*build_do_while(parser)?);
                }
                Token::Try => {
                    ast.push(*build_try(parser)?);
                }
                Token::Switch => {
                    ast.push(*build_switch(parser)?);
                }
                Token::Return => {
                    parser.regex_allowed = true;
                    parser.next()?;
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
                    parser.next()?;
                    ast.push(BreakStatement { label: None })
                }
                Token::Continue => {
                    parser.next()?;
                    ast.push(ContinueStatement { label: None })
                }
                Token::Throw => {
                    parser.regex_allowed = true;
                    parser.next()?;
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

    pub fn parse_block(parser: &mut Parser) -> Result<Box<Node>, String> {
        let consequent: Box<Node>;
        if !is_ctrl_word(&parser.current, "{") {
            return Err("handle_block expect {".to_string());
        }
        parser.regex_allowed = true;
        parser.next()?;
        consequent = Box::new(BlockStatement {
            body: Parser::parse_statement_list(parser)?,
        });
        expect(parser, "}")?;
        Ok(consequent)
    }

    pub fn build_maybe_empty_body(&mut self) -> Result<Box<Node>, String> {
        let body: Box<Node>;
        if is_ctrl_word(&self.current, "{") {
            body = Parser::parse_block(self)?;
        } else if is_ctrl_word(&self.current, ";") {
            body = Box::new(EmptyStatement {});
            self.next()?;
        } else {
            return Err("for body error".to_string());
        }
        Ok(body)
    }

    pub fn parse(&mut self) -> Result<Vec<Node>, String> {
        Parser::parse_statement_list(self)
    }
}

#[cfg(test)]
mod parser_test {
    use crate::parser::Parser;
    use crate::token::Token;

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
