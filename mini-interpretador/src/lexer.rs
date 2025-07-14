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
    loop {
        let proximo_char = self.chars.next()?;

        if proximo_char.is_whitespace() {
            continue; // ignora espaços e quebras de linha
        }

        if let Some(digit) = proximo_char.to_digit(10) {
            let mut number = digit as i64;
            while let Some(&next_char) = self.chars.clone().as_str().chars().next().as_ref() {
                if let Some(d) = next_char.to_digit(10) {
                    number = number * 10 + d as i64;
                    self.chars.next(); // consome o dígito
                } else {
                    break;
                }
            }
            return Some(Token::Number(number));
        }

        let token = match proximo_char {
            '+' => Some(Token::Mais),
            '-' => Some(Token::Menos),
            '*' => Some(Token::Multiplica),
            '/' => Some(Token::Divide),
            _ => None,
        };

        if let Some(tok) = token {
            return Some(tok);
        }

        // Se não reconheceu o caractere, ignora
    }
}

}
