use std::collections::HashMap;
use crate::ast::{Function};

#[derive(Default)]
pub struct Environment {
    pub variables: HashMap<String, i64>,
    pub functions: HashMap<String, Function>,
}

