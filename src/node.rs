use crate::token::Token;
use std::any::Any;
use std::fmt::{Debug, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct Extra {
    pub parenthesized: bool,
}

pub trait Node {
    fn as_any(&self) -> &dyn Any;
    fn get_node_type(&self) -> &'static str;
    fn set_parenthesized(&mut self, value: bool) {}
}

impl Clone for Box<dyn Node> {
    fn clone(&self) -> Self {
        self.clone()
    }
}

impl Debug for Box<dyn Node> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

#[derive(Clone, Debug)]
pub struct EmptyStatement {}

impl Node for EmptyStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "Identity"
    }
}

#[derive(Clone, Debug)]
pub struct Identity {
    pub name: String,
    pub extra: Option<Extra>,
}
impl Identity {
    pub fn new(name: String) -> Box<Identity> {
        Box::new(Identity { name, extra: None })
    }
}
impl Node for Identity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "Identity"
    }
    fn set_parenthesized(&mut self, value: bool) {
        // self.extra.parenthesized = value;
    }
}

#[derive(Clone, Debug)]
pub struct NumericLiteral {
    pub value: String,
    pub extra: Option<Extra>,
}
impl NumericLiteral {
    pub fn new(name: String) -> Box<NumericLiteral> {
        Box::new(NumericLiteral {
            value: name,
            extra: None,
        })
    }
}
impl Node for NumericLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "NumericLiteral"
    }
}

#[derive(Clone, Debug)]
pub struct StringLiteral {
    pub value: String,
    pub extra: Option<Extra>,
}
impl StringLiteral {
    pub fn new(name: String) -> Box<StringLiteral> {
        Box::new(StringLiteral {
            value: name,
            extra: None,
        })
    }
}
impl Node for StringLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "StringLiteral"
    }
}

#[derive(Clone, Debug)]
pub struct BooleanLiteral {
    pub value: bool,
    pub extra: Option<Extra>,
}
impl BooleanLiteral {
    pub fn new(value: bool) -> Box<BooleanLiteral> {
        Box::new(BooleanLiteral { value, extra: None })
    }
}
impl Node for BooleanLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "BooleanLiteral"
    }
}

#[derive(Clone, Debug)]
pub struct NullLiteral {
    pub extra: Option<Extra>,
}
impl NullLiteral {
    pub fn new() -> Box<NullLiteral> {
        Box::new(NullLiteral { extra: None })
    }
}
impl Node for NullLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "NullLiteral"
    }
}

#[derive(Clone, Debug)]
pub struct RegExpLiteral {
    pub pattern: String,
    pub flags: String,
    pub extra: Option<Extra>,
}
impl RegExpLiteral {
    pub fn new(pattern: String, flags: String) -> Box<RegExpLiteral> {
        Box::new(RegExpLiteral {
            pattern,
            flags,
            extra: None,
        })
    }
}
impl Node for RegExpLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "RegExpLiteral"
    }
}

#[derive(Clone, Debug)]
pub struct TemplateLiteral {
    pub expressions: Vec<Box<dyn Node>>,
    pub quasis: Vec<Box<dyn Node>>,
    pub extra: Option<Extra>,
}
impl TemplateLiteral {
    pub fn new(
        expressions: Vec<Box<dyn Node>>,
        quasis: Vec<Box<dyn Node>>,
    ) -> Box<TemplateLiteral> {
        Box::new(TemplateLiteral {
            expressions,
            quasis,
            extra: None,
        })
    }
}
impl Node for TemplateLiteral {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "TemplateLiteral"
    }
}

#[derive(Clone, Debug)]
pub struct TemplateElement {
    pub value: String,
    pub extra: Option<Extra>,
}
impl TemplateElement {
    pub fn new(value: String) -> Box<TemplateElement> {
        Box::new(TemplateElement { value, extra: None })
    }
}
impl Node for TemplateElement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "TemplateElement"
    }
}

#[derive(Clone, Debug)]
pub struct ArrayExpression {
    pub elements: Vec<Box<dyn Node>>,
    pub extra: Option<Extra>,
}
impl ArrayExpression {
    pub fn new(elements: Vec<Box<dyn Node>>) -> Box<ArrayExpression> {
        Box::new(ArrayExpression {
            elements,
            extra: None,
        })
    }
}
impl Node for ArrayExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "ArrayExpression"
    }
}

#[derive(Clone, Debug)]
pub struct ObjectExpression {
    pub properties: Vec<Box<dyn Node>>,
    pub extra: Option<Extra>,
}
impl ObjectExpression {
    pub fn new(properties: Vec<Box<dyn Node>>) -> Box<ObjectExpression> {
        Box::new(ObjectExpression {
            properties,
            extra: None,
        })
    }
}
impl Node for ObjectExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "ObjectExpression"
    }
}

