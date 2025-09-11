use crate::lex::Loc;
use crate::token::Token;
use std::any::Any;
use std::fmt::{Debug, Formatter, Pointer};

#[derive(Debug, PartialEq, Clone)]
pub struct Extra {
    pub parenthesized: bool,
    pub paren_start: usize,
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
    fn is_parenthesized(&self) -> bool;
    fn check_parenthesized(&self, extra: &Option<Extra>) -> bool {
        if let Some(extra) = &extra {
            if extra.parenthesized { true } else { false }
        } else {
            false
        }
    }
    fn print_node(&self) -> String {
        let node_text = self.print_node_inner();
        if self.is_parenthesized() {
            return format!("({})", node_text);
        }
        node_text
    }
    fn print_node_inner(&self) -> String;
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
//
// impl Debug for Box<dyn Node> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         Debug::fmt(&**self, f)
//     }
// }

#[derive(Clone, Debug)]
pub struct EmptyStatement {}
impl EmptyStatement {
    pub fn new() -> EmptyStatement {
        EmptyStatement {}
    }
}
impl Node for EmptyStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        "".to_string()
    }
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        self.name.clone()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        self.value.clone()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        if self.is_single_quoted {
            return "'".to_string() + &self.value.clone() + "'";
        }
        "\"".to_string() + &self.value.clone() + "\""
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        if self.value {
            "true".to_string()
        } else {
            "false".to_string()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        "null".to_string()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        "/".to_string() + &self.pattern.clone() + "/" + &self.flags
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        "TemplateLiteral".to_string()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        "TemplateElement".to_string()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        let a: Vec<String> = self.elements.iter().map(|node| node.print_node()).collect();
        "[".to_string() + &a.join(",") + "]"
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        let a: Vec<String> = self
            .properties
            .iter()
            .map(|node| node.print_node())
            .collect();
        "{".to_string() + &a.join(",") + "}"
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        self.key.print_node() + ":" + &self.value.print_node()
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        let a: Vec<String> = self.params.iter().map(|node| node.print_node()).collect();
        self.key.print_node() + "(" + &a.join("") + ")" + &self.body.print_node()
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        let a: Vec<String> = self
            .properties
            .iter()
            .map(|node| node.print_node())
            .collect();
        "{".to_string() + &a.join(",") + "}"
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        let a: Vec<String> = self.elements.iter().map(|node| node.print_node()).collect();
        "[".to_string() + &a.join(",") + "]"
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        let a: Vec<String> = self
            .expressions
            .iter()
            .map(|node| node.print_node())
            .collect();
        a.join(",")
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        let a: Vec<String> = self
            .declarations
            .iter()
            .map(|node| node.print_node())
            .collect();
        self.kind.to_string() + " " + &a.join(",")
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        if let Some(init) = &self.init {
            self.id.print_node() + " = " + &init.print_node()
        } else {
            self.id.print_node()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        let a = self.left.print_node();
        let b = self.right.print_node();
        format!("/*{}*/{}{}{}", self.loc.start.line, a, self.operator, b)
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        self.left.print_node() + " " + &self.operator + " " + &self.right.print_node()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        self.left.print_node() + " " + &self.operator + " " + &self.right.print_node()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        if self.prefix {
            return self.operator.clone() + " " + &self.argument.print_node();
        }
        "UnaryExpression prefix=false".to_string()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        if self.prefix {
            return self.operator.clone() + &self.argument.print_node();
        }
        self.argument.print_node() + &self.operator.to_string()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        if self.computed {
            return self.object.print_node() + "[" + &self.property.print_node() + "]";
        }
        self.object.print_node() + "." + &self.property.print_node()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        self.test.print_node()
            + "?"
            + &self.consequent.print_node()
            + ":"
            + &self.alternate.print_node()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        let a: Vec<String> = self
            .arguments
            .iter()
            .map(|node| node.print_node())
            .collect();
        self.callee.print_node() + "(" + &a.join(",") + ")"
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        let a: Vec<String> = self
            .arguments
            .iter()
            .map(|node| node.print_node())
            .collect();
        "new ".to_string() + &self.callee.print_node() + "(" + &a.join(",") + ")"
    }
}

