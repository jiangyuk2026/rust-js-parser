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
use crate::node::{
    BlockStatement, BreakStatement, ContinueStatement, EmptyStatement, ReturnStatement,
    ThrowStatement,
};
use crate::token::Token;
use std::rc::Rc;

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
    pub current: Rc<Token>,
    pub is_arrow_function: IsArrowFunction,
    pub is_for_in: IsForIn,
    pub in_for_init: bool,
    pub list: Vec<Rc<Token>>,
    pub loc: Loc,
    pub last_loc_line: usize,
    comment_list: Vec<Token>,
    pub regex_allowed: bool,
    pub is_identity_keyword: bool,
    pub is_identity_finally: bool,
    pub total_word_count: usize,
    pub parenthesized: bool,
    lex: Lex,
}

pub fn lex_next(lex: &mut Lex) -> Result<(Rc<Token>, Loc, Vec<Token>, usize), String> {
    let mut total = 0;
    let mut comment_list = vec![];
    let mut current;
    let mut loc;
    loop {
        println!("loop");
        (current, loc) = lex.next()?;
        total += 1;
        /*if self.last_loc_line> 9320 {
            println!("token: {}", self.current);
        }*/
        // self.list.insert(0, self.current.clone());
        if matches!(current, Token::Comment(_)) {
            comment_list.push(current);
        } else {
            break;
        }
    }
    println!("{:#?}", loc);
    Ok((Rc::new(current), loc, comment_list, total))
}

impl Parser {
    pub fn new(input: String) -> Result<Parser, String> {
        let mut lex = Lex::new(input.to_string());
        let mut current;
        let mut loc;
        let comment_list;
        let total;
        (current, loc, comment_list, total) = lex_next(&mut lex)?;

        let parser = Parser {
            current: Rc::clone(&current),
            comment_list,
            list: vec![Rc::clone(&current)],
            loc: loc.clone(),
            last_loc_line: 0,
            is_arrow_function: IsArrowFunction::Maybe,
            in_for_init: false,
            is_for_in: IsForIn::Maybe,
            regex_allowed: true,
            is_identity_keyword: false,
            is_identity_finally: false,
            total_word_count: 0,
            parenthesized: false,
            lex,
        };

        Ok(parser)
    }

    pub fn next(&mut self) -> Result<(), String> {
        self.lex.regex_allowed = self.regex_allowed;
        self.last_loc_line = self.loc.end.line;
        let (current, loc, comment_list, total) = lex_next(&mut self.lex)?;
        self.total_word_count += total;
        self.current = Rc::clone(&current);
        // self.list.insert(0, Rc::clone(&current));
        self.loc = loc;
        self.comment_list = comment_list;
        self.regex_allowed = false;
        Ok(())
    }

    pub fn is_same_line(&self) -> bool {
        self.last_loc_line == self.loc.start.line
    }

    pub fn parse_statement(&mut self) -> Result<Box<dyn Node>, String> {
        let node:Box<dyn Node>;
        match *self.current {
            Token::EOF => return Err("expect statement".to_string()),
            Token::Var | Token::Let | Token::Const => node = build_let(self)?,
            Token::For => node = build_for(self)?,
            Token::Function => node = build_function(self, true)?,
            Token::If => node = build_if(self)?,
            Token::While => node = build_while(self)?,
            Token::Do => node = build_do_while(self)?,
            Token::Try => node = build_try(self)?,
            Token::Switch => node = build_switch(self)?,
            Token::Return => {
                self.regex_allowed = true;
                self.next()?;
                if !self.is_same_line() || *self.current == Token::EOF {
                    node = Box::new(ReturnStatement { argument: None })
                } else if is_ctrl_word(&self.current, "}") || is_ctrl_word(&self.current, ";") {
                    node = Box::new(ReturnStatement { argument: None })
                } else {
                    node = Box::new(ReturnStatement {
                        argument: Some(parse_expression(self, 0)?),
                    });
                    if is_ctrl_word(&self.current, ";") {
                        self.next()?;
                    }
                }
            }
            Token::Break => {
                self.next()?;
                node = Box::new(BreakStatement { label: None });
                if is_ctrl_word(&self.current, ";") {
                    self.next()?;
                }
            }
            Token::Continue => {
                self.next()?;
                node = Box::new(ContinueStatement { label: None });
                if is_ctrl_word(&self.current, ";") {
                    self.next()?;
                }
            }
            Token::Throw => {
                self.regex_allowed = true;
                self.next()?;
                if !self.is_same_line() || *self.current == Token::EOF {
                    return Err("expression expected".to_string());
                }
                if is_ctrl_word(&self.current, "}") || is_ctrl_word(&self.current, ";") {
                    return Err("Unexpected token".to_string());
                }
                node = Box::new(ThrowStatement {
                    argument: parse_expression(self, 0)?,
                });
                if is_ctrl_word(&self.current, ";") {
                    self.next()?;
                }
            }
            _ => {
                node = parse_expression(self, 0)?;
                if is_ctrl_word(&self.current, ";") {
                    self.next()?;
                }
            }
        };
        Ok(node)
    }

    pub fn parse_statement_list(&mut self) -> Result<Vec<Box<dyn Node>>, String> {
        let mut ast:Vec<Box<dyn Node>> = vec![];
        loop {
            match &*self.current {
                Token::EOF => break,
                Token::Case | Token::Default => break,
                Token::Control(s) => match s.as_str() {
                    "{"=> {
                        ast.push(self.parse_block()?);
                        continue;
                    }
                    "}" => break,
                    ";" => {
                        ast.push(Box::new(EmptyStatement {}));
                        self.regex_allowed = true;
                        self.next()?;
                        continue;
                    }
                    _ => {}
                },
                _ => {}
            }
            let statement = self.parse_statement()?;
            ast.push(statement);
        }
        Ok(ast)
    }

    pub fn parse_block(&mut self) -> Result<Box<dyn Node>, String> {
        let consequent: Box<dyn Node>;
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

    pub fn build_maybe_empty_body(&mut self) -> Result<Box<dyn Node>, String> {
        let body: Box<dyn Node>;
        if is_ctrl_word(&self.current, "{") {
            body = Parser::parse_block(self)?;
        } else if is_ctrl_word(&self.current, ";") {
            body = Box::new(EmptyStatement {});
            self.regex_allowed = true;
            self.next()?;
        } else {
            body = self.parse_statement()?;
        }
        Ok(body)
    }

    pub fn parse(&mut self) -> Result<Vec<Box<dyn Node>>, String> {
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

        assert_eq!(Token::Let, *parser.current);
        parser.next().unwrap();
        assert_eq!(Token::Variable("a".to_string()), *parser.current);
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
