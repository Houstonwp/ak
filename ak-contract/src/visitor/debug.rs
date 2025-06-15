use crate::{ast::Node, visitor::Visitor};
use std::io::{self, Write};

#[derive(Debug)]
pub struct DebugVisitor<W: Write> {
    depth: usize,
    writer: W,
}

impl<W: Write + Default> Default for DebugVisitor<W> {
    fn default() -> Self {
        Self {
            depth: 0,
            writer: W::default(),
        }
    }
}

impl DebugVisitor<std::io::Stdout> {
    pub fn new() -> Self {
        DebugVisitor {
            depth: 0,
            writer: io::stdout(),
        }
    }
}

impl<W: Write> DebugVisitor<W> {
    pub fn with_writer(writer: W) -> Self {
        DebugVisitor { depth: 0, writer }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W: Write> Visitor for DebugVisitor<W> {
    fn pre_visit(&mut self, node: &Node) {
        // print this node at current depth
        let indent = "  ".repeat(self.depth);
        writeln!(self.writer, "{}{:?}", indent, node).unwrap();
        // go deeper for children
        self.depth += 1;
    }

    fn post_visit(&mut self, _node: &Node) {
        // back out after all children
        self.depth -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Node, walk_node};
    use std::io::Cursor;

    fn boxed(n: Node) -> Box<Node> {
        Box::new(n)
    }

    fn capture_output(expr: &Node) -> String {
        let cursor = Cursor::new(Vec::new());
        let mut v = DebugVisitor::with_writer(cursor);
        walk_node(&mut v, expr);
        let cursor = v.into_inner();
        String::from_utf8(cursor.into_inner()).unwrap()
    }

    #[test]
    fn simple_add() {
        let expr = boxed(Node::Add(
            boxed(Node::Constant(1.0)),
            boxed(Node::Constant(2.0)),
        ));
        let output = capture_output(&expr);
        assert_eq!(
            output,
            "Add(Constant(1.0), Constant(2.0))\n  Constant(1.0)\n  Constant(2.0)\n"
        );
    }

    #[test]
    fn nested_add() {
        let expr = boxed(Node::Add(
            boxed(Node::Add(
                boxed(Node::Constant(1.0)),
                boxed(Node::Constant(2.0)),
            )),
            boxed(Node::Constant(3.0)),
        ));
        let output = capture_output(&expr);
        assert_eq!(
            output,
            "Add(Add(Constant(1.0), Constant(2.0)), Constant(3.0))\n  Add(Constant(1.0), Constant(2.0))\n    Constant(1.0)\n    Constant(2.0)\n  Constant(3.0)\n"
        );
    }

    #[test]
    fn if_node() {
        let expr = boxed(Node::If(
            0,
            boxed(Node::Constant(1.0)),
            boxed(Node::Constant(2.0)),
        ));
        let output = capture_output(&expr);
        assert_eq!(
            output,
            "If(0, Constant(1.0), Constant(2.0))\n  Constant(1.0)\n  Constant(2.0)\n"
        );
    }
}
