use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Extra {
    None,
    Parenthesized,
}

#[derive(Debug, PartialEq, Clone)]
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
    BooleanLiteral {
        value: bool,
    },
    NullLiteral {},
    RegExpLiteral {
        pattern: String,
        flags: String,
    },
    TemplateLiteral {
        expressions: Vec<Node>,
        quasis: Vec<Node>,
    },
    TemplateElement {
        value: String,
    },
    ArrayExpression {
        elements: Vec<Node>,
    },
    ObjectExpression {
        properties: Vec<Node>,
    },
    ObjectProperty {
        key: Box<Node>,
        value: Box<Node>,
    },
    ObjectMethod {
        key: Box<Node>,
        params: Vec<Node>,
        body: Box<Node>,
    },
    ObjectPattern {
        properties: Vec<Node>,
    },
    ArrayPattern {
        elements: Vec<Node>,
    },
    SequenceExpression {
        expressions: Vec<Node>,
        extra: Extra,
    },
    VariableDeclaration {
        kind: Token,
        declarations: Vec<Node>,
    },
    VariableDeclarator {
        id: Box<Node>,
        init: Option<Box<Node>>,
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
        extra: Extra,
    },
    LogicalExpression {
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
    NewExpression {
        callee: Box<Node>,
        arguments: Vec<Node>,
    },
    ForStatement {
        init: Box<Node>,
        test: Box<Node>,
        update: Box<Node>,
        body: Box<Node>,
    },
    ForInStatement {
        left: Box<Node>,
        right: Box<Node>,
        body: Box<Node>,
    },
    FunctionDeclaration {
        id: Box<Node>,
        params: Vec<Node>,
        body: Box<Node>,
    },
    FunctionExpression {
        id: Option<Box<Node>>,
        params: Vec<Node>,
        body: Box<Node>,
    },
    ArrowFunctionExpression {
        params: Vec<Node>,
        body: Box<Node>,
    },
    ThisExpression {},
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
        argument: Option<Box<Node>>,
    },
    SwitchStatement {
        discriminant: Box<Node>,
        cases: Vec<Node>,
    },
    SwitchCase {
        test: Option<Box<Node>>,
        consequent: Vec<Node>,
    },
    LabeledStatement {
        label: Box<Node>,
        body: Box<Node>,
    },
    BreakStatement {
        label: Option<Box<Node>>,
    },
    ContinueStatement {
        label: Option<Box<Node>>,
    },
    ThrowStatement {
        argument: Box<Node>,
    },
}
