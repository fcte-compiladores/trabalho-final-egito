mod lexer;
mod parser;
mod ast;
mod interpreter;

use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    //Caso queira que outro programa seja lido, o path terá de ser mudado
    let input = std::fs::read_to_string("programa.mc").expect("Erro ao ler arquivo");

    // Etapa 1: análise léxica
    let mut lexer = Lexer::nova_instancia(&input);
    let tokens = lexer.tokenizador();

    println!("=== TOKENS ===");
    for token in &tokens {
        println!("{:?}", token);
    }
    println!();

    // Etapa 2: análise sintática
    let mut parser = Parser::new(tokens);
    let (funcoes, main_body) = parser.parse();

    println!("=== FUNÇÕES DEFINIDAS ===");
    for f in &funcoes {
        println!("{:#?}", f);
    }
    println!();

    println!("=== CORPO DA MAIN ===");
    for stmt in &main_body {
        println!("{:#?}", stmt);
    }
    println!();

    // Etapa 3: interpretação
    let mut interpreter = Interpreter::new();

    println!("=== EXECUTANDO PROGRAMA ===");
    match interpreter.interpret(funcoes, main_body) {
        Ok(result) => {
            println!("Programa executado com sucesso!");
            match result {
                interpreter::Value::Number(n) => println!("Resultado: {}", n),
                interpreter::Value::Void => println!("Resultado: void"),
            }
        }
        Err(e) => {
            eprintln!("Erro durante a execução: {}", e);
            std::process::exit(1);
        }
    }
}
