use mini_interpretador::{Lexer, Parser, Interpreter, Value};
fn main() {
    //Caso queira que outro programa seja lido, o path terá de ser mudado aqui
    //Por enquanto, o default/padrão será o programa.mc mesmo, que inclusive está nesta mesma pasta
    let input = std::fs::read_to_string("programa.mc").expect("Erro ao ler arquivo");
    println!("Este programa quebra seu código em pequenas partes, analisa-o léxica e
sintaticamente e o interpreta!");
    println!("Hora de rodar o seu programa. Ihuul! Vamos dividi-lo em 3:\n");
    //etapa 1: análise léxica
    let mut lexer = Lexer::nova_instancia(&input);
    let tokens = lexer.tokenizador();
    println!("Estes são os tokens, que representam a análise léxica do seu programa:");
    for token in &tokens {
        println!("{:?}", token);
    }
    println!();

    //etapa 2: análise sintática
    let mut parser = Parser::new(tokens);
    let (funcoes, main_body) = parser.parse();
    println!("\nAs funções definidas pela análise sintática são as seguintes:");
    for f in &funcoes {
        println!("{:#?}", f);
    }
    println!();

    println!("\nO corpo da main é este:");
    for stmt in &main_body {
        println!("{:#?}", stmt);
    }
    println!();

    //etapa 3: interpretação
    let mut interpreter = Interpreter::new();
    println!("Hora de executar o programa com a ajuda do interpretador!");
    match interpreter.interpret(funcoes, main_body) {
        Ok(result) => {
            println!("Programa executado com sucesso!");
            match result {
                Value::Number(n) => println!("Resultado: {}", n),
                Value::Bool(b) => println!("Resultado: {}", b),
                Value::Void => println!("Resultado: void"),
            }
        }
        //caso haja erro
        Err(e) => {
            eprintln!("Erro durante a execução: {}", e);
            std::process::exit(1);
        }
    }
}
