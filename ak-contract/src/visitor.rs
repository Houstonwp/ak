use crate::ast::Node;

pub mod debug;
pub mod evaluator;
pub mod variable_index;

pub trait Visitor {
    /// Called before recursing into children.
    fn pre_visit(&mut self, node: &Node);
    /// Called after all children have been visited.
    fn post_visit(&mut self, node: &Node);
}

pub trait ConstVisitor {
    /// Called before recursing into children.
    fn pre_visit(&self, node: &Node);
    /// Called after all children have been visited.
    fn post_visit(&self, node: &Node);
}