#[derive(Clone, Debug)]
pub struct ObjectProperty {
    pub key: Box<dyn Node>,
    pub value: Box<dyn Node>,
}
impl ObjectProperty {
    pub fn new(key: Box<dyn Node>, value: Box<dyn Node>) -> Box<ObjectProperty> {
        Box::new(ObjectProperty { key, value })
    }
}
impl Node for ObjectProperty {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "ObjectProperty"
    }
}

#[derive(Clone, Debug)]
pub struct ObjectMethod {
    pub key: Box<dyn Node>,
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}
impl ObjectMethod {
    pub fn new(
        key: Box<dyn Node>,
        params: Vec<Box<dyn Node>>,
        body: Box<dyn Node>,
    ) -> Box<ObjectMethod> {
        Box::new(ObjectMethod { key, params, body })
    }
}
impl Node for ObjectMethod {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "ObjectMethod"
    }
}

#[derive(Clone, Debug)]
pub struct ObjectPattern {
    pub properties: Vec<Box<dyn Node>>,
}
impl ObjectPattern {}
impl Node for ObjectPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "ObjectPattern"
    }
}

#[derive(Clone, Debug)]
pub struct ArrayPattern {
    pub elements: Vec<Box<dyn Node>>,
}
impl ArrayPattern {}
impl Node for ArrayPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "ArrayPattern"
    }
}

#[derive(Clone, Debug)]
pub struct SequenceExpression {
    pub expressions: Vec<Box<dyn Node>>,
    pub extra: Option<Extra>,
}
impl SequenceExpression {}
impl Node for SequenceExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "SequenceExpression"
    }
}

#[derive(Clone, Debug)]
pub struct VariableDeclaration {
    pub kind: Token,
    pub declarations: Vec<Box<dyn Node>>,
}
impl VariableDeclaration {}
impl Node for VariableDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "VariableDeclaration"
    }
}

#[derive(Clone, Debug)]
pub struct VariableDeclarator {
    pub id: Box<dyn Node>,
    pub init: Option<Box<dyn Node>>,
}
impl VariableDeclarator {}
impl Node for VariableDeclarator {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "VariableDeclarator"
    }
}

#[derive(Clone, Debug)]
pub struct AssignmentExpression {
    pub left: Box<dyn Node>,
    pub operator: String,
    pub right: Box<dyn Node>,
}
impl AssignmentExpression {}
impl Node for AssignmentExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "AssignmentExpression"
    }
}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    pub left: Box<dyn Node>,
    pub operator: String,
    pub right: Box<dyn Node>,
    pub extra: Extra,
}
impl BinaryExpression {}
impl Node for BinaryExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "BinaryExpression"
    }
}

#[derive(Clone, Debug)]
pub struct LogicalExpression {
    pub left: Box<dyn Node>,
    pub operator: String,
    pub right: Box<dyn Node>,
}
impl LogicalExpression {}
impl Node for LogicalExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "LogicalExpression"
    }
}

#[derive(Clone, Debug)]
pub struct UnaryExpression {
    pub operator: String,
    pub prefix: bool,
    pub argument: Box<dyn Node>,
}
impl UnaryExpression {}
impl Node for UnaryExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "UnaryExpression"
    }
}

#[derive(Clone, Debug)]
pub struct UpdateExpression {
    pub operator: String,
    pub prefix: bool,
    pub argument: Box<dyn Node>,
}
impl UpdateExpression {}
impl Node for UpdateExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "UpdateExpression"
    }
}

#[derive(Clone, Debug)]
pub struct MemberExpression {
    pub object: Box<dyn Node>,
    pub property: Box<dyn Node>,
    pub computed: bool,
}
impl MemberExpression {
    pub fn new(
        computed: bool,
        object: Box<dyn Node>,
        property: Box<dyn Node>,
    ) -> Box<MemberExpression> {
        Box::new(MemberExpression {
            computed,
            object,
            property,
        })
    }
}
impl Node for MemberExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "MemberExpression"
    }
}

#[derive(Clone, Debug)]
pub struct ConditionalExpression {
    pub test: Box<dyn Node>,
    pub consequent: Box<dyn Node>,
    pub alternate: Box<dyn Node>,
}
impl ConditionalExpression {}
impl Node for ConditionalExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "ConditionalExpression"
    }
}

#[derive(Clone, Debug)]
pub struct CallExpression {
    pub callee: Box<dyn Node>,
    pub arguments: Vec<Box<dyn Node>>,
}
impl CallExpression {}
impl Node for CallExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "CallExpression"
    }
}

#[derive(Clone, Debug)]
pub struct NewExpression {
    pub callee: Box<dyn Node>,
    pub arguments: Vec<Box<dyn Node>>,
}
impl NewExpression {}
impl Node for NewExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "NewExpression"
    }
}

