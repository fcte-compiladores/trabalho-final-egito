#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl { name: String, value: Expr },
    Return(Expr),
    ExprStmt(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Var(String),
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

