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
    SequenceExpression {
        expressions: Vec<Box<Node>>,
    },
    VariableDeclaration {
        kind: Token,
        declarations: Vec<Box<Node>>,
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
        arguments: Vec<Box<Node>>,
    },
    ForStatement {
        init: Box<Node>,
        test: Box<Node>,
        update: Box<Node>,
        body: Vec<Node>,
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
}
