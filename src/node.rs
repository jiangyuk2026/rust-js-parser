use crate::lex::{Loc, Position};
use crate::token::Token;
use std::any::Any;
use std::fmt::{Debug, Formatter, Pointer};

#[derive(Debug, PartialEq, Clone)]
pub struct Extra {
    pub parenthesized: bool,
    pub paren_start: usize,
}

pub struct PrintContext {
    line: usize,
    column: usize,
    index: usize,
    pub output: String,
}

impl PrintContext {
    pub fn new() -> Self {
        PrintContext {
            line: 1,
            column: 1,
            index: 0,
            output: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        self.output.push_str(text);
        self.index += text.len();
    }
    pub fn add_start_loc_text(&mut self, pos: &Position, text: &str) {
        self.padding(&Position {
            line: pos.line,
            column: pos.column,
            index: pos.index - 1,
        });
        self.add_text(text);
    }
    pub fn add_end_loc_text(&mut self, pos: &Position, text: &str) {
        self.padding(&Position {
            line: pos.line,
            column: pos.column,
            index: pos.index - text.len(),
        });
        self.add_text(text);
    }
    pub fn padding(&mut self, pos: &Position) {
        let mut result = "".to_string();
        if self.line > pos.line {
            panic!("loc line error, {:#?}", pos)
        }
        while self.line < pos.line {
            self.line += 1;
            result += "\n";
        }
        if self.index > pos.index {
            panic!("loc index error, {:#?}", pos)
        }
        while pos.index > 0 && self.index < pos.index - 1 {
            println!("{}, {}", self.index, pos.index);
            self.index += 1;
            result += " ";
        }
        self.output.push_str(&result);
    }
}

pub trait NodeClone {
    fn clone_box(&self) -> Box<dyn Node>;
    fn fmt_node(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl<T> NodeClone for T
where
    T: 'static + Any + Clone + Node,
{
    fn clone_box(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn fmt_node(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

pub trait Node: NodeClone + Debug {
    fn as_any(&self) -> &dyn Any;
    fn set_parenthesized(&mut self, _extra: Option<Extra>) {}
    fn get_extra(&self) -> &Option<Extra>;
    fn get_loc(&self) -> &Loc;
    fn check_parenthesized(&self, extra: &Option<Extra>) -> bool {
        if let Some(extra) = &extra {
            if extra.parenthesized { true } else { false }
        } else {
            false
        }
    }
    fn print_node(&self, print_context: &mut PrintContext) {
        let extra = self.get_extra();
        let loc = self.get_loc();
        if self.check_parenthesized(extra) {
            print_context.padding(&Position {
                line: loc.start.line,
                column: loc.start.column,
                index: loc.start.index - 1,
            });
            print_context.add_text("(");
        } else {
            print_context.padding(&loc.start);
        }
        self.print_node_inner(print_context);
        if self.check_parenthesized(extra) {
            print_context.add_text(")");
        }
    }
    fn print_node_inner(&self, print_context: &mut PrintContext);
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone, Debug)]
pub struct EmptyStatement {
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl EmptyStatement {
    pub fn new(start_loc: Loc, end_loc: Loc) -> EmptyStatement {
        EmptyStatement {
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for EmptyStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }

    fn get_loc(&self) -> &Loc {
        &self.loc
    }

    fn print_node_inner(&self, print_context: &mut PrintContext) {}
}

#[derive(Clone, Debug)]
pub struct Identity {
    pub name: String,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl Identity {
    pub fn new(name: String, start_loc: Loc, end_loc: Loc) -> Self {
        Identity {
            name,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for Identity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        print_context.add_text(self.name.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct NumericLiteral {
    pub value: String,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl NumericLiteral {
    pub fn new(value: String, start_loc: Loc, end_loc: Loc) -> Self {
        NumericLiteral {
            value,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for NumericLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        print_context.add_start_loc_text(&self.loc.start, self.value.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct StringLiteral {
    pub value: String,
    pub is_single_quoted: bool,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl StringLiteral {
    pub fn new(value: String, is_single_quoted: bool, start_loc: Loc, end_loc: Loc) -> Self {
        StringLiteral {
            value,
            is_single_quoted,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for StringLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        if self.is_single_quoted {
            print_context.add_start_loc_text(&self.loc.start, "'");
            print_context.add_text(self.value.as_str());
            print_context.add_text("'");
        } else {
            print_context.add_start_loc_text(&self.loc.start, "\"");
            print_context.add_text(self.value.as_str());
            print_context.add_text("\"");
        }
    }
}

#[derive(Clone, Debug)]
pub struct BooleanLiteral {
    pub value: bool,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl BooleanLiteral {
    pub fn new(value: bool, start_loc: Loc, end_loc: Loc) -> Self {
        BooleanLiteral {
            value,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for BooleanLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        if self.value {
            print_context.add_start_loc_text(&self.loc.start, "true");
        } else {
            print_context.add_start_loc_text(&self.loc.start, "false");
        }
    }
}

#[derive(Clone, Debug)]
pub struct NullLiteral {
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl NullLiteral {
    pub fn new(start_loc: Loc, end_loc: Loc) -> Self {
        NullLiteral {
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for NullLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        print_context.add_start_loc_text(&self.loc.start, "null")
    }
}

#[derive(Clone, Debug)]
pub struct RegExpLiteral {
    pub pattern: String,
    pub flags: String,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl RegExpLiteral {
    pub fn new(pattern: String, flags: String, start_loc: Loc, end_loc: Loc) -> Self {
        RegExpLiteral {
            pattern,
            flags,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for RegExpLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        print_context.add_start_loc_text(&self.loc.start, "/");
        print_context.add_text(self.pattern.as_str());
        print_context.add_text("/");
        print_context.add_text(self.flags.as_str());
    }
}

#[derive(Clone, Debug)]
pub struct TemplateLiteral {
    pub expressions: Vec<Box<dyn Node>>,
    pub quasis: Vec<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl TemplateLiteral {
    pub fn new(
        expressions: Vec<Box<dyn Node>>,
        quasis: Vec<Box<dyn Node>>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        TemplateLiteral {
            expressions,
            quasis,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for TemplateLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        print_context.add_start_loc_text(&self.loc.start, "TemplateLiteral");
    }
}

#[derive(Clone, Debug)]
pub struct TemplateElement {
    pub value: String,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl TemplateElement {
    pub fn new(value: String, start_loc: Loc, end_loc: Loc) -> Self {
        TemplateElement {
            value,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for TemplateElement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        print_context.add_start_loc_text(&self.loc.start, "TemplateElement");
    }
}

#[derive(Clone, Debug)]
pub struct ArrayExpression {
    pub elements: Vec<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl ArrayExpression {
    pub fn new(elements: Vec<Box<dyn Node>>, start_loc: Loc, end_loc: Loc) -> Self {
        ArrayExpression {
            elements,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for ArrayExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        print_context.add_start_loc_text(&self.loc.start, "[");
        for node in &self.elements {
            node.print_node(print_context)
        }
        print_context.add_text("]");
    }
}

#[derive(Clone, Debug)]
pub struct ObjectExpression {
    pub properties: Vec<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl ObjectExpression {
    pub fn new(properties: Vec<Box<dyn Node>>, start_loc: Loc, end_loc: Loc) -> Self {
        ObjectExpression {
            properties,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for ObjectExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        print_context.add_start_loc_text(&self.loc.start, "{");
        for (i, node) in self.properties.iter().enumerate() {
            node.print_node(print_context);
            let is_last = i == self.properties.len() - 1;
            if !is_last {
                print_context.add_text(",");
            }
        }
        print_context.add_text("}");
    }
}

#[derive(Clone, Debug)]
pub struct ObjectProperty {
    pub key: Box<dyn Node>,
    pub value: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl ObjectProperty {
    pub fn new(key: Box<dyn Node>, value: Box<dyn Node>, start_loc: Loc, end_loc: Loc) -> Self {
        ObjectProperty {
            key,
            value,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for ObjectProperty {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        self.key.print_node(print_context);
        print_context.add_text(":");
        &self.value.print_node(print_context);
    }
}

#[derive(Clone, Debug)]
pub struct ObjectMethod {
    pub key: Box<dyn Node>,
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl ObjectMethod {
    pub fn new(
        key: Box<dyn Node>,
        params: Vec<Box<dyn Node>>,
        body: Box<dyn Node>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        ObjectMethod {
            key,
            params,
            body,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for ObjectMethod {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        self.key.print_node(print_context);
        print_context.add_text("(");
        for node in &self.params {
            node.print_node(print_context)
        }
        print_context.add_text(")");
        self.body.print_node(print_context)
    }
}

#[derive(Clone, Debug)]
pub struct ObjectPattern {
    pub properties: Vec<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl ObjectPattern {
    pub fn new(properties: Vec<Box<dyn Node>>, start_loc: Loc, end_loc: Loc) -> Self {
        ObjectPattern {
            properties,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for ObjectPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        print_context.add_start_loc_text(&self.loc.start, "{");
        for (i, node) in self.properties.iter().enumerate() {
            node.print_node(print_context);
            let is_last = i == self.properties.len() - 1;
            if !is_last {
                print_context.add_text(",");
            }
        }
        print_context.add_text("}");
    }
}

#[derive(Clone, Debug)]
pub struct ArrayPattern {
    pub elements: Vec<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl ArrayPattern {
    pub fn new(elements: Vec<Box<dyn Node>>, start_loc: Loc, end_loc: Loc) -> Self {
        ArrayPattern {
            elements,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for ArrayPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        print_context.add_start_loc_text(&self.loc.start, "[");
        for (i, node) in self.elements.iter().enumerate() {
            node.print_node(print_context);
            let is_last = i == self.elements.len() - 1;
            if !is_last {
                print_context.add_text(",");
            }
        }
        print_context.add_text("]");
    }
}

#[derive(Clone, Debug)]
pub struct SequenceExpression {
    pub expressions: Vec<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl SequenceExpression {
    pub fn new(expressions: Vec<Box<dyn Node>>, start_loc: Loc, end_loc: Loc) -> Self {
        SequenceExpression {
            expressions,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for SequenceExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        for (i, node) in self.expressions.iter().enumerate() {
            node.print_node(print_context);
            let is_last = i == self.expressions.len() - 1;
            if !is_last {
                print_context.add_text(",");
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    pub kind: Token,
    pub declarations: Vec<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl VariableDeclaration {
    pub fn new(
        kind: Token,
        declarations: Vec<Box<dyn Node>>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        VariableDeclaration {
            kind,
            declarations,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for VariableDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        print_context.add_text(&self.kind.to_string());
        for (i, node) in self.declarations.iter().enumerate() {
            node.print_node(print_context);
            let is_last = i == self.declarations.len() - 1;
            if !is_last {
                print_context.add_text(",");
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct VariableDeclarator {
    pub id: Box<dyn Node>,
    pub init: Option<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl VariableDeclarator {
    pub fn new(
        id: Box<dyn Node>,
        init: Option<Box<dyn Node>>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        VariableDeclarator {
            id,
            init,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for VariableDeclarator {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        if let Some(init) = &self.init {
            self.id.print_node(print_context);
            print_context.add_text("=");
            init.print_node(print_context);
        } else {
            self.id.print_node(print_context)
        }
    }
}

#[derive(Clone, Debug)]
pub struct AssignmentExpression {
    pub left: Box<dyn Node>,
    pub operator: String,
    pub right: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl AssignmentExpression {
    pub fn new(
        left: Box<dyn Node>,
        operator: String,
        right: Box<dyn Node>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        AssignmentExpression {
            left,
            operator,
            right,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for AssignmentExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        self.left.print_node(print_context);
        print_context.add_text(&self.operator);
        self.right.print_node(print_context);
    }
}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    pub left: Box<dyn Node>,
    pub operator: String,
    pub right: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl BinaryExpression {
    pub fn new(
        left: Box<dyn Node>,
        operator: String,
        right: Box<dyn Node>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        BinaryExpression {
            left,
            operator,
            right,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for BinaryExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        self.left.print_node(print_context);
        print_context.add_text(&self.operator);
        self.right.print_node(print_context);
    }
}

#[derive(Clone, Debug)]
pub struct LogicalExpression {
    pub left: Box<dyn Node>,
    pub operator: String,
    pub right: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl LogicalExpression {
    pub fn new(
        left: Box<dyn Node>,
        operator: String,
        right: Box<dyn Node>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        LogicalExpression {
            left,
            operator,
            right,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for LogicalExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        self.left.print_node(print_context);
        print_context.add_text(&self.operator);
        self.right.print_node(print_context)
    }
}

#[derive(Clone, Debug)]
pub struct UnaryExpression {
    pub operator: String,
    pub prefix: bool,
    pub argument: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl UnaryExpression {
    pub fn new(
        operator: String,
        prefix: bool,
        argument: Box<dyn Node>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        UnaryExpression {
            operator,
            prefix,
            argument,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for UnaryExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        if self.prefix {
            print_context.add_start_loc_text(&self.loc.start, &self.operator);
            self.argument.print_node(print_context);
        }
        print_context.add_text("UnaryExpression prefix=false");
    }
}

#[derive(Clone, Debug)]
pub struct UpdateExpression {
    pub operator: String,
    pub prefix: bool,
    pub argument: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl UpdateExpression {
    pub fn new(
        operator: String,
        prefix: bool,
        argument: Box<dyn Node>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        UpdateExpression {
            operator,
            prefix,
            argument,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for UpdateExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        if self.prefix {
            print_context.add_start_loc_text(&self.loc.start, &self.operator.clone());
            self.argument.print_node(print_context);
        }
        self.argument.print_node(print_context);
        print_context.add_end_loc_text(&self.loc.start, &self.operator.clone());
    }
}

#[derive(Clone, Debug)]
pub struct MemberExpression {
    pub object: Box<dyn Node>,
    pub property: Box<dyn Node>,
    pub computed: bool,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl MemberExpression {
    pub fn new(
        object: Box<dyn Node>,
        property: Box<dyn Node>,
        computed: bool,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        MemberExpression {
            object,
            property,
            computed,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for MemberExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        if self.computed {
            return self.object.print_node(print_context)
                + "["
                + &self.property.print_node(print_context)
                + "]";
        }
        self.object.print_node(print_context) + "." + &self.property.print_node(print_context)
    }
}

#[derive(Clone, Debug)]
pub struct ConditionalExpression {
    pub test: Box<dyn Node>,
    pub consequent: Box<dyn Node>,
    pub alternate: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl ConditionalExpression {
    pub fn new(
        test: Box<dyn Node>,
        consequent: Box<dyn Node>,
        alternate: Box<dyn Node>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        ConditionalExpression {
            test,
            consequent,
            alternate,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for ConditionalExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        self.test.print_node(print_context)
            + "?"
            + &self.consequent.print_node(print_context)
            + ":"
            + &self.alternate.print_node(print_context)
    }
}

#[derive(Clone, Debug)]
pub struct CallExpression {
    pub callee: Box<dyn Node>,
    pub arguments: Vec<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl CallExpression {
    pub fn new(
        callee: Box<dyn Node>,
        arguments: Vec<Box<dyn Node>>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        CallExpression {
            callee,
            arguments,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for CallExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        for (i, node) in self.arguments.iter().enumerate() {
            node.print_node(print_context);
            let is_last = i == self.arguments.len() - 1;
            if !is_last {
                print_context.add_text(",");
            }
        }
        self.callee.print_node(print_context) + "(" + ")"
    }
}

#[derive(Clone, Debug)]
pub struct NewExpression {
    pub callee: Box<dyn Node>,
    pub arguments: Vec<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl NewExpression {
    pub fn new(
        callee: Box<dyn Node>,
        arguments: Vec<Box<dyn Node>>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        NewExpression {
            callee,
            arguments,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for NewExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        for (i, node) in self.arguments.iter().enumerate() {
            node.print_node(print_context);
            let is_last = i == self.arguments.len() - 1;
            if !is_last {
                print_context.add_text(",");
            }
        }
        "new ".to_string() + &self.callee.print_node(print_context) + "(" + &a.join(",") + ")"
    }
}

#[derive(Clone, Debug)]
pub struct ForStatement {
    pub init: Option<Box<dyn Node>>,
    pub test: Option<Box<dyn Node>>,
    pub update: Option<Box<dyn Node>>,
    pub body: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl ForStatement {
    pub fn new(
        init: Option<Box<dyn Node>>,
        test: Option<Box<dyn Node>>,
        update: Option<Box<dyn Node>>,
        body: Box<dyn Node>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        ForStatement {
            init,
            test,
            update,
            body,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for ForStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        let init_str = if let Some(init) = &self.init {
            &init.print_node(print_context)
        } else {
            ""
        };
        let test_str = if let Some(test) = &self.test {
            &test.print_node(print_context)
        } else {
            ""
        };
        let update_str = if let Some(update) = &self.update {
            &update.print_node(print_context)
        } else {
            ""
        };
        "for".to_string()
            + "("
            + init_str
            + ";"
            + test_str
            + ";"
            + update_str
            + ")"
            + &self.body.print_node(print_context)
    }
}

#[derive(Clone, Debug)]
pub struct ForInStatement {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
    pub body: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl ForInStatement {
    pub fn new(
        left: Box<dyn Node>,
        right: Box<dyn Node>,
        body: Box<dyn Node>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        ForInStatement {
            left,
            right,
            body,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for ForInStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        "for".to_string()
            + "("
            + &self.left.print_node(print_context)
            + " in "
            + &self.right.print_node(print_context)
            + ")"
            + &self.body.print_node(print_context)
    }
}

#[derive(Clone, Debug)]
pub struct WhileStatement {
    pub test: Box<dyn Node>,
    pub body: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl WhileStatement {
    pub fn new(test: Box<dyn Node>, body: Box<dyn Node>, start_loc: Loc, end_loc: Loc) -> Self {
        WhileStatement {
            test,
            body,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for WhileStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        "while(".to_string()
            + &self.test.print_node(print_context)
            + ")"
            + &self.body.print_node(print_context)
    }
}

#[derive(Clone, Debug)]
pub struct DoWhileStatement {
    pub body: Box<dyn Node>,
    pub test: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl DoWhileStatement {
    pub fn new(body: Box<dyn Node>, test: Box<dyn Node>, start_loc: Loc, end_loc: Loc) -> Self {
        DoWhileStatement {
            body,
            test,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for DoWhileStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        "do{".to_string()
            + &self.body.print_node(print_context)
            + "}while("
            + &self.test.print_node(print_context)
            + ")"
    }
}

#[derive(Clone, Debug)]
pub struct FunctionDeclaration {
    pub id: Box<dyn Node>,
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl FunctionDeclaration {
    pub fn new(
        id: Box<dyn Node>,
        params: Vec<Box<dyn Node>>,
        body: Box<dyn Node>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        FunctionDeclaration {
            id,
            params,
            body,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for FunctionDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        for (i, node) in self.params.iter().enumerate() {
            node.print_node(print_context);
            let is_last = i == self.params.len() - 1;
            if !is_last {
                print_context.add_text(",");
            }
        }
        "function ".to_string()
            + &self.id.print_node(print_context)
            + "("
            + ")"
            + &self.body.print_node(print_context)
    }
}

#[derive(Clone, Debug)]
pub struct FunctionExpression {
    pub id: Option<Box<dyn Node>>,
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl FunctionExpression {
    pub fn new(
        id: Option<Box<dyn Node>>,
        params: Vec<Box<dyn Node>>,
        body: Box<dyn Node>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        FunctionExpression {
            id,
            params,
            body,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for FunctionExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        for (i, node) in self.params.iter().enumerate() {
            node.print_node(print_context);
            let is_last = i == self.params.len() - 1;
            if !is_last {
                print_context.add_text(",");
            }
        }
        "/*".to_string()
            + &self.loc.start.line.to_string()
            + "*/function("
            + ")"
            + &self.body.print_node(print_context)
    }
}

#[derive(Clone, Debug)]
pub struct ArrowFunctionExpression {
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl ArrowFunctionExpression {
    pub fn new(
        params: Vec<Box<dyn Node>>,
        body: Box<dyn Node>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        ArrowFunctionExpression {
            params,
            body,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for ArrowFunctionExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        for (i, node) in self.params.iter().enumerate() {
            node.print_node(print_context);
            let is_last = i == self.params.len() - 1;
            if !is_last {
                print_context.add_text(",");
            }
        }
        "(".to_string() + ")=>" + &self.body.print_node(print_context)
    }
}

#[derive(Clone, Debug)]
pub struct ThisExpression {
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
    pub extra: Option<Extra>,
}

impl ThisExpression {
    pub fn new(start_loc: Loc, end_loc: Loc) -> Self {
        ThisExpression {
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
            extra: None,
        }
    }
}

impl Node for ThisExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, extra: Option<Extra>) {
        self.extra = extra;
    }
    fn get_extra(&self) -> &Option<Extra> {
        &self.extra
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        "this".to_string()
    }
}

#[derive(Clone, Debug)]
pub struct AssignmentPattern {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl AssignmentPattern {
    pub fn new(left: Box<dyn Node>, right: Box<dyn Node>, start_loc: Loc, end_loc: Loc) -> Self {
        AssignmentPattern {
            left,
            right,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for AssignmentPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        self.left.print_node(print_context) + "=" + &self.right.print_node(print_context)
    }
}

#[derive(Clone, Debug)]
pub struct BlockStatement {
    pub body: Vec<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl BlockStatement {
    pub fn new(body: Vec<Box<dyn Node>>, start_loc: Loc, end_loc: Loc) -> Self {
        BlockStatement {
            body,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for BlockStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        for node in &self.body {
            node.print_node(print_context)
        }
        "{".to_string() + &a.join("\n") + &context.padding(&self.loc.end) + "}"
    }
}

#[derive(Clone, Debug)]
pub struct IfStatement {
    pub test: Box<dyn Node>,
    pub consequent: Box<dyn Node>,
    pub alternate: Option<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl IfStatement {
    pub fn new(
        test: Box<dyn Node>,
        consequent: Box<dyn Node>,
        alternate: Option<Box<dyn Node>>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        IfStatement {
            test,
            consequent,
            alternate,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for IfStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        if let Some(alternate) = &self.alternate {
            return "if(".to_string()
                + &self.test.print_node(print_context)
                + ")"
                + &self.consequent.print_node(print_context)
                + "\nelse "
                + &alternate.print_node(print_context);
        }
        "if(".to_string()
            + &self.test.print_node(print_context)
            + ")"
            + &self.consequent.print_node(print_context)
    }
}

#[derive(Clone, Debug)]
pub struct TryStatement {
    pub block: Box<dyn Node>,
    pub handle: Option<Box<dyn Node>>,
    pub finalizer: Option<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl TryStatement {
    pub fn new(
        block: Box<dyn Node>,
        handle: Option<Box<dyn Node>>,
        finalizer: Option<Box<dyn Node>>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        TryStatement {
            block,
            handle,
            finalizer,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for TryStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        let text = "try".to_string() + &self.block.print_node(print_context);
        let mut catch_text = "".to_string();
        let mut finally_text = "".to_string();
        if let Some(handle) = &self.handle {
            catch_text = handle.print_node(print_context)
        }
        if let Some(finalizer) = &self.finalizer {
            finally_text = "finally ".to_string() + &finalizer.print_node(print_context);
        }
        text + &catch_text + &finally_text
    }
}

#[derive(Clone, Debug)]
pub struct CatchClause {
    pub param: Option<Box<dyn Node>>,
    pub body: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl CatchClause {
    pub fn new(
        param: Option<Box<dyn Node>>,
        body: Box<dyn Node>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        CatchClause {
            param,
            body,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for CatchClause {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        if let Some(param) = &self.param {
            return "catch(".to_string()
                + &param.print_node(print_context)
                + ")"
                + &self.body.print_node(print_context);
        }
        "catch()".to_string() + &self.body.print_node(print_context)
    }
}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
    pub argument: Option<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl ReturnStatement {
    pub fn new(argument: Option<Box<dyn Node>>, start_loc: Loc, end_loc: Loc) -> Self {
        ReturnStatement {
            argument,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for ReturnStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        if let Some(argument) = &self.argument {
            return "return ".to_string() + &argument.print_node(print_context);
        }
        "return".to_string()
    }
}

#[derive(Clone, Debug)]
pub struct SwitchStatement {
    pub discriminant: Box<dyn Node>,
    pub cases: Vec<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl SwitchStatement {
    pub fn new(
        discriminant: Box<dyn Node>,
        cases: Vec<Box<dyn Node>>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        SwitchStatement {
            discriminant,
            cases,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for SwitchStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        for node in &self.cases {
            node.print_node(print_context)
        }
        "switch(".to_string()
            + &self.discriminant.print_node(print_context)
            + "){"
            + &a.join("\n")
            + "}"
    }
}

#[derive(Clone, Debug)]
pub struct SwitchCase {
    pub test: Option<Box<dyn Node>>,
    pub consequent: Vec<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl SwitchCase {
    pub fn new(
        test: Option<Box<dyn Node>>,
        consequent: Vec<Box<dyn Node>>,
        start_loc: Loc,
        end_loc: Loc,
    ) -> Self {
        SwitchCase {
            test,
            consequent,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for SwitchCase {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        for node in &self.consequent {
            node.print_node(print_context)
        }
        if let Some(test) = &self.test {
            return "case ".to_string() + &test.print_node(print_context) + ":\n" + &a.join("\n");
        }
        "default:\n".to_string() + &a.join("\n")
    }
}

#[derive(Clone, Debug)]
pub struct LabeledStatement {
    pub label: Box<dyn Node>,
    pub body: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl LabeledStatement {
    pub fn new(label: Box<dyn Node>, body: Box<dyn Node>, start_loc: Loc, end_loc: Loc) -> Self {
        LabeledStatement {
            label,
            body,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for LabeledStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        "unsupported LabeledStatement".to_string()
    }
}

#[derive(Clone, Debug)]
pub struct BreakStatement {
    pub label: Option<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl BreakStatement {
    pub fn new(label: Option<Box<dyn Node>>, start_loc: Loc, end_loc: Loc) -> Self {
        BreakStatement {
            label,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for BreakStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        if let Some(label) = &self.label {
            return "break ".to_string() + &label.print_node(print_context);
        }
        "break".to_string()
    }
}

#[derive(Clone, Debug)]
pub struct ContinueStatement {
    pub label: Option<Box<dyn Node>>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl ContinueStatement {
    pub fn new(label: Option<Box<dyn Node>>, start_loc: Loc, end_loc: Loc) -> Self {
        ContinueStatement {
            label,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for ContinueStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        if let Some(label) = &self.label {
            return "continue ".to_string() + &label.print_node(print_context);
        }
        "continue".to_string()
    }
}

#[derive(Clone, Debug)]
pub struct ThrowStatement {
    pub argument: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl ThrowStatement {
    pub fn new(argument: Box<dyn Node>, start_loc: Loc, end_loc: Loc) -> Self {
        ThrowStatement {
            argument,
            start: start_loc.start.index,
            end: end_loc.end.index,
            loc: Loc {
                start: start_loc.start,
                end: end_loc.end,
            },
        }
    }
}

impl Node for ThrowStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_extra(&self) -> &Option<Extra> {
        &None
    }
    fn get_loc(&self) -> &Loc {
        &self.loc
    }
    fn print_node_inner(&self, print_context: &mut PrintContext) {
        "throw ".to_string() + &self.argument.print_node(print_context)
    }
}
