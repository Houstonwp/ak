use std::collections::HashMap;

use crate::ast::{Name, Node};
use crate::visitor::Visitor;

#[derive(Debug, Default, Clone)]
pub struct VariableIndexVisitor {
    pub index: HashMap<Name, usize>,
}

impl VariableIndexVisitor {
    pub fn new() -> Self {
        VariableIndexVisitor {
            index: HashMap::new(),
        }
    }
}

impl Visitor for VariableIndexVisitor {
    fn pre_visit(&mut self, node: &Node) {
        if let Node::Variable(ref name, _) = *node {
            if !self.index.contains_key(name) {
                let new_index = self.index.len();
                self.index.insert(name.clone(), new_index);
            }
        }
    }

    fn post_visit(&mut self, _node: &Node) {
        // No action needed after visiting children
    }
}
