use crate::ast::Node;
use crate::visitor::Visitor;

pub struct EvaluatorVisitor {
    pub variables: Vec<f64>,
    pub float_stack: Vec<f64>,
    pub bool_stack: Vec<bool>,
}

impl Visitor for EvaluatorVisitor {
    fn pre_visit(&mut self, _: &Node) {}

    fn post_visit(&mut self, node: &Node) {
        // Handle operations and logic here
        match *node {
            Node::Constant(value) => {
                self.float_stack.push(value);
            }
            Node::Add(_, _) => {
                let right = self.float_stack.pop().expect("Stack underflow");
                let left = self.float_stack.pop().expect("Stack underflow");
                self.float_stack.push(left + right);
            }
            Node::Sub(_, _) => {
                let right = self.float_stack.pop().expect("Stack underflow");
                let left = self.float_stack.pop().expect("Stack underflow");
                self.float_stack.push(left - right);
            }
            Node::Mul(_, _) => {
                let right = self.float_stack.pop().expect("Stack underflow");
                let left = self.float_stack.pop().expect("Stack underflow");
                self.float_stack.push(left * right);
            }
            Node::Div(_, _) => {
                let right = self.float_stack.pop().expect("Stack underflow");
                let left = self.float_stack.pop().expect("Stack underflow");
                if right == 0.0 {
                    panic!("Division by zero");
                }
                self.float_stack.push(left / right);
            }
            Node::Equal(_, _) => {
                let right = self.float_stack.pop().expect("Stack underflow");
                let left = self.float_stack.pop().expect("Stack underflow");
                self.bool_stack.push(left == right);
            }
            Node::Different(_, _) => {
                let right = self.float_stack.pop().expect("Stack underflow");
                let left = self.float_stack.pop().expect("Stack underflow");
                self.bool_stack.push(left != right);
            }
            _ => {}
        }
    }
}
