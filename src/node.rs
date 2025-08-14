use crate::lex::Token;

#[derive(Debug, PartialEq)]
pub enum Node {
    EmptyStatement {},
    Identity {
        name: String,
    },
    NumericLiteral {
        value: String,
    },
    StringLiteral {
        value: String,
    },
    ObjectExpression {
        properties: Vec<Node>,
    },
    ObjectProperty {
        key: Box<Node>,
        value: Box<Node>,
    },
    ObjectPattern {
        properties: Vec<Node>,
    },
    SequenceExpression {
        expressions: Vec<Node>,
    },
    VariableDeclaration {
        kind: Token,
        declarations: Vec<Node>,
    },
    VariableDeclarator {
        id: Box<Node>,
        init: Box<Node>,
    },
    AssignmentExpression {
        left: Box<Node>,
        operator: String,
        right: Box<Node>,
    },
    BinaryExpression {
        left: Box<Node>,
        operator: String,
        right: Box<Node>,
    },
    UnaryExpression {
        operator: String,
        prefix: bool,
        argument: Box<Node>,
    },
    UpdateExpression {
        operator: String,
        prefix: bool,
        argument: Box<Node>,
    },
    MemberExpression {
        object: Box<Node>,
        property: Box<Node>,
        computed: bool,
    },
    ConditionalExpression {
        test: Box<Node>,
        consequent: Box<Node>,
        alternate: Box<Node>,
    },
    CallExpression {
        callee: Box<Node>,
        arguments: Vec<Node>,
    },
    ForStatement {
        init: Box<Node>,
        test: Box<Node>,
        update: Box<Node>,
        body: Box<Node>,
    },
    FunctionDeclaration {
        id: Box<Node>,
        params: Vec<Node>,
        body: Box<Node>,
    },
    AssignmentPattern {
        left: Box<Node>,
        right: Box<Node>,
    },
    BlockStatement {
        body: Vec<Node>,
    },
    IfStatement {
        test: Box<Node>,
        consequent: Box<Node>,
        alternate: Option<Box<Node>>,
    },
    TryStatement {
        block: Box<Node>,
        handle: Option<Box<Node>>,
        finalizer: Option<Box<Node>>,
    },
    CatchClause {
        param: Option<Box<Node>>,
        body: Box<Node>,
    },
    ReturnStatement {
        argument: Box<Node>,
    },
    ArrowFunctionExpression {
        params: Vec<Node>,
        body: Box<Node>,
    },
}
