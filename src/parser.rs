use crate::lexer::Token;
use crate::ast::{Stmt, Expr, Function, BinOp};

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
        // Consome 'int'
        self.consume(&Token::Int, "Esperado 'int'");

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
                // Tipo do parâmetro (int)
                self.consume(&Token::Int, "Esperado 'int'");

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
        // Declaração de variável: int nome = expressão;
        if self.match_token(&Token::Int) {
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

        // Expression statement
        let expr = self.parse_expression();
        self.consume(&Token::PontoEVirgula, "Esperado ';'");
        Stmt::ExprStmt(expr)
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_additive()
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
        let mut expr = self.parse_primary();

        while self.match_tokens(&[Token::Multiplica, Token::Divide]) {
            let op = match self.previous() {
                Token::Multiplica => BinOp::Mul,
                Token::Divide => BinOp::Div,
                _ => unreachable!(),
            };
            let right = self.parse_primary();
            expr = Expr::Binary {
                op,
                lhs: Box::new(expr),
                rhs: Box::new(right),
            };
        }

        expr
    }

    fn parse_primary(&mut self) -> Expr {
        // Número
        if let Token::Number(n) = self.peek() {
            let num = *n;
            self.advance();
            return Expr::Number(num);
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
