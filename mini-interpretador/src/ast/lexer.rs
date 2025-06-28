use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i64),
    Mais,
    Menos,
    Multiplica,
    Divide,
}

pub struct Lexer<'a> {
    //Isso aqui representa o próprio lexer e o chars funciona como iterador pelos chars da string
    chars: Chars<'a>    
}

impl<'a> Lexer<'a> {
    //Cria uma nova instância do lexer com a string digitada pelo usuário
    pub fn nova_instancia(input: &'a str) -> Self {
        Lexer { chars: input.chars() } 
    }   

pub fn tokenizador(&mut self) -> Vec<Token> {
//Transforma a string de entrada e retorna um vetor de token
    let mut tokens = Vec::new();
    while let Some(token) = self.proximo_token() {
        //Esse loop basicamente é chamado até não ter mais tokens para serem "lidos"
        tokens.push(token);
    }
    tokens
}

fn proximo_token(&mut self) -> Option<Token> {
    let proximo_char = self.chars.next()?;
    match proximo_char {
        '+' => Some(Token::Mais),
        '-' => Some(Token::Menos),
        '*' => Some(Token::Multiplica),
        '/' => Some(Token::Divide),
        '0'..='9' => {
            let mut number = proximo_char.to_digit(10)? as i64;
            while let Some(proximo_char) = self.chars.clone().next() {
                if let Some(digit) = proximo_char.to_digit(10) {
                    number = number * 10 + digit as i64;
                    self.chars.next();
                } else {
                    break;
                }
            }
            Some(Token::Number(number))
        }
        _ => None,
    }
}
}
