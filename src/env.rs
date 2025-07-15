use std::collections::HashMap;
use crate::ast::{Function};

// Aqui será o ambiente de execução do programa, em que variáveis e funções associadas a nomes
// serão acessadas. Elas estão em hash pra facilitar o acesso e ser mais dinâMica, de maneira geral
#[derive(Default)]
pub struct Environment {
    pub variables: HashMap<String, i64>,
    pub functions: HashMap<String, Function>,
}

