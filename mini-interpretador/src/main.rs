mod ast;

fn main() {
    println!("Olá. Este é um mini interpretador de expressões aritméticas em Rust");
    println!("Digite uma expressão aritmética abaixo, como: '3 + (4 * 2)':");
        
    //Em caso de sucesso - input
    println!("A expressão que você digitou é válida!");
    println!("Ela retorna...");
    
    //Em caso de falha - input
    println!("A expressão que você digitou não é válida!");
    println!("Atente-se aos formatos básicos (como +,-,/) e tente novamente.");
}
