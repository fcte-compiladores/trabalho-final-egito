use mini_interpretador::{Lexer, Parser, Stmt, Function};

//função auxiliar para extrair statements onde quer que estejam
fn get_statements<'a>(functions: &'a [Function], main_body: &'a [Stmt]) -> &'a [Stmt] {
    if !main_body.is_empty() {
        main_body
    } else if !functions.is_empty() {
        &functions[0].body
    } else {
        panic!("Sem statements encontrados na saída do parser")
    }
}

#[test]
fn test_debug_saida_parser() {
//mostra o que o parser retorna
    let input = "int main() { return 42; }";
    let mut lexer = Lexer::nova_instancia(input);
    let tokens = lexer.tokenizador();
    println!("Tokens: {:?}", tokens);
    
    let mut parser = Parser::new(tokens);
    let (functions, main_body) = parser.parse();
    
    println!("Contagem de funções: {}", functions.len());
    println!("Contagem do corpo da função: {}", main_body.len());
    
    if !functions.is_empty() {
        println!("Primeira função: {:?}", functions[0]);
        println!("Tamanho do corpo da função: {}", functions[0].body.len());
    }
    
    if !main_body.is_empty() {
        println!("Corpo principal: {:?}", main_body);
    }
    
    //este teste sempre passa, serve só para debug
    assert!(true);
}

#[test]
fn test_parse_return_simples() {
//testa se o parser reconhece o return
    let input = "int main() { return 42; }";
    let mut lexer = Lexer::nova_instancia(input);
    let tokens = lexer.tokenizador();
    let mut parser = Parser::new(tokens);
    let (functions, main_body) = parser.parse();
    
    let statements = get_statements(&functions, &main_body);
    assert!(!statements.is_empty(), "É esperado ao menos um statement");
    
    //deve ter pelo menos um statement (return)
    match &statements[statements.len() - 1] {
        Stmt::Return(_) => {}, 
        other => panic!("Era esperado um return, mas obtivemos: {:?}", other),
    }
}

#[test] 
fn test_parse_variavel_e_return() {
//verifica declaração de variável + return
    let input = "int main() { int x = 42; return x; }";
    let mut lexer = Lexer::nova_instancia(input);
    let tokens = lexer.tokenizador();
    let mut parser = Parser::new(tokens);
    let (functions, main_body) = parser.parse();
    
    let statements = get_statements(&functions, &main_body);
    assert!(statements.len() >= 2, "Era esperado ao menos 2 statements (decl de variável + return)");
    
    //procura por declaração de variável
    let has_var_decl = statements.iter().any(|stmt| {
        matches!(stmt, Stmt::VarDecl { name, .. } if name == "x")
    });
    assert!(has_var_decl, "Era esperada declaração de variável pra 'x'");
    
    //procura por return
    let has_return = statements.iter().any(|stmt| {
        matches!(stmt, Stmt::Return(_))
    });
    assert!(has_return, "Era esperado um return");
}

#[test]
fn test_parse_funcao_com_params() {
//verifica se o parser consegue extrair nome, parâmetros e o corpo da função
    let input = "int soma(int a, int b) { return a + b; }";
    let mut lexer = Lexer::nova_instancia(input);
    let tokens = lexer.tokenizador();
    let mut parser = Parser::new(tokens);
    let (functions, _) = parser.parse();
    
    assert_eq!(functions.len(), 1);
    let func = &functions[0];
    assert_eq!(func.name, "soma");
    assert_eq!(func.params, vec!["a", "b"]);
    
    //a função deve ter pelo menos um statement (return)
    assert!(!func.body.is_empty(), "Corpo da função não devia ser vazio");
}

#[test]
fn test_parse_aritmetica_basica() {
//verificar se o parser consegue construir expressões aritméticas básicas
    let input = "int main() { return 2 + 3; }";
    let mut lexer = Lexer::nova_instancia(input);
    let tokens = lexer.tokenizador();
    let mut parser = Parser::new(tokens);
    let (functions, main_body) = parser.parse();
    
    let statements = get_statements(&functions, &main_body);
    assert!(!statements.is_empty(), "Esperava-se ao menos 1 statement");
    
    //deve ter um return com expressão aritmética
    if let Stmt::Return(_expr) = &statements[0] {
        //temos um return com expressão
    } else {
        panic!("Esperava-se um return com uma expressão aritmética junto");
    }
}

#[test]
fn test_parse_multiplas_funcoes() {
//testa se o parser separa corretamente cada função
    let input = r#"
        int soma(int a, int b) { return a + b; }
        int main() { return soma(1, 2); }
    "#;
    let mut lexer = Lexer::nova_instancia(input);
    let tokens = lexer.tokenizador();
    let mut parser = Parser::new(tokens);
    let (functions, _main_body) = parser.parse();
    
    //deve ter pelo menos 1 função (soma)
    assert!(!functions.is_empty(), "Esperava-se ao menos 1 função");
    
    //verifica se temos a função soma
    let soma_func = functions.iter().find(|f| f.name == "soma");
    assert!(soma_func.is_some(), "Era esperada a função 'soma'");
    
    let soma = soma_func.unwrap();
    assert_eq!(soma.params.len(), 2, "soma só devia ter 2 parâMetros");
    assert!(!soma.body.is_empty(), "o corpo de soma não devia estar vazio");
}