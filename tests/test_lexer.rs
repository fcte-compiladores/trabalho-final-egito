use mini_interpretador::{Lexer, Token};

#[test]
fn test_tokenizar_tokens_basicos() {
//testa números e operadores básicos
    let mut lexer = Lexer::nova_instancia("123 + 456");
    let tokens = lexer.tokenizador();
    
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::Number(123));
    assert_eq!(tokens[1], Token::Mais);
    assert_eq!(tokens[2], Token::Number(456));
}

#[test]
fn test_tokenizar_palavras_chave() {
//teste de palavras-chave
    let mut lexer = Lexer::nova_instancia("int bool return if else while");
    let tokens = lexer.tokenizador();
    
    assert_eq!(tokens[0], Token::Int);
    assert_eq!(tokens[1], Token::Bool);
    assert_eq!(tokens[2], Token::Return);
    assert_eq!(tokens[3], Token::If);
    assert_eq!(tokens[4], Token::Else);
    assert_eq!(tokens[5], Token::While);
}

#[test]
fn test_tokenizar_identificadores() {
//testa nomes de variáveis e funções
    let mut lexer = Lexer::nova_instancia("main soma variavel_teste");
    let tokens = lexer.tokenizador();
    
    assert_eq!(tokens[0], Token::Ident("main".to_string()));
    assert_eq!(tokens[1], Token::Ident("soma".to_string()));
    assert_eq!(tokens[2], Token::Ident("variavel_teste".to_string()));
}

#[test]
fn test_tokenizar_operadores() {
//faz um teste de operadores lógicos e de comparação
    let mut lexer = Lexer::nova_instancia("== != <= >= && ||");
    let tokens = lexer.tokenizador();
    
    assert_eq!(tokens[0], Token::Equal);
    assert_eq!(tokens[1], Token::NotEqual);
    assert_eq!(tokens[2], Token::LessEqual);
    assert_eq!(tokens[3], Token::GreaterEqual);
    assert_eq!(tokens[4], Token::And);
    assert_eq!(tokens[5], Token::Or);
}

#[test]
fn test_tokenizar_pontuacao() {
//testa os símbolos de pontuações, sinais gráficos e afins
    let mut lexer = Lexer::nova_instancia("( ) { } ; ,");
    let tokens = lexer.tokenizador();
    
    assert_eq!(tokens[0], Token::AbrePar);
    assert_eq!(tokens[1], Token::FechaPar);
    assert_eq!(tokens[2], Token::AbreChave);
    assert_eq!(tokens[3], Token::FechaChave);
    assert_eq!(tokens[4], Token::PontoEVirgula);
    assert_eq!(tokens[5], Token::Virgula);
}

#[test]
fn test_tokenizar_booleanos() {
//testa valores booleanos
    let mut lexer = Lexer::nova_instancia("true false");
    let tokens = lexer.tokenizador();
    
    assert_eq!(tokens[0], Token::True);
    assert_eq!(tokens[1], Token::False);
}