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
    pub is_identity_keyword: bool,
    pub is_identity_finally: bool,
    pub total_word_count: usize,
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
            is_identity_keyword: false,
            is_identity_finally: false,
            total_word_count: 0,
            lex,
        };

        Ok(parser)
    }

    pub fn next(&mut self) -> Result<(), String> {
        self.lex.regex_allowed = self.regex_allowed;
        self.last_loc_line = self.loc.end.line;
        loop {
            (self.current, self.loc) = self.lex.next()?;
            self.total_word_count += 1;
            /*if self.last_loc_line> 9320 {
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

    pub fn parse_statement_list(&mut self) -> Result<Vec<Node>, String> {
        let mut ast = vec![];
        loop {
            match &self.current {
                Token::EOF => break,
                Token::Comment(_) => {
                    self.comment = Some(self.current.clone());
                    self.next()?;
                }
                Token::Control(s) => match s.as_str() {
                    ";" => {
                        self.regex_allowed = true;
                        self.next()?;
                    }
                    "}" => break,
                    "{" => {
                        ast.push(*self.parse_block()?);
                    }
                    _ => ast.push(*parse_expression(self, 0)?),
                },
                Token::Case | Token::Default => {
                    break;
                }
                Token::Var | Token::Let | Token::Const => {
                    ast.push(*build_let(self)?);
                }
                Token::For => {
                    ast.push(*build_for(self)?);
                }
                Token::Function => {
                    ast.push(*build_function(self, true)?);
                }
                Token::If => {
                    ast.push(*build_if(self)?);
                }
                Token::While => {
                    ast.push(*build_while(self)?);
                }
                Token::Do => {
                    ast.push(*build_do_while(self)?);
                }
                Token::Try => {
                    ast.push(*build_try(self)?);
                }
                Token::Switch => {
                    ast.push(*build_switch(self)?);
                }
                Token::Return => {
                    self.regex_allowed = true;
                    self.next()?;
                    if !self.is_same_line() || self.current == Token::EOF {
                        ast.push(ReturnStatement { argument: None })
                    } else if is_ctrl_word(&self.current, "}") || is_ctrl_word(&self.current, ";") {
                        ast.push(ReturnStatement { argument: None })
                    } else {
                        ast.push(ReturnStatement {
                            argument: Some(parse_expression(self, 0)?),
                        })
                    }
                }
                Token::Break => {
                    self.next()?;
                    ast.push(BreakStatement { label: None })
                }
                Token::Continue => {
                    self.next()?;
                    ast.push(ContinueStatement { label: None })
                }
                Token::Throw => {
                    self.regex_allowed = true;
                    self.next()?;
                    if !self.is_same_line() || self.current == Token::EOF {
                        return Err("expression expected".to_string());
                    }
                    if is_ctrl_word(&self.current, "}") || is_ctrl_word(&self.current, ";") {
                        return Err("Unexpected token".to_string());
                    }
                    ast.push(ThrowStatement {
                        argument: parse_expression(self, 0)?,
                    })
                }
                _ => {
                    ast.push(*parse_expression(self, 0)?);
                }
            }
        }
        Ok(ast)
    }

    pub fn parse_block(&mut self) -> Result<Box<Node>, String> {
        let consequent: Box<Node>;
        if !is_ctrl_word(&self.current, "{") {
            return Err("handle_block expect {".to_string());
        }
        self.regex_allowed = true;
        self.next()?;
        consequent = Box::new(BlockStatement {
            body: Parser::parse_statement_list(self)?,
        });
        expect(self, "}")?;
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
