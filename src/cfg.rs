use std::collections::HashMap;

use crate::{productionrule::ProductionRule, token::Token};

#[derive(Debug, Default)]
pub struct CFG {
    pub productions: HashMap<Token, Vec<ProductionRule>>,
    pub inverted_dep_graph: HashMap<Token, Vec<Token>>,
    pub alphabet: Vec<String>,
    pub variables: Vec<String>
}
