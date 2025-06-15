use crate::{ast::Node, visitor::Visitor};

#[derive(Debug, Default)]
pub struct DebugVisitor {
    depth: usize,
}

impl DebugVisitor {
    pub fn new() -> Self {
        DebugVisitor { depth: 0 }
    }
}

impl Visitor for DebugVisitor {
    fn pre_visit(&mut self, node: &Node) {
        // print this node at current depth
        let indent = "  ".repeat(self.depth);
        println!("{}{:?}", indent, node);
        // go deeper for children
        self.depth += 1;
    }

    fn post_visit(&mut self, _node: &Node) {
        // back out after all children
        self.depth -= 1;
    }
}
