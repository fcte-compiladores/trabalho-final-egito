# Mini Interpretador de Rust
Este projeto serve como um pequeno interpretador de uma linguagem similar ao C/C++ em Rust

### Pré-requisitos
- Rust 1.70+ (instale via [rustup.rs](https://rustup.rs/))
- Cargo (incluído com Rust)

## Como usar
Basta usar o arquivo 'programa.mc' na pasta src ou criar um novo arquivo com a
extensão .mc e alterar o main.rs para que ele leia o nome do novo arquivo feito!

Após isto, dentro da pasta src, basta digitar:
```bash
git clone <url-do-repositório>
cd mini_interpretador

cargo build
cargo run 
```
### Testes
Para executar testes, basta executar:
```bash
cargo test
```


## Arquitetura do Interpretador
Ele implementa as seguintes análises:
1. Análise Léxica: converte o código em tokens
2. Análise Sintática: constroi uma AST, similar ao que tinha no Lox
3. Interpretação: executa o código através da AST

### Fluxo de execução:
```
Código Fonte (.mc) → Lexer → Tokens → Parser → AST → Interpreter → Resultado
```

## Escopo do programa
O programa abrange dados inteiros e booleanos, os 4 operadores aritméticos básicos e atribuição(=), operadores de comparação, lógicos e unários, além de expressões, return, funções com parâMetros, chamadas de função, variáveis locais, condicionais, laços de repetição e comandos de expressão!

### Exemplo de programa:
```c
int fatorial(int n) {
    if (n <= 1) {
        return 1;
    } else {
        return n * fatorial(n - 1);
    }
}

int main() {
    int resultado = fatorial(5);
    return resultado;  // retorna 120
}
```

### O que não está no programa
O programa não implementa quaisquer outras funcionalidades que já não tenham sido mencionadas acima!

## Referências
- [Documentação do Rust](https://www.rust-lang.org/) - Para documentação do Rust, em geral
- [Rustlings](https://rustlings.rust-lang.org/) - Exercícios que me ajudaram a entender a linguagem melhor
- [Crafting Interpreters](https://craftinginterpreters.com/) - algumas das estruturas usadas foram baseadas no repositório de Lox da matéria, que por sua vez foi inspirado no Crafting Interpreters.
- Repositório Lox de Compiladores (Lox-base) - muitas das estruturas são similares, então facilitou para o desenvolvimento deste projeto.