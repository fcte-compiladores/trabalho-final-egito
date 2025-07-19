use std::collections::HashMap;
use crate::ast::{Stmt, Expr, Function, BinOp, LogicalOp, UnaryOp};

// Os tipos de valores
#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    Bool(bool),
    Void,
}

//O tratamento desses valores
impl Value {
    fn as_number(&self) -> i64 {
        match self {
            Value::Number(n) => *n,
            _ => panic!("Tentativa de usar não-número como número"),
        }
    }

    fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0,
            Value::Void => false,
        }
    }
}

pub struct Interpreter {
    globals: HashMap<String, Function>,
    locals: HashMap<String, Value>,
}

#[derive(Debug)]
pub enum RuntimeError {
    UndefinedVariable(String),
    UndefinedFunction(String),
    WrongArgumentCount(String, usize, usize),
    DivisionByZero,
    Return(Value), // Usado para controle de fluxo do return
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            globals: HashMap::new(),
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, funcoes: Vec<Function>, main_body: Vec<Stmt>) -> Result<Value, RuntimeError> {
        // Registra todas as funções
        for func in funcoes {
            self.globals.insert(func.name.clone(), func);
        }

        // Executa o corpo da main
        self.execute_statements(&main_body)
    }

    fn execute_statements(&mut self, statements: &[Stmt]) -> Result<Value, RuntimeError> {
        let mut last_value = Value::Void;

        for stmt in statements {
            match self.execute_statement(stmt) {
                Ok(value) => last_value = value,
                Err(RuntimeError::Return(value)) => return Ok(value),
                Err(e) => return Err(e),
            }
        }

        Ok(last_value)
    }

    fn execute_statement(&mut self, stmt: &Stmt) -> Result<Value, RuntimeError> {
        //Tal como havia nos exemplos do Lox, temos o uso de statements
        match stmt {
            Stmt::VarDecl { name, value } => {
                let val = self.evaluate_expression(value)?;
                self.locals.insert(name.clone(), val.clone());
                Ok(val)
            }
            Stmt::Return(expr) => {
                let value = self.evaluate_expression(expr)?;
                Err(RuntimeError::Return(value))
            }
            Stmt::ExprStmt(expr) => {
                self.evaluate_expression(expr)
            }
            Stmt::If { condition, then_branch, else_branch } => {
                let condition_value = self.evaluate_expression(condition)?;
                if condition_value.is_truthy() {
                    self.execute_statements(then_branch)
                } else if let Some(else_stmts) = else_branch {
                    self.execute_statements(else_stmts)
                } else {
                    Ok(Value::Void)
                }
            }
            Stmt::While { condition, body } => {
                let mut last_value = Value::Void;
                loop {
                    let condition_value = self.evaluate_expression(condition)?;
                    if !condition_value.is_truthy() {
                        break;
                    }
                    last_value = self.execute_statements(body)?;
                }
                Ok(last_value)
            }
            Stmt::For { init, condition, update, body } => {
                let mut last_value = Value::Void;

                // Executa inicialização, se houver
                if let Some(init_stmt) = init {
                    self.execute_statement(init_stmt)?;
                }

                loop {
                    // Verifica condição, caso haja
                    if let Some(cond_expr) = condition {
                        let condition_value = self.evaluate_expression(cond_expr)?;
                        if !condition_value.is_truthy() {
                            break;
                        }
                    }

                    // Executa o corpo do código
                    last_value = self.execute_statements(body)?;

                    // Executa atualização, se tiver
                    if let Some(update_expr) = update {
                        self.evaluate_expression(update_expr)?;
                    }
                }

                Ok(last_value)
            }
        }
    }

    fn evaluate_expression(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),

            Expr::Bool(b) => Ok(Value::Bool(*b)),

            Expr::Var(name) => {
                if let Some(value) = self.locals.get(name) {
                    Ok(value.clone())
                } else {
                    Err(RuntimeError::UndefinedVariable(name.clone()))
                }
            }

            Expr::Binary { op, lhs, rhs } => {
                let left_val = self.evaluate_expression(lhs)?;
                let right_val = self.evaluate_expression(rhs)?;

                match op {
                    BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div => {
                        let left_num = left_val.as_number();
                        let right_num = right_val.as_number();

                        let result = match op {
                            BinOp::Add => left_num + right_num,
                            BinOp::Sub => left_num - right_num,
                            BinOp::Mul => left_num * right_num,
                            BinOp::Div => {
                                if right_num == 0 {
                                    return Err(RuntimeError::DivisionByZero);
                                }
                                left_num / right_num
                            }
                            _ => unreachable!(),
                        };

                        Ok(Value::Number(result))
                    }
                    BinOp::Equal => Ok(Value::Bool(self.values_equal(&left_val, &right_val))),
                    BinOp::NotEqual => Ok(Value::Bool(!self.values_equal(&left_val, &right_val))),
                    BinOp::Greater => Ok(Value::Bool(left_val.as_number() > right_val.as_number())),
                    BinOp::GreaterEqual => Ok(Value::Bool(left_val.as_number() >= right_val.as_number())),
                    BinOp::Less => Ok(Value::Bool(left_val.as_number() < right_val.as_number())),
                    BinOp::LessEqual => Ok(Value::Bool(left_val.as_number() <= right_val.as_number())),
                }
            }

            Expr::Logical { op, lhs, rhs } => {
                let left_val = self.evaluate_expression(lhs)?;

                match op {
                    LogicalOp::And => {
                        if !left_val.is_truthy() {
                            Ok(left_val)
                        } else {
                            self.evaluate_expression(rhs)
                        }
                    }
                    LogicalOp::Or => {
                        if left_val.is_truthy() {
                            Ok(left_val)
                        } else {
                            self.evaluate_expression(rhs)
                        }
                    }
                }
            }

            Expr::Unary { op, expr } => {
                let val = self.evaluate_expression(expr)?;

                match op {
                    UnaryOp::Not => Ok(Value::Bool(!val.is_truthy())),
                    UnaryOp::Minus => Ok(Value::Number(-val.as_number())),
                }
            }

            Expr::Call { name, args } => {
                self.call_function(name, args)
            }
        }
    }

    fn values_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Void, Value::Void) => true,
            _ => false,
        }
    }

    fn call_function(&mut self, name: &str, args: &[Expr]) -> Result<Value, RuntimeError> {
        // Procura a função
        let function = self.globals.get(name)
            .ok_or_else(|| RuntimeError::UndefinedFunction(name.to_string()))?
            .clone();

        // Verifica se o número de argumentos está correto
        if args.len() != function.params.len() {
            return Err(RuntimeError::WrongArgumentCount(
                name.to_string(),
                function.params.len(),
                args.len(),
            ));
        }

        // Salva o estado atual das variáveis locais
        let saved_locals = self.locals.clone();

        // Avalia os argumentos!
        for (param, arg) in function.params.iter().zip(args.iter()) {
            let arg_value = self.evaluate_expression(arg)?;
            self.locals.insert(param.clone(), arg_value);
        }

        // Executa o corpo da função
        let result = match self.execute_statements(&function.body) {
            Ok(value) => Ok(value),
            Err(RuntimeError::Return(value)) => Ok(value),
            Err(e) => Err(e),
        };

        // Restaura as variáveis locais
        self.locals = saved_locals;

        result
    }
}

//Basicamente, gestão de erros e comportamentos anôMalos
impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::UndefinedVariable(name) => {
                write!(f, "Erro: Variável '{}' não definida", name)
            }
            RuntimeError::UndefinedFunction(name) => {
                write!(f, "Erro: Função '{}' não definida", name)
            }
            RuntimeError::WrongArgumentCount(name, expected, actual) => {
                write!(f, "Erro: Função '{}' espera {} argumentos, mas recebeu {}",
                       name, expected, actual)
            }
            RuntimeError::DivisionByZero => {
                write!(f, "Erro: Divisão por zero")
            }
            RuntimeError::Return(_) => {
                write!(f, "Erro interno: Return não capturado")
            }
        }
    }
}

impl std::error::Error for RuntimeError {}
