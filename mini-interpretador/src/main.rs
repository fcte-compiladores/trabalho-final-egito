mod ast;
use ast::lexer::{Lexer};
use std::io::{self, Write};

fn main() {
    println!("Olá. Este é um mini interpretador de expressões aritméticas em Rust");
    println!("Digite uma expressão aritmética abaixo, como: '3 + (4 * 2)':");

    //Entrada do usuário
    let mut input = String::new();
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).expect("Falha ao ler entrada. Digite uma expressão aritmética válida!");

    let input = input.trim();
    let mut lexer = Lexer::nova_instancia(input);
    let tokens = lexer.tokenizador();
    
    //Em caso de sucesso - input
    println!("A expressão que você digitou é válida!");
    println!("Tokens gerados:");
    for token in tokens {
        println!("{:?}", token);
    }
}
