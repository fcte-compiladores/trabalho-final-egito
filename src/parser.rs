use crate::lexer::Token;
use crate::ast::{Stmt, Expr, Function, BinOp, LogicalOp, UnaryOp};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> (Vec<Function>, Vec<Stmt>) {
        let mut funcoes = Vec::new();
        let mut main_body = Vec::new();
        let mut in_main = false;

        while !self.is_at_end() {
            // Verifica se é uma declaração de função
            if self.check(&Token::Int) && self.peek_ahead(1).map_or(false, |t| matches!(t, Token::Ident(_))) {
                let func = self.parse_function();
                if func.name == "main" {
                    in_main = true;
                    main_body = func.body;
                } else {
                    funcoes.push(func);
                }
            } else if in_main {
                // Parse statements dentro da main
                let stmt = self.parse_statement();
                main_body.push(stmt);
            } else {
                // Avança se não conseguir fazer parse
                self.advance();
            }
        }

        (funcoes, main_body)
    }

    fn parse_function(&mut self) -> Function {
        // Tipo de retorno (int ou bool)
        if !self.match_tokens(&[Token::Int, Token::Bool]) {
            panic!("Esperado tipo de retorno");
        }

        // Nome da função
        let name = if let Token::Ident(n) = self.advance() {
            n.clone()
        } else {
            panic!("Esperado nome da função");
        };

        // Parâmetros
        self.consume(&Token::AbrePar, "Esperado '('");
        let mut params = Vec::new();

        if !self.check(&Token::FechaPar) {
            loop {
                // Tipo do parâmetro (int ou bool)
                if !self.match_tokens(&[Token::Int, Token::Bool]) {
                    panic!("Esperado tipo do parâmetro");
                }

                // Nome do parâmetro
                if let Token::Ident(param_name) = self.advance() {
                    params.push(param_name.clone());
                } else {
                    panic!("Esperado nome do parâmetro");
                }

                if !self.match_token(&Token::Virgula) {
                    break;
                }
            }
        }

        self.consume(&Token::FechaPar, "Esperado ')'");

        // Corpo da função
        self.consume(&Token::AbreChave, "Esperado '{'");
        let mut body = Vec::new();

        while !self.check(&Token::FechaChave) && !self.is_at_end() {
            body.push(self.parse_statement());
        }

        self.consume(&Token::FechaChave, "Esperado '}'");

        Function { name, params, body }
    }

    fn parse_statement(&mut self) -> Stmt {
        // Declaração de variável: int/bool nome = expressão;
        if self.match_tokens(&[Token::Int, Token::Bool]) {
            let name = if let Token::Ident(n) = self.advance() {
                n.clone()
            } else {
                panic!("Esperado nome da variável");
            };

            self.consume(&Token::Assign, "Esperado '='");
            let value = self.parse_expression();
            self.consume(&Token::PontoEVirgula, "Esperado ';'");

            return Stmt::VarDecl { name, value };
        }

        // Return statement
        if self.match_token(&Token::Return) {
            let expr = self.parse_expression();
            self.consume(&Token::PontoEVirgula, "Esperado ';'");
            return Stmt::Return(expr);
        }

        // If statement
        if self.match_token(&Token::If) {
            self.consume(&Token::AbrePar, "Esperado '('");
            let condition = self.parse_expression();
            self.consume(&Token::FechaPar, "Esperado ')'");

            let then_branch = self.parse_block();

            let else_branch = if self.match_token(&Token::Else) {
                Some(self.parse_block())
            } else {
                None
            };

            return Stmt::If { condition, then_branch, else_branch };
        }

        // While loop
        if self.match_token(&Token::While) {
            self.consume(&Token::AbrePar, "Esperado '('");
            let condition = self.parse_expression();
            self.consume(&Token::FechaPar, "Esperado ')'");

            let body = self.parse_block();

            return Stmt::While { condition, body };
        }

        // For loop
        if self.match_token(&Token::For) {
            self.consume(&Token::AbrePar, "Esperado '('");

            // Inicialização (opcional)
            let init = if self.check(&Token::PontoEVirgula) {
                None
            } else {
                Some(Box::new(self.parse_statement()))
            };

            if init.is_none() {
                self.consume(&Token::PontoEVirgula, "Esperado ';'");
            }

            // Condição (opcional)
            let condition = if self.check(&Token::PontoEVirgula) {
                None
            } else {
                Some(self.parse_expression())
            };
            self.consume(&Token::PontoEVirgula, "Esperado ';'");

            // Atualização (opcional)
            let update = if self.check(&Token::FechaPar) {
                None
            } else {
                Some(self.parse_expression())
            };
            self.consume(&Token::FechaPar, "Esperado ')'");

            let body = self.parse_block();

            return Stmt::For { init, condition, update, body };
        }

        // Expression statement
        let expr = self.parse_expression();
        self.consume(&Token::PontoEVirgula, "Esperado ';'");
        Stmt::ExprStmt(expr)
    }

    fn parse_block(&mut self) -> Vec<Stmt> {
        self.consume(&Token::AbreChave, "Esperado '{'");
        let mut statements = Vec::new();

        while !self.check(&Token::FechaChave) && !self.is_at_end() {
            statements.push(self.parse_statement());
        }

        self.consume(&Token::FechaChave, "Esperado '}'");
        statements
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Expr {
        let mut expr = self.parse_logical_and();

        while self.match_token(&Token::Or) {
            let right = self.parse_logical_and();
            expr = Expr::Logical {
                op: LogicalOp::Or,
                lhs: Box::new(expr),
                rhs: Box::new(right),
            };
        }

        expr
    }

    fn parse_logical_and(&mut self) -> Expr {
        let mut expr = self.parse_equality();

        while self.match_token(&Token::And) {
            let right = self.parse_equality();
            expr = Expr::Logical {
                op: LogicalOp::And,
                lhs: Box::new(expr),
                rhs: Box::new(right),
            };
        }

        expr
    }

    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_comparison();

        while self.match_tokens(&[Token::Equal, Token::NotEqual]) {
            let op = match self.previous() {
                Token::Equal => BinOp::Equal,
                Token::NotEqual => BinOp::NotEqual,
                _ => unreachable!(),
            };
            let right = self.parse_comparison();
            expr = Expr::Binary {
                op,
                lhs: Box::new(expr),
                rhs: Box::new(right),
            };
        }

        expr
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_additive();

        while self.match_tokens(&[Token::Greater, Token::GreaterEqual, Token::Less, Token::LessEqual]) {
            let op = match self.previous() {
                Token::Greater => BinOp::Greater,
                Token::GreaterEqual => BinOp::GreaterEqual,
                Token::Less => BinOp::Less,
                Token::LessEqual => BinOp::LessEqual,
                _ => unreachable!(),
            };
            let right = self.parse_additive();
            expr = Expr::Binary {
                op,
                lhs: Box::new(expr),
                rhs: Box::new(right),
            };
        }

        expr
    }

    fn parse_additive(&mut self) -> Expr {
        let mut expr = self.parse_multiplicative();

        while self.match_tokens(&[Token::Mais, Token::Menos]) {
            let op = match self.previous() {
                Token::Mais => BinOp::Add,
                Token::Menos => BinOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_multiplicative();
            expr = Expr::Binary {
                op,
                lhs: Box::new(expr),
                rhs: Box::new(right),
            };
        }

        expr
    }

    fn parse_multiplicative(&mut self) -> Expr {
        let mut expr = self.parse_unary();

        while self.match_tokens(&[Token::Multiplica, Token::Divide]) {
            let op = match self.previous() {
                Token::Multiplica => BinOp::Mul,
                Token::Divide => BinOp::Div,
                _ => unreachable!(),
            };
            let right = self.parse_unary();
            expr = Expr::Binary {
                op,
                lhs: Box::new(expr),
                rhs: Box::new(right),
            };
        }

        expr
    }

    fn parse_unary(&mut self) -> Expr {
        if self.match_tokens(&[Token::Not, Token::Menos]) {
            let op = match self.previous() {
                Token::Not => UnaryOp::Not,
                Token::Menos => UnaryOp::Minus,
                _ => unreachable!(),
            };
            let expr = self.parse_unary();
            return Expr::Unary {
                op,
                expr: Box::new(expr),
            };
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expr {
        // Número
        if let Token::Number(n) = self.peek() {
            let num = *n;
            self.advance();
            return Expr::Number(num);
        }

        // Booleanos
        if self.match_token(&Token::True) {
            return Expr::Bool(true);
        }

        if self.match_token(&Token::False) {
            return Expr::Bool(false);
        }

        // Identificador (variável ou chamada de função)
        if let Token::Ident(name) = self.peek() {
            let name = name.clone();
            self.advance();

            // Verifica se é uma chamada de função
            if self.match_token(&Token::AbrePar) {
                let mut args = Vec::new();

                if !self.check(&Token::FechaPar) {
                    loop {
                        args.push(self.parse_expression());
                        if !self.match_token(&Token::Virgula) {
                            break;
                        }
                    }
                }

                self.consume(&Token::FechaPar, "Esperado ')'");
                return Expr::Call { name, args };
            }

            // É uma variável
            return Expr::Var(name);
        }

        // Expressão entre parênteses
        if self.match_token(&Token::AbrePar) {
            let expr = self.parse_expression();
            self.consume(&Token::FechaPar, "Esperado ')'");
            return expr;
        }

        panic!("Expressão inválida");
    }

    // Métodos utilitários
    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_tokens(&mut self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(self.peek()) == std::mem::discriminant(token)
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn peek_ahead(&self, distance: usize) -> Option<&Token> {
        let index = self.current + distance;
        if index < self.tokens.len() {
            Some(&self.tokens[index])
        } else {
            None
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token: &Token, message: &str) {
        if self.check(token) {
            self.advance();
        } else {
            panic!("{}", message);
        }
    }
}
