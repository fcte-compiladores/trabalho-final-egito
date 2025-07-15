// Aqui são os Statements/Comandos, que alteram o fluxo de execução do código e afins, que nem
// tinha na ast do Lox
#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl { name: String, value: Expr },
    Return(Expr),
    ExprStmt(Expr),
}

// Aqui são as expressões, que podem ser avaliados pra produzir um valor, tal como já visto em Lox
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

// Representa as funções, com nomes, parâmetros e um corpo de código a ser executado. Detalhe
// interessante: no Lox tbm era representado por nomes, parâmetros e corpo.
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

// Aqui são as operações aritméticas básicas, tal qual existiam no Lox
#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