#[derive(Clone, Debug)]
pub struct ForStatement {
    pub init: Box<dyn Node>,
    pub test: Box<dyn Node>,
    pub update: Box<dyn Node>,
    pub body: Box<dyn Node>,
}
impl ForStatement {}
impl Node for ForStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "ForStatement"
    }
}

#[derive(Clone, Debug)]
pub struct ForInStatement {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
    pub body: Box<dyn Node>,
}
impl ForInStatement {}
impl Node for ForInStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "ForInStatement"
    }
}

#[derive(Clone, Debug)]
pub struct WhileStatement {
    pub test: Box<dyn Node>,
    pub body: Box<dyn Node>,
}
impl WhileStatement {}
impl Node for WhileStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "WhileStatement"
    }
}

#[derive(Clone, Debug)]
pub struct DoWhileStatement {
    pub body: Box<dyn Node>,
    pub test: Box<dyn Node>,
}
impl DoWhileStatement {}
impl Node for DoWhileStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "DoWhileStatement"
    }
}

#[derive(Clone, Debug)]
pub struct FunctionDeclaration {
    pub id: Box<dyn Node>,
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}
impl FunctionDeclaration {}
impl Node for FunctionDeclaration {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "FunctionDeclaration"
    }
}

#[derive(Clone, Debug)]
pub struct FunctionExpression {
    pub id: Option<Box<dyn Node>>,
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}
impl FunctionExpression {}
impl Node for FunctionExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "FunctionExpression"
    }
}

#[derive(Clone, Debug)]
pub struct ArrowFunctionExpression {
    pub params: Vec<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}
impl ArrowFunctionExpression {}
impl Node for ArrowFunctionExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "ArrowFunctionExpression"
    }
}

#[derive(Clone, Debug)]
pub struct ThisExpression {}
impl ThisExpression {}
impl Node for ThisExpression {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "ThisExpression"
    }
}

#[derive(Clone, Debug)]
pub struct AssignmentPattern {
    pub left: Box<dyn Node>,
    pub right: Box<dyn Node>,
}
impl AssignmentPattern {}
impl Node for AssignmentPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "AssignmentPattern"
    }
}

#[derive(Clone, Debug)]
pub struct BlockStatement {
    pub body: Vec<Box<dyn Node>>,
}
impl BlockStatement {}
impl Node for BlockStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "BlockStatement"
    }
}

#[derive(Clone, Debug)]
pub struct IfStatement {
    pub test: Box<dyn Node>,
    pub consequent: Box<dyn Node>,
    pub alternate: Option<Box<dyn Node>>,
}
impl IfStatement {}
impl Node for IfStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "IfStatement"
    }
}

#[derive(Clone, Debug)]
pub struct TryStatement {
    pub block: Box<dyn Node>,
    pub handle: Option<Box<dyn Node>>,
    pub finalizer: Option<Box<dyn Node>>,
}
impl TryStatement {}
impl Node for TryStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "TryStatement"
    }
}

#[derive(Clone, Debug)]
pub struct CatchClause {
    pub param: Option<Box<dyn Node>>,
    pub body: Box<dyn Node>,
}
impl CatchClause {}
impl Node for CatchClause {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "CatchClause"
    }
}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
    pub argument: Option<Box<dyn Node>>,
}
impl ReturnStatement {}
impl Node for ReturnStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "ReturnStatement"
    }
}

#[derive(Clone, Debug)]
pub struct SwitchStatement {
    pub discriminant: Box<dyn Node>,
    pub cases: Vec<Box<dyn Node>>,
}
impl SwitchStatement {}
impl Node for SwitchStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "SwitchStatement"
    }
}

#[derive(Clone, Debug)]
pub struct SwitchCase {
    pub test: Option<Box<dyn Node>>,
    pub consequent: Vec<Box<dyn Node>>,
}
impl SwitchCase {}
impl Node for SwitchCase {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "SwitchCase"
    }
}

#[derive(Clone, Debug)]
pub struct LabeledStatement {
    pub label: Box<dyn Node>,
    pub body: Box<dyn Node>,
}
impl LabeledStatement {}
impl Node for LabeledStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "LabeledStatement"
    }
}

#[derive(Clone, Debug)]
pub struct BreakStatement {
    pub label: Option<Box<dyn Node>>,
}
impl BreakStatement {}
impl Node for BreakStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "BreakStatement"
    }
}

#[derive(Clone, Debug)]
pub struct ContinueStatement {
    pub label: Option<Box<dyn Node>>,
}
impl ContinueStatement {}
impl Node for ContinueStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "ContinueStatement"
    }
}

#[derive(Clone, Debug)]
pub struct ThrowStatement {
    pub argument: Box<dyn Node>,
}
impl ThrowStatement {}
impl Node for ThrowStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn get_node_type(&self) -> &'static str {
        "ThrowStatement"
    }
}
