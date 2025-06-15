use std::collections::HashMap;

use crate::ast::Node;
use crate::visitor::Visitor;

#[derive(Debug, Default, Clone)]
pub struct VariableIndexVisitor {
    pub index: HashMap<String, usize>,
}

impl VariableIndexVisitor {
    pub fn new() -> Self {
        VariableIndexVisitor {
            index: HashMap::new(),
        }
    }

    pub fn variable_names(self) -> Vec<String> {
        let mut vector = vec![String::new(); self.index.len()];
        for (name, index) in self.index {
            vector.insert(index, name);
        }
        vector
    }
}

impl Visitor for VariableIndexVisitor {
    fn pre_visit(&mut self, node: &Node) {
        match node {
            Node::Variable(name, idx_cell) => {
                let index = if let Some(&i) = self.index.get(name) {
                    i
                } else {
                    let new_index = self.index.len();
                    self.index.insert(name.clone(), new_index);
                    new_index
                };
                idx_cell.set(Some(index));
            }
            Node::Assign(name, _) => {
                if !self.index.contains_key(name) {
                    let new_index = self.index.len();
                    self.index.insert(name.clone(), new_index);
                }
            }
            _ => {}
        }
    }

    fn post_visit(&mut self, _node: &Node) {
        // No action needed after visiting children
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ExprTree, Node, walk_node};
    use std::cell::Cell;

    fn boxed(n: Node) -> ExprTree {
        Box::new(n)
    }

    #[test]
    fn empty_indexer_does_not_panic() {
        let indexer = VariableIndexVisitor::new();
        let names = indexer.variable_names();
        assert!(names.is_empty());
    }

    #[test]
    fn collects_single_variable() {
        let expr = boxed(Node::Assign("x".into(), boxed(Node::Constant(1.0))));
        let mut indexer = VariableIndexVisitor::new();
        walk_node(&mut indexer, &expr);
        assert_eq!(indexer.index.get("x"), Some(&0));
        let names = indexer.variable_names();
        assert!(names.contains(&"x".to_string()));
    }

    #[test]
    fn populates_variable_index() {
        let expr = boxed(Node::Variable("x".into(), Cell::new(None)));
        let mut indexer = VariableIndexVisitor::new();
        walk_node(&mut indexer, &expr);
        assert_eq!(indexer.index.get("x"), Some(&0));
        if let Node::Variable(_, ref cell) = *expr {
            assert_eq!(cell.get(), Some(0));
        } else {
            panic!("expected variable node");
        }
    }

    #[test]
    fn collects_multiple_variables_without_duplicates() {
        let expr = boxed(Node::Add(
            boxed(Node::Assign("a".into(), boxed(Node::Constant(1.0)))),
            boxed(Node::Assign("b".into(), boxed(Node::Variable("a".into(), Cell::new(None))))),
        ));

        let mut indexer = VariableIndexVisitor::new();
        walk_node(&mut indexer, &expr);
        assert_eq!(indexer.index.get("a"), Some(&0));
        assert_eq!(indexer.index.get("b"), Some(&1));
        let names = indexer.variable_names();
        assert!(names.contains(&"a".to_string()));
        assert!(names.contains(&"b".to_string()));
    }
}
