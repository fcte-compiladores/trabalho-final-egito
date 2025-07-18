// Aqui são os Statements/Comandos, que alteram o fluxo de execução do código e afins
#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl { name: String, value: Expr },
    Return(Expr),
    ExprStmt(Expr),
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>
    },
    While {
        condition: Expr,
        body: Vec<Stmt>
    },
    For {
        init: Option<Box<Stmt>>,
        condition: Option<Expr>,
        update: Option<Expr>,
        body: Vec<Stmt>
    },
}

// Aqui são as expressões, que podem ser avaliadas pra produzir um valor
#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Bool(bool),
    Var(String),
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Logical {
        op: LogicalOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

// Representa as funções, com nomes, parâMetros e um corpo de código a ser executado
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

// Operações aritméticas e de comparação
#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

// Operações lógicas (AND, OR)
#[derive(Debug, Clone, Copy)]
pub enum LogicalOp {
    And,
    Or,
}

// Operações unárias (NOT, negação)
#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Not,
    Minus,
}
