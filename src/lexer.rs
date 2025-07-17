// O lexer basicamente faz análise léxica dividindo unidades em tokens e identificando qual o tipo
// deles
use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    // Literais
    Number(i64),
    Ident(String),
    True,
    False,
    // Palavras-chave
    Int,
    Bool,
    Return,
    If,
    Else,
    While,
    For,
    // Operadores aritméticos
    Mais,
    Menos,
    Multiplica,
    Divide,
    Assign, // '='
    // Operadores de comparação
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=
    // Operadores lógicos
    And,          // &&
    Or,           // ||
    Not,          // !
    // Pontuação
    AbrePar,      // (
    FechaPar,     // )
    AbreChave,    // {
    FechaChave,   // }
    PontoEVirgula,// ;
    Virgula,      // ,
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    #[allow(dead_code)]
    buffer: String,
}

impl<'a> Lexer<'a> {
    pub fn nova_instancia(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
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
            // Pula espaços em branco
            if c.is_whitespace() {
                continue;
            }

            // Trata comentários
            if c == '/' {
                if let Some(&next_char) = self.chars.peek() {
                    if next_char == '/' {
                        // Comentário de linha: // até o fim da linha
                        self.chars.next(); // consome o segundo '/'
                        self.skip_line_comment();
                        continue;
                    } else if next_char == '*' {
                        // Comentário de bloco: /* ... */
                        self.chars.next(); // consome o '*'
                        if self.skip_block_comment() {
                            continue;
                        } else {
                            // Se não conseguiu fechar o comentário, erro
                            panic!("Comentário de bloco não fechado");
                        }
                    } else {
                        // É apenas um operador de divisão
                        return Some(Token::Divide);
                    }
                } else {
                    // É apenas um operador de divisão no final do arquivo
                    return Some(Token::Divide);
                }
            }

            // Números
            if c.is_ascii_digit() {
                let mut number = c.to_digit(10).unwrap() as i64;
                while let Some(&next) = self.chars.peek() {
                    if next.is_ascii_digit() {
                        self.chars.next();
                        number = number * 10 + next.to_digit(10).unwrap() as i64;
                    } else {
                        break;
                    }
                }
                return Some(Token::Number(number));
            }

            // Identificadores e palavras-chave
            if c.is_ascii_alphabetic() || c == '_' {
                let mut ident = c.to_string();
                while let Some(&next) = self.chars.peek() {
                    if next.is_ascii_alphanumeric() || next == '_' {
                        self.chars.next();
                        ident.push(next);
                    } else {
                        break;
                    }
                }
                return Some(match ident.as_str() {
                    "int" => Token::Int,
                    "bool" => Token::Bool,
                    "return" => Token::Return,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    "for" => Token::For,
                    "true" => Token::True,
                    "false" => Token::False,
                    _ => Token::Ident(ident),
                });
            }

            // Outros tokens
            return Some(match c {
                '+' => Token::Mais,
                '-' => Token::Menos,
                '*' => Token::Multiplica,
                '(' => Token::AbrePar,
                ')' => Token::FechaPar,
                '{' => Token::AbreChave,
                '}' => Token::FechaChave,
                ';' => Token::PontoEVirgula,
                ',' => Token::Virgula,
                '=' => {
                    if let Some(&'=') = self.chars.peek() {
                        self.chars.next(); // consome o segundo '='
                        Token::Equal
                    } else {
                        Token::Assign
                    }
                }
                '!' => {
                    if let Some(&'=') = self.chars.peek() {
                        self.chars.next(); // consome o '='
                        Token::NotEqual
                    } else {
                        Token::Not
                    }
                }
                '<' => {
                    if let Some(&'=') = self.chars.peek() {
                        self.chars.next(); // consome o '='
                        Token::LessEqual
                    } else {
                        Token::Less
                    }
                }
                '>' => {
                    if let Some(&'=') = self.chars.peek() {
                        self.chars.next(); // consome o '='
                        Token::GreaterEqual
                    } else {
                        Token::Greater
                    }
                }
                '&' => {
                    if let Some(&'&') = self.chars.peek() {
                        self.chars.next(); // consome o segundo '&'
                        Token::And
                    } else {
                        continue; // ignora '&' sozinho por enquanto
                    }
                }
                '|' => {
                    if let Some(&'|') = self.chars.peek() {
                        self.chars.next(); // consome o segundo '|'
                        Token::Or
                    } else {
                        continue; // ignora '|' sozinho por enquanto
                    }
                }
                _ => continue, // ignora caracteres desconhecidos
            });
        }
        None
    }

    fn skip_line_comment(&mut self) {
        // Consome caracteres até encontrar nova linha ou fim do arquivo
        while let Some(c) = self.chars.next() {
            if c == '\n' {
                break;
            }
        }
    }

    fn skip_block_comment(&mut self) -> bool {
        // Consome caracteres até encontrar */ ou fim do arquivo
        while let Some(c) = self.chars.next() {
            if c == '*' {
                if let Some(&next_char) = self.chars.peek() {
                    if next_char == '/' {
                        self.chars.next(); // consome o '/'
                        return true; // Comentário fechado com sucesso
                    }
                }
            }
        }
        false // Chegou ao fim do arquivo sem fechar o comentário
    }
}
