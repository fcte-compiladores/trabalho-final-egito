use mini_interpretador::{Lexer, Parser, Interpreter, Value};

//função auxiliar que executa todo o pipeline do lexer ao interpretador
fn run_program(input: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let mut lexer = Lexer::nova_instancia(input);
    let tokens = lexer.tokenizador();
    let mut parser = Parser::new(tokens);
    let (functions, main_body) = parser.parse();
    let mut interpreter = Interpreter::new();
    Ok(interpreter.interpret(functions, main_body)?)
}

#[test]
fn test_operacoes_aritmeticas() {
  //verifica a precedência dos operadores
    let result = run_program("int main() { return 2 + 3 * 4; }").unwrap();
    if let Value::Number(n) = result {
        assert_eq!(n, 14); // 2 + (3 * 4)
    } else {
        panic!("Expected number result");
    }
}

#[test]
fn test_chamada_funcao() {
  //testar se o interpretador consegue interpretar funcoes definidas pelo usuario
    let input = r#"
        int soma(int a, int b) { 
            return a + b; 
        }
        int main() { 
            return soma(5, 3); 
        }
    "#;
    
    let result = run_program(input).unwrap();
    if let Value::Number(n) = result {
        assert_eq!(n, 8);
    } else {
        panic!("Expected number result");
    }
}

#[test]
fn test_designacao_variavel() {
  //verifica a declaração e uso de variáveis locais
    let input = r#"
        int main() {
            int x = 10;
            int y = 20;
            return x + y;
        }
    "#;
    
    let result = run_program(input).unwrap();
    if let Value::Number(n) = result {
        assert_eq!(n, 30);
    } else {
        panic!("Expected number result");
    }
}

#[test]
fn test_execucao_condicional() {
  //verifica se o controle de fluxo funciona certinho
    let input = r#"
        int main() {
            if (true) {
                return 42;
            } else {
                return 0;
            }
        }
    "#;
    
    let result = run_program(input).unwrap();
    if let Value::Number(n) = result {
        assert_eq!(n, 42);
    } else {
        panic!("Expected number result");
    }
}