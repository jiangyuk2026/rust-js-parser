use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct Extra {
    pub parenthesized: bool,
}

pub trait Node {
    fn set_parenthesized(&mut self, value: bool) {}
}

pub struct EmptyStatement {}

impl Node for EmptyStatement {}

pub struct Identity {
    pub name: String,
    // pub extra: Extra,
}
impl Node for Identity {
    fn set_parenthesized(&mut self, value: bool) {
        // self.extra.parenthesized = value;
    }
}

pub struct NumericLiteral {
    pub value: String,
}
impl Node for NumericLiteral {}

pub struct StringLiteral {
    pub value: String,
}
impl Node for StringLiteral {}

pub struct BooleanLiteral {
    pub value: bool,
}
impl Node for BooleanLiteral {}

pub struct NullLiteral {}
impl Node for NullLiteral {}

pub struct RegExpLiteral {
    pub pattern: String,
    pub flags: String,
}
impl Node for RegExpLiteral {}

pub struct TemplateLiteral {
    pub expressions: Vec<Box<dyn Node>>,
    pub quasis: Vec<Box<dyn Node>>,
}
impl Node for TemplateLiteral {}

pub struct TemplateElement {
    pub value: String,
}
impl Node for TemplateElement {}

pub struct ArrayExpression {
    pub elements: Vec<Box<dyn Node>>,
}
impl Node for ArrayExpression {}

pub struct ObjectExpression {
    pub properties: Vec<Box<dyn Node>>,
}
impl Node for ObjectExpression {}

pub struct ObjectProperty {
    pub key: Box<dyn Node>,
    pub value: Box<dyn Node>,
}
impl Node for ObjectProperty {}

pub struct ObjectMethod {
    pub key: Box<dyn Node>,
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}
impl Node for ObjectMethod {}

pub struct ObjectPattern {
    pub properties: Vec<Box<dyn Node>>,
}
impl Node for ObjectPattern {}

pub struct ArrayPattern {
    pub elements: Vec<Box<dyn Node>>,
}
impl Node for ArrayPattern {}

pub struct SequenceExpression {
    pub expressions: Vec<Box<dyn Node>>,
    pub extra: Extra,
}
impl Node for SequenceExpression {}

pub struct VariableDeclaration {
    pub kind: Token,
    pub declarations: Vec<Box<dyn Node>>,
}
impl Node for VariableDeclaration {}

pub struct VariableDeclarator {
    pub id: Box<dyn Node>,
    pub init: Option<Box<dyn Node>>,
}
impl Node for VariableDeclarator {}

pub struct AssignmentExpression {
    pub left: Box<dyn Node>,
    pub operator: String,
    pub right: Box<dyn Node>,
}
impl Node for AssignmentExpression {}

pub struct BinaryExpression {
    pub left: Box<dyn Node>,
    pub operator: String,
    pub right: Box<dyn Node>,
    pub extra: Extra,
}
impl Node for BinaryExpression {}

pub struct LogicalExpression {
    pub left: Box<dyn Node>,
    pub operator: String,
    pub right: Box<dyn Node>,
}
impl Node for LogicalExpression {}

pub struct UnaryExpression {
    pub operator: String,
    pub prefix: bool,
    pub argument: Box<dyn Node>,
}
impl Node for UnaryExpression {}

pub struct UpdateExpression {
    pub operator: String,
    pub prefix: bool,
    pub argument: Box<dyn Node>,
}
impl Node for UpdateExpression {}

pub struct MemberExpression {
    pub object: Box<dyn Node>,
    pub property: Box<dyn Node>,
    pub computed: bool,
}
impl Node for MemberExpression {}

pub struct ConditionalExpression {
    pub test: Box<dyn Node>,
    pub consequent: Box<dyn Node>,
    pub alternate: Box<dyn Node>,
}
impl Node for ConditionalExpression {}

pub struct CallExpression {
    pub callee: Box<dyn Node>,
    pub arguments: Vec<Box<dyn Node>>,
}
impl Node for CallExpression {}

pub struct NewExpression {
    pub callee: Box<dyn Node>,
    pub arguments: Vec<Box<dyn Node>>,
}
impl Node for NewExpression {}

pub struct ForStatement {
    pub init: Box<dyn Node>,
    pub test: Box<dyn Node>,
    pub update: Box<dyn Node>,
    pub body: Box<dyn Node>,
}
impl Node for ForStatement {}

pub struct ForInStatement {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
    pub body: Box<dyn Node>,
}
impl Node for ForInStatement {}

pub struct WhileStatement {
    pub test: Box<dyn Node>,
    pub body: Box<dyn Node>,
}
impl Node for WhileStatement {}

pub struct DoWhileStatement {
    pub body: Box<dyn Node>,
    pub test: Box<dyn Node>,
}
impl Node for DoWhileStatement {}

pub struct FunctionDeclaration {
    pub id: Box<dyn Node>,
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}
impl Node for FunctionDeclaration {}

pub struct FunctionExpression {
    pub id: Option<Box<dyn Node>>,
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}
impl Node for FunctionExpression {}

pub struct ArrowFunctionExpression {
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}
impl Node for ArrowFunctionExpression {}

pub struct ThisExpression {}
impl Node for ThisExpression {}

pub struct AssignmentPattern {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
}
impl Node for AssignmentPattern {}

pub struct BlockStatement {
    pub body: Vec<Box<dyn Node>>,
}
impl Node for BlockStatement {}

pub struct IfStatement {
    pub test: Box<dyn Node>,
    pub consequent: Box<dyn Node>,
    pub alternate: Option<Box<dyn Node>>,
}
impl Node for IfStatement {}

pub struct TryStatement {
    pub block: Box<dyn Node>,
    pub handle: Option<Box<dyn Node>>,
    pub finalizer: Option<Box<dyn Node>>,
}
impl Node for TryStatement {}

pub struct CatchClause {
    pub param: Option<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}
impl Node for CatchClause {}

pub struct ReturnStatement {
    pub argument: Option<Box<dyn Node>>,
}
impl Node for ReturnStatement {}

pub struct SwitchStatement {
    pub discriminant: Box<dyn Node>,
    pub cases: Vec<Box<dyn Node>>,
}
impl Node for SwitchStatement {}

pub struct SwitchCase {
    pub test: Option<Box<dyn Node>>,
    pub consequent: Vec<Box<dyn Node>>,
}
impl Node for SwitchCase {}

pub struct LabeledStatement {
    pub label: Box<dyn Node>,
    pub body: Box<dyn Node>,
}
impl Node for LabeledStatement {}

pub struct BreakStatement {
    pub label: Option<Box<dyn Node>>,
}
impl Node for BreakStatement {}

pub struct ContinueStatement {
    pub label: Option<Box<dyn Node>>,
}
impl Node for ContinueStatement {}

pub struct ThrowStatement {
    pub argument: Box<dyn Node>,
}
impl Node for ThrowStatement {}
