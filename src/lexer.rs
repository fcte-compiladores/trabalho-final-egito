
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    // Literais
    Number(i64),
    Ident(String),

    // Palavras-chave
    Int,
    Return,

    // Operadores
    Mais,
    Menos,
    Multiplica,
    Divide,
    Assign, // '='

    // Pontuação
    AbrePar,      // (
    FechaPar,     // )
    AbreChave,    // {
    FechaChave,   // }
    PontoEVirgula,// ;
    Virgula,      // ,
}

pub struct Lexer<'a> {
    chars: Chars<'a>,
    buffer: String,
}

impl<'a> Lexer<'a> {
    pub fn nova_instancia(input: &'a str) -> Self {
        Lexer {
            chars: input.chars(),
            buffer: String::new(),
        }
    }

    pub fn tokenizador(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.proximo_token() {
            tokens.push(token);
        }
        tokens
    }

    fn proximo_token(&mut self) -> Option<Token> {
        while let Some(c) = self.chars.next() {
            if c.is_whitespace() {
                continue;
            }

            if c.is_ascii_digit() {
                let mut number = c.to_digit(10).unwrap() as i64;
                while let Some(next) = self.chars.clone().next() {
                    if next.is_ascii_digit() {
                        self.chars.next();
                        number = number * 10 + next.to_digit(10).unwrap() as i64;
                    } else {
                        break;
                    }
                }
                return Some(Token::Number(number));
            }

            if c.is_ascii_alphabetic() || c == '_' {
                let mut ident = c.to_string();
                while let Some(next) = self.chars.clone().next() {
                    if next.is_ascii_alphanumeric() || next == '_' {
                        self.chars.next();
                        ident.push(next);
                    } else {
                        break;
                    }
                }

                return Some(match ident.as_str() {
                    "int" => Token::Int,
                    "return" => Token::Return,
                    _ => Token::Ident(ident),
                });
            }

            return Some(match c {
                '+' => Token::Mais,
                '-' => Token::Menos,
                '*' => Token::Multiplica,
                '/' => Token::Divide,
                '=' => Token::Assign,
                '(' => Token::AbrePar,
                ')' => Token::FechaPar,
                '{' => Token::AbreChave,
                '}' => Token::FechaChave,
                ';' => Token::PontoEVirgula,
                ',' => Token::Virgula,
                _ => continue, // ignora caracteres desconhecidos
            });
        }

        None
    }
}
