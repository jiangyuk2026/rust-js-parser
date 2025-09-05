use crate::token::Token;
use std::any::Any;
use std::fmt::{Debug, Formatter, Pointer};

#[derive(Debug, PartialEq, Clone)]
pub struct Extra {
    pub parenthesized: bool,
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
    fn set_parenthesized(&mut self, value: bool) {}
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
impl Node for EmptyStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct Identity {
    pub name: String,
    pub extra: Option<Extra>,
}

impl Node for Identity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, value: bool) {
        self.extra = Some(Extra {
            parenthesized: value,
        });
    }
}

#[derive(Clone, Debug)]
pub struct NumericLiteral {
    pub value: String,
    pub extra: Option<Extra>,
}

impl Node for NumericLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, value: bool) {
        self.extra = Some(Extra {
            parenthesized: value,
        });
    }
}

#[derive(Clone, Debug)]
pub struct StringLiteral {
    pub value: String,
    pub extra: Option<Extra>,
}

impl Node for StringLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, value: bool) {
        self.extra = Some(Extra {
            parenthesized: value,
        });
    }
}

#[derive(Clone, Debug)]
pub struct BooleanLiteral {
    pub value: bool,
    pub extra: Option<Extra>,
}

impl Node for BooleanLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, value: bool) {
        self.extra = Some(Extra {
            parenthesized: value,
        });
    }
}

#[derive(Clone, Debug)]
pub struct NullLiteral {
    pub extra: Option<Extra>,
}

impl Node for NullLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, value: bool) {
        self.extra = Some(Extra {
            parenthesized: value,
        });
    }
}

#[derive(Clone, Debug)]
pub struct RegExpLiteral {
    pub pattern: String,
    pub flags: String,
    pub extra: Option<Extra>,
}

impl Node for RegExpLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, value: bool) {
        self.extra = Some(Extra {
            parenthesized: value,
        });
    }
}

#[derive(Clone, Debug)]
pub struct TemplateLiteral {
    pub expressions: Vec<Box<dyn Node>>,
    pub quasis: Vec<Box<dyn Node>>,
    pub extra: Option<Extra>,
}

impl Node for TemplateLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn set_parenthesized(&mut self, value: bool) {
        self.extra = Some(Extra {
            parenthesized: value,
        });
    }
}

#[derive(Clone, Debug)]
pub struct TemplateElement {
    pub value: String,
    pub extra: Option<Extra>,
}

impl Node for TemplateElement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ArrayExpression {
    pub elements: Vec<Box<dyn Node>>,
    pub extra: Option<Extra>,
}

impl Node for ArrayExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ObjectExpression {
    pub properties: Vec<Box<dyn Node>>,
    pub extra: Option<Extra>,
}

impl Node for ObjectExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ObjectProperty {
    pub key: Box<dyn Node>,
    pub value: Box<dyn Node>,
}

impl Node for ObjectProperty {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ObjectMethod {
    pub key: Box<dyn Node>,
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}

impl Node for ObjectMethod {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ObjectPattern {
    pub properties: Vec<Box<dyn Node>>,
}

impl Node for ObjectPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ArrayPattern {
    pub elements: Vec<Box<dyn Node>>,
}

impl Node for ArrayPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct SequenceExpression {
    pub expressions: Vec<Box<dyn Node>>,
    pub extra: Option<Extra>,
}

impl Node for SequenceExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    pub kind: Token,
    pub declarations: Vec<Box<dyn Node>>,
}

impl Node for VariableDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct VariableDeclarator {
    pub id: Box<dyn Node>,
    pub init: Option<Box<dyn Node>>,
}

impl Node for VariableDeclarator {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct AssignmentExpression {
    pub left: Box<dyn Node>,
    pub operator: String,
    pub right: Box<dyn Node>,
}

impl Node for AssignmentExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    pub left: Box<dyn Node>,
    pub operator: String,
    pub right: Box<dyn Node>,
    pub extra: Extra,
}

impl Node for BinaryExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct LogicalExpression {
    pub left: Box<dyn Node>,
    pub operator: String,
    pub right: Box<dyn Node>,
}

impl Node for LogicalExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct UnaryExpression {
    pub operator: String,
    pub prefix: bool,
    pub argument: Box<dyn Node>,
}

impl Node for UnaryExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct UpdateExpression {
    pub operator: String,
    pub prefix: bool,
    pub argument: Box<dyn Node>,
}

impl Node for UpdateExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct MemberExpression {
    pub object: Box<dyn Node>,
    pub property: Box<dyn Node>,
    pub computed: bool,
}

impl Node for MemberExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ConditionalExpression {
    pub test: Box<dyn Node>,
    pub consequent: Box<dyn Node>,
    pub alternate: Box<dyn Node>,
}

impl Node for ConditionalExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct CallExpression {
    pub callee: Box<dyn Node>,
    pub arguments: Vec<Box<dyn Node>>,
}

impl Node for CallExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct NewExpression {
    pub callee: Box<dyn Node>,
    pub arguments: Vec<Box<dyn Node>>,
}

impl Node for NewExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ForStatement {
    pub init: Box<dyn Node>,
    pub test: Box<dyn Node>,
    pub update: Box<dyn Node>,
    pub body: Box<dyn Node>,
}

impl Node for ForStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ForInStatement {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
    pub body: Box<dyn Node>,
}

impl Node for ForInStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct WhileStatement {
    pub test: Box<dyn Node>,
    pub body: Box<dyn Node>,
}

impl Node for WhileStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct DoWhileStatement {
    pub body: Box<dyn Node>,
    pub test: Box<dyn Node>,
}

impl Node for DoWhileStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct FunctionDeclaration {
    pub id: Box<dyn Node>,
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}

impl Node for FunctionDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct FunctionExpression {
    pub id: Option<Box<dyn Node>>,
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}

impl Node for FunctionExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ArrowFunctionExpression {
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}

impl Node for ArrowFunctionExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ThisExpression {}

impl Node for ThisExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct AssignmentPattern {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
}

impl Node for AssignmentPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct BlockStatement {
    pub body: Vec<Box<dyn Node>>,
}

impl Node for BlockStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct IfStatement {
    pub test: Box<dyn Node>,
    pub consequent: Box<dyn Node>,
    pub alternate: Option<Box<dyn Node>>,
}

impl Node for IfStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct TryStatement {
    pub block: Box<dyn Node>,
    pub handle: Option<Box<dyn Node>>,
    pub finalizer: Option<Box<dyn Node>>,
}

impl Node for TryStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct CatchClause {
    pub param: Option<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}

impl Node for CatchClause {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
    pub argument: Option<Box<dyn Node>>,
}

impl Node for ReturnStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct SwitchStatement {
    pub discriminant: Box<dyn Node>,
    pub cases: Vec<Box<dyn Node>>,
}

impl Node for SwitchStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct SwitchCase {
    pub test: Option<Box<dyn Node>>,
    pub consequent: Vec<Box<dyn Node>>,
}

impl Node for SwitchCase {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct LabeledStatement {
    pub label: Box<dyn Node>,
    pub body: Box<dyn Node>,
}

impl Node for LabeledStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct BreakStatement {
    pub label: Option<Box<dyn Node>>,
}

impl Node for BreakStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ContinueStatement {
    pub label: Option<Box<dyn Node>>,
}

impl Node for ContinueStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug)]
pub struct ThrowStatement {
    pub argument: Box<dyn Node>,
}

impl Node for ThrowStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