#[derive(Clone, Debug)]
pub struct ForStatement {
    pub init: Box<dyn Node>,
    pub test: Box<dyn Node>,
    pub update: Box<dyn Node>,
    pub body: Box<dyn Node>,
    pub start: usize,
    pub end: usize,
    pub loc: Loc,
}

impl ForStatement {
    pub fn new(
        init: Box<dyn Node>,
        test: Box<dyn Node>,
        update: Box<dyn Node>,
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        "for".to_string()
            + "("
            + &self.init.print_node()
            + ";"
            + &self.test.print_node()
            + ";"
            + &self.update.print_node()
            + ")"
            + &self.body.print_node()
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        "for".to_string()
            + "("
            + &self.left.print_node()
            + " in "
            + &self.right.print_node()
            + ")"
            + &self.body.print_node()
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        "while(".to_string() + &self.test.print_node() + ")" + &self.body.print_node()
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        "do{".to_string() + &self.body.print_node() + "}while(" + &self.test.print_node() + ")"
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        let a: Vec<String> = self.params.iter().map(|node| node.print_node()).collect();
        "function ".to_string()
            + &self.id.print_node()
            + "("
            + &a.join(", ")
            + ")"
            + &self.body.print_node()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        let a: Vec<String> = self.params.iter().map(|node| node.print_node()).collect();
        "/*".to_string()
            + &self.loc.start.line.to_string()
            + "*/function("
            + &a.join(", ")
            + ")"
            + &self.body.print_node()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
        let a: Vec<String> = self.params.iter().map(|node| node.print_node()).collect();
        "(".to_string() + &a.join(", ") + ")=>" + &self.body.print_node()
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
    fn is_parenthesized(&self) -> bool {
        self.check_parenthesized(&self.extra)
    }
    fn print_node_inner(&self) -> String {
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        self.left.print_node() + "=" + &self.right.print_node()
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        let a: Vec<String> = self.body.iter().map(|node| node.print_node()).collect();

        "{".to_string() + &a.join("\n") + "}"
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        if let Some(alternate) = &self.alternate {
            return "if(".to_string()
                + &self.test.print_node()
                + ")"
                + &self.consequent.print_node()
                + "\nelse "
                + &alternate.print_node();
        }
        "if(".to_string() + &self.test.print_node() + ")" + &self.consequent.print_node()
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        let text = "try".to_string() + &self.block.print_node();
        let mut catch_text = "".to_string();
        let mut finally_text = "".to_string();
        if let Some(handle) = &self.handle {
            catch_text = handle.print_node()
        }
        if let Some(finalizer) = &self.finalizer {
            finally_text = "finally ".to_string() + &finalizer.print_node();
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        if let Some(param) = &self.param {
            return "catch(".to_string() + &param.print_node() + ")" + &self.body.print_node();
        }
        "catch()".to_string() + &self.body.print_node()
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        if let Some(argument) = &self.argument {
            return "return ".to_string() + &argument.print_node();
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        let a: Vec<String> = self.cases.iter().map(|node| node.print_node()).collect();
        "switch(".to_string() + &self.discriminant.print_node() + "){" + &a.join("\n") + "}"
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        let a: Vec<String> = self
            .consequent
            .iter()
            .map(|node| node.print_node())
            .collect();
        if let Some(test) = &self.test {
            return "case ".to_string() + &test.print_node() + ":\n" + &a.join("\n");
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        if let Some(label) = &self.label {
            return "break ".to_string() + &label.print_node();
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        if let Some(label) = &self.label {
            return "continue ".to_string() + &label.print_node();
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
    fn is_parenthesized(&self) -> bool {
        false
    }
    fn print_node_inner(&self) -> String {
        "throw ".to_string() + &self.argument.print_node()
    }
}
