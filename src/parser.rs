use crate::lexer::Token;
use crate::ast::{Stmt, Expr, Function, BinOp, LogicalOp, UnaryOp};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

//implementação do funcionamento do parser, a nossa análise sintática!
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> (Vec<Function>, Vec<Stmt>) {
        let mut funcoes = Vec::new();
        let mut main_body = Vec::new();
        let mut in_main = false;

        while !self.is_at_end() {
            //verifica se é uma declaração de função
            if self.check(&Token::Int) && self.peek_ahead(1).map_or(false, |t| matches!(t, Token::Ident(_))) {
                let func = self.parse_function();
                if func.name == "main" {
                    in_main = true;
                    main_body = func.body;
                } else {
                    funcoes.push(func);
                }
            } else if in_main {
                //Faz o parse de statements dentro da main
                let stmt = self.parse_statement();
                main_body.push(stmt);
            } else {
                //avança se não conseguir fazer parse
                self.advance();
            }
        }

        (funcoes, main_body)
    }

    fn parse_function(&mut self) -> Function {
        //tipo de retorno (int ou booleano)
        if !self.match_tokens(&[Token::Int, Token::Bool]) {
            panic!("Esperado tipo de retorno");
        }

        //nome da função
        let name = if let Token::Ident(n) = self.advance() {
            n.clone()
        } else {
            panic!("Esperado nome da função");
        };

        //parâmetros
        self.consume(&Token::AbrePar, "Esperado '('");
        let mut params = Vec::new();

        if !self.check(&Token::FechaPar) {
            loop {
                //tipo do parâmetro (int ou booleano)
                if !self.match_tokens(&[Token::Int, Token::Bool]) {
                    panic!("Esperado tipo do parâmetro");
                }

                //nome do parâmetro
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

        //corpo da função
        self.consume(&Token::AbreChave, "Esperado '{'");
        let mut body = Vec::new();

        while !self.check(&Token::FechaChave) && !self.is_at_end() {
            body.push(self.parse_statement());
        }

        self.consume(&Token::FechaChave, "Esperado '}'");

        Function { name, params, body }
    }

    fn parse_statement(&mut self) -> Stmt {
        //declaração de variável: int/booleano e nome = expressão;
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

        //o uso do retorno
        if self.match_token(&Token::Return) {
            let expr = self.parse_expression();
            self.consume(&Token::PontoEVirgula, "Esperado ';'");
            return Stmt::Return(expr);
        }

        //o uso do if
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

        //o uso do while
        if self.match_token(&Token::While) {
            self.consume(&Token::AbrePar, "Esperado '('");
            let condition = self.parse_expression();
            self.consume(&Token::FechaPar, "Esperado ')'");

            let body = self.parse_block();

            return Stmt::While { condition, body };
        }

        //o uso do for
        if self.match_token(&Token::For) {
            self.consume(&Token::AbrePar, "Esperado '('");

            //inicialização (opcional)
            let init = if self.check(&Token::PontoEVirgula) {
                None
            } else {
                Some(Box::new(self.parse_statement()))
            };

            if init.is_none() {
                self.consume(&Token::PontoEVirgula, "Esperado ';'");
            }

            //condição (opcional)
            let condition = if self.check(&Token::PontoEVirgula) {
                None
            } else {
                Some(self.parse_expression())
            };
            self.consume(&Token::PontoEVirgula, "Esperado ';'");

            //atualização (opcional)
            let update = if self.check(&Token::FechaPar) {
                None
            } else {
                Some(self.parse_expression())
            };
            self.consume(&Token::FechaPar, "Esperado ')'");

            let body = self.parse_block();

            return Stmt::For { init, condition, update, body };
        }

        //o uso da expressão
        let expr = self.parse_expression();
        self.consume(&Token::PontoEVirgula, "Esperado ';'");
        Stmt::ExprStmt(expr)
    }

    //o parse do block
    fn parse_block(&mut self) -> Vec<Stmt> {
        self.consume(&Token::AbreChave, "Esperado '{'");
        let mut statements = Vec::new();

        while !self.check(&Token::FechaChave) && !self.is_at_end() {
            statements.push(self.parse_statement());
        }

        self.consume(&Token::FechaChave, "Esperado '}'");
        statements
    }

    //o parse da expressão
    fn parse_expression(&mut self) -> Expr {
        self.parse_logical_or()
    }

    //o parse da expressão lógica or
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

    //o parse da exŕessão lógica and
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

    //o parse da igualdade, caso uma expressãp seja igual à outra e afins
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

    //aqui é o parse da comparação
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

    //aqui é o parse que regulamente adição e subtração
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

    //aqui é o parse da multiplicação/divisão
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

    //esse parse mexe com os valores unários, como não, por exemplo, tal como tinha na gramática e
    //parser do Lox
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

    //aqui é o parse de tipos primários, como o número, booleano etc.
    fn parse_primary(&mut self) -> Expr {
        //número
        if let Token::Number(n) = self.peek() {
            let num = *n;
            self.advance();
            return Expr::Number(num);
        }

        //booleanos
        if self.match_token(&Token::True) {
            return Expr::Bool(true);
        }

        if self.match_token(&Token::False) {
            return Expr::Bool(false);
        }

        //identificador (variável ou chamada de função)
        if let Token::Ident(name) = self.peek() {
            let name = name.clone();
            self.advance();

            //verifica se é uma chamada de função
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

            //é uma variável
            return Expr::Var(name);
        }

        //expressão entre parênteses
        if self.match_token(&Token::AbrePar) {
            let expr = self.parse_expression();
            self.consume(&Token::FechaPar, "Esperado ')'");
            return expr;
        }

        panic!("Expressão inválida");
    }

    //métodos utilitários
    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    //verifica se os tokens correspondem a tokens esperados
    fn match_tokens(&mut self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    //verifica se o tipo de token é o mesmo do fornecido
    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(self.peek()) == std::mem::discriminant(token)
        }
    }

    //move para o próximo token
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    //verificar se acabou a lista de tokens
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    //retorna o token atual, mas sem avançar
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    //verifica tokens mais à frente na lista de tokens
    fn peek_ahead(&self, distance: usize) -> Option<&Token> {
        let index = self.current + distance;
        if index < self.tokens.len() {
            Some(&self.tokens[index])
        } else {
            None
        }
    }

    //retorna o token anterior em relação ao atual
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    //verifica se o token atual é o esperado e o consome
    fn consume(&mut self, token: &Token, message: &str) {
        if self.check(token) {
            self.advance();
        } else {
            panic!("{}", message);
        }
    }
}
