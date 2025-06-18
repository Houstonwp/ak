use crate::ast::Node;
use crate::simulation::Scenario;
use crate::visitor::Visitor;
use std::cell::Cell;
use std::sync::Arc;

fn get_index(cell: &Cell<Option<usize>>) -> usize {
    cell.get().expect("variable index not set")
}

pub struct EvaluatorVisitor {
    pub variables: Vec<f64>,
    pub float_stack: Vec<f64>,
    pub bool_stack: Vec<bool>,
    pub scenario: Arc<Scenario>,
}

impl Visitor for EvaluatorVisitor {
    fn pre_visit(&mut self, _: &Node) {}

    fn post_visit(&mut self, node: &Node) {
        // Handle operations and logic here
        match *node {
            Node::Constant(value) => {
                self.float_stack.push(value);
            }
            Node::Uplus(_) => {
                let val = self.float_stack.pop().expect("Stack underflow");
                self.float_stack.push(val);
            }
            Node::Uminus(_) => {
                let val = self.float_stack.pop().expect("Stack underflow");
                self.float_stack.push(-val);
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
            Node::Pow(_, _) => {
                let right = self.float_stack.pop().expect("Stack underflow");
                let left = self.float_stack.pop().expect("Stack underflow");
                self.float_stack.push(left.powf(right));
            }
            Node::Log(_) => {
                let val = self.float_stack.pop().expect("Stack underflow");
                self.float_stack.push(val.ln());
            }
            Node::Sqrt(_) => {
                let val = self.float_stack.pop().expect("Stack underflow");
                self.float_stack.push(val.sqrt());
            }
            Node::Max(_, _) => {
                let right = self.float_stack.pop().expect("Stack underflow");
                let left = self.float_stack.pop().expect("Stack underflow");
                self.float_stack.push(left.max(right));
            }
            Node::Min(_, _) => {
                let right = self.float_stack.pop().expect("Stack underflow");
                let left = self.float_stack.pop().expect("Stack underflow");
                self.float_stack.push(left.min(right));
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
            Node::Superior(_, _) => {
                let right = self.float_stack.pop().expect("Stack underflow");
                let left = self.float_stack.pop().expect("Stack underflow");
                self.bool_stack.push(left > right);
            }
            Node::SuperiorEqual(_, _) => {
                let right = self.float_stack.pop().expect("Stack underflow");
                let left = self.float_stack.pop().expect("Stack underflow");
                self.bool_stack.push(left >= right);
            }
            Node::Inferior(_, _) => {
                let right = self.float_stack.pop().expect("Stack underflow");
                let left = self.float_stack.pop().expect("Stack underflow");
                self.bool_stack.push(left < right);
            }
            Node::InferiorEqual(_, _) => {
                let right = self.float_stack.pop().expect("Stack underflow");
                let left = self.float_stack.pop().expect("Stack underflow");
                self.bool_stack.push(left <= right);
            }
            Node::And(_, _) => {
                let right = self.bool_stack.pop().expect("Stack underflow");
                let left = self.bool_stack.pop().expect("Stack underflow");
                self.bool_stack.push(left && right);
            }
            Node::Or(_, _) => {
                let right = self.bool_stack.pop().expect("Stack underflow");
                let left = self.bool_stack.pop().expect("Stack underflow");
                self.bool_stack.push(left || right);
            }
            Node::Assign(_, _) => {
                let val = self.float_stack.pop().expect("Stack underflow");
                // assignment currently only yields value back to the stack
                self.float_stack.push(val);
            }
            Node::Spot(_) => {}
            Node::If(_, _, _) => {
                let else_val = self.float_stack.pop().expect("Stack underflow");
                let then_val = self.float_stack.pop().expect("Stack underflow");
                let cond = self.bool_stack.pop().expect("Stack underflow");
                self.float_stack
                    .push(if cond { then_val } else { else_val });
            }
            Node::Variable(_, ref cell) => {
                let idx = get_index(cell);
                let value = *self.variables.get(idx).unwrap_or(&0.0);
                self.float_stack.push(value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ExprTree, Node, walk_node};

    fn boxed(n: Node) -> ExprTree {
        Box::new(n)
    }

    fn eval_f(expr: ExprTree) -> f64 {
        let mut ev = EvaluatorVisitor {
            variables: vec![],
            float_stack: Vec::new(),
            bool_stack: Vec::new(),
            scenario: Arc::new(vec![]),
        };
        walk_node(&mut ev, &expr);
        ev.float_stack.pop().expect("no value")
    }

    fn eval_b(expr: ExprTree) -> bool {
        let mut ev = EvaluatorVisitor {
            variables: vec![],
            float_stack: Vec::new(),
            bool_stack: Vec::new(),
            scenario: Arc::new(vec![]),
        };
        walk_node(&mut ev, &expr);
        ev.bool_stack.pop().expect("no value")
    }

    fn eval_event(stmts: Vec<ExprTree>) -> EvaluatorVisitor {
        let mut ev = EvaluatorVisitor {
            variables: vec![],
            float_stack: Vec::new(),
            bool_stack: Vec::new(),
            scenario: Arc::new(vec![]),
        };
        for s in stmts.iter() {
            walk_node(&mut ev, s);
        }
        ev
    }

    #[test]
    fn add_two_numbers() {
        let expr = boxed(Node::Add(
            boxed(Node::Constant(1.0)),
            boxed(Node::Constant(2.0)),
        ));
        assert_eq!(eval_f(expr), 3.0);
    }

    #[test]
    fn unary_minus() {
        let expr = boxed(Node::Uminus(boxed(Node::Constant(5.0))));
        assert_eq!(eval_f(expr), -5.0);
    }

    #[test]
    fn power_operation() {
        let expr = boxed(Node::Pow(
            boxed(Node::Constant(2.0)),
            boxed(Node::Constant(3.0)),
        ));
        assert_eq!(eval_f(expr), 8.0);
    }

    #[test]
    fn boolean_and() {
        let left = boxed(Node::Equal(
            boxed(Node::Constant(1.0)),
            boxed(Node::Constant(1.0)),
        ));
        let right = boxed(Node::Different(
            boxed(Node::Constant(2.0)),
            boxed(Node::Constant(3.0)),
        ));
        let expr = boxed(Node::And(left, right));
        assert!(eval_b(expr));
    }

    #[test]
    fn uplus_is_identity() {
        let expr = boxed(Node::Uplus(boxed(Node::Constant(7.0))));
        assert_eq!(eval_f(expr), 7.0);
    }

    #[test]
    fn subtraction() {
        let expr = boxed(Node::Sub(
            boxed(Node::Constant(5.0)),
            boxed(Node::Constant(3.0)),
        ));
        assert_eq!(eval_f(expr), 2.0);
    }

    #[test]
    fn multiplication() {
        let expr = boxed(Node::Mul(
            boxed(Node::Constant(3.0)),
            boxed(Node::Constant(4.0)),
        ));
        assert_eq!(eval_f(expr), 12.0);
    }

    #[test]
    fn division() {
        let expr = boxed(Node::Div(
            boxed(Node::Constant(10.0)),
            boxed(Node::Constant(2.0)),
        ));
        assert_eq!(eval_f(expr), 5.0);
    }

    #[test]
    fn log_operation() {
        let expr = boxed(Node::Log(boxed(Node::Constant(std::f64::consts::E))));
        assert!((eval_f(expr) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn sqrt_operation() {
        let expr = boxed(Node::Sqrt(boxed(Node::Constant(9.0))));
        assert_eq!(eval_f(expr), 3.0);
    }

    #[test]
    fn max_operation() {
        let expr = boxed(Node::Max(
            boxed(Node::Constant(2.0)),
            boxed(Node::Constant(5.0)),
        ));
        assert_eq!(eval_f(expr), 5.0);
    }

    #[test]
    fn min_operation() {
        let expr = boxed(Node::Min(
            boxed(Node::Constant(2.0)),
            boxed(Node::Constant(5.0)),
        ));
        assert_eq!(eval_f(expr), 2.0);
    }

    #[test]
    fn equality() {
        let expr = boxed(Node::Equal(
            boxed(Node::Constant(2.0)),
            boxed(Node::Constant(2.0)),
        ));
        assert!(eval_b(expr));
    }

    #[test]
    fn inequality() {
        let expr = boxed(Node::Different(
            boxed(Node::Constant(2.0)),
            boxed(Node::Constant(3.0)),
        ));
        assert!(eval_b(expr));
    }

    #[test]
    fn superior_comparison() {
        let expr = boxed(Node::Superior(
            boxed(Node::Constant(5.0)),
            boxed(Node::Constant(3.0)),
        ));
        assert!(eval_b(expr));
    }

    #[test]
    fn superior_equal_comparison() {
        let expr = boxed(Node::SuperiorEqual(
            boxed(Node::Constant(3.0)),
            boxed(Node::Constant(3.0)),
        ));
        assert!(eval_b(expr));
    }

    #[test]
    fn inferior_comparison() {
        let expr = boxed(Node::Inferior(
            boxed(Node::Constant(3.0)),
            boxed(Node::Constant(5.0)),
        ));
        assert!(eval_b(expr));
    }

    #[test]
    fn inferior_equal_comparison() {
        let expr = boxed(Node::InferiorEqual(
            boxed(Node::Constant(3.0)),
            boxed(Node::Constant(3.0)),
        ));
        assert!(eval_b(expr));
    }

    #[test]
    fn boolean_or() {
        let left = boxed(Node::Equal(
            boxed(Node::Constant(0.0)),
            boxed(Node::Constant(1.0)),
        ));
        let right = boxed(Node::Equal(
            boxed(Node::Constant(2.0)),
            boxed(Node::Constant(2.0)),
        ));
        let expr = boxed(Node::Or(left, right));
        assert!(eval_b(expr));
    }

    #[test]
    fn assignment_returns_value() {
        let expr = boxed(Node::Assign("x".into(), boxed(Node::Constant(42.0))));
        assert_eq!(eval_f(expr), 42.0);
    }

    #[test]
    fn variable_lookup() {
        use std::cell::Cell;
        let expr = boxed(Node::Variable("x".into(), Cell::new(Some(0))));
        let mut ev = EvaluatorVisitor {
            variables: vec![10.0],
            float_stack: Vec::new(),
            bool_stack: Vec::new(),
            scenario: Arc::new(vec![]),
        };
        walk_node(&mut ev, &expr);
        assert_eq!(ev.float_stack.pop().unwrap(), 10.0);
    }

    #[test]
    fn if_statement() {
        let cond = boxed(Node::Equal(
            boxed(Node::Constant(1.0)),
            boxed(Node::Constant(1.0)),
        ));
        let then_branch = boxed(Node::Constant(7.0));
        let else_branch = boxed(Node::Constant(3.0));
        let statements = vec![cond, boxed(Node::If(0, then_branch, else_branch))];
        let ev = eval_event(statements);
        assert_eq!(ev.float_stack.last().copied(), Some(7.0));
    }
}
