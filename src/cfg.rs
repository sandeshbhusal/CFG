use std::collections::HashMap;

use crate::{productionrule::ProductionRule, token::Token};

#[derive(Debug, Default, Clone)]
pub struct CFG {
    pub productions: HashMap<Token, Vec<ProductionRule>>,
    pub alphabet: Vec<Token>,
    pub variables: Vec<Token>
}
