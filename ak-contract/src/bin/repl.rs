use std::collections::HashMap;
use std::io::{self, Write};

use ak_contract::ast::{self, Node};
use ak_contract::parser::parse_expression;
use ak_contract::visitor::Visitor;

struct ReplEvaluator<'a> {
    env: &'a mut HashMap<String, f64>,
    float_stack: Vec<f64>,
    bool_stack: Vec<bool>,
}

impl<'a> ReplEvaluator<'a> {
    fn new(env: &'a mut HashMap<String, f64>) -> Self {
        Self { env, float_stack: Vec::new(), bool_stack: Vec::new() }
    }
}

impl<'a> Visitor for ReplEvaluator<'a> {
    fn pre_visit(&mut self, _node: &Node) {}

    fn post_visit(&mut self, node: &Node) {
        match node {
            Node::Constant(v) => self.float_stack.push(*v),
            Node::Uplus(_) => {
                if let Some(v) = self.float_stack.pop() { self.float_stack.push(v); }
            }
            Node::Uminus(_) => {
                if let Some(v) = self.float_stack.pop() { self.float_stack.push(-v); }
            }
            Node::Add(_, _) => {
                if let (Some(r), Some(l)) = (self.float_stack.pop(), self.float_stack.pop()) {
                    self.float_stack.push(l + r);
                }
            }
            Node::Sub(_, _) => {
                if let (Some(r), Some(l)) = (self.float_stack.pop(), self.float_stack.pop()) {
                    self.float_stack.push(l - r);
                }
            }
            Node::Mul(_, _) => {
                if let (Some(r), Some(l)) = (self.float_stack.pop(), self.float_stack.pop()) {
                    self.float_stack.push(l * r);
                }
            }
            Node::Div(_, _) => {
                if let (Some(r), Some(l)) = (self.float_stack.pop(), self.float_stack.pop()) {
                    self.float_stack.push(l / r);
                }
            }
            Node::Pow(_, _) => {
                if let (Some(r), Some(l)) = (self.float_stack.pop(), self.float_stack.pop()) {
                    self.float_stack.push(l.powf(r));
                }
            }
            Node::Log(_) => {
                if let Some(v) = self.float_stack.pop() { self.float_stack.push(v.ln()); }
            }
            Node::Sqrt(_) => {
                if let Some(v) = self.float_stack.pop() { self.float_stack.push(v.sqrt()); }
            }
            Node::Max(_, _) => {
                if let (Some(r), Some(l)) = (self.float_stack.pop(), self.float_stack.pop()) {
                    self.float_stack.push(l.max(r));
                }
            }
            Node::Min(_, _) => {
                if let (Some(r), Some(l)) = (self.float_stack.pop(), self.float_stack.pop()) {
                    self.float_stack.push(l.min(r));
                }
            }
            Node::Equal(_, _) => {
                if let (Some(r), Some(l)) = (self.float_stack.pop(), self.float_stack.pop()) {
                    self.bool_stack.push((l - r).abs() < f64::EPSILON);
                }
            }
            Node::Different(_, _) => {
                if let (Some(r), Some(l)) = (self.float_stack.pop(), self.float_stack.pop()) {
                    self.bool_stack.push((l - r).abs() >= f64::EPSILON);
                }
            }
            Node::Superior(_, _) => {
                if let (Some(r), Some(l)) = (self.float_stack.pop(), self.float_stack.pop()) {
                    self.bool_stack.push(l > r);
                }
            }
            Node::SuperiorEqual(_, _) => {
                if let (Some(r), Some(l)) = (self.float_stack.pop(), self.float_stack.pop()) {
                    self.bool_stack.push(l >= r);
                }
            }
            Node::Inferior(_, _) => {
                if let (Some(r), Some(l)) = (self.float_stack.pop(), self.float_stack.pop()) {
                    self.bool_stack.push(l < r);
                }
            }
            Node::InferiorEqual(_, _) => {
                if let (Some(r), Some(l)) = (self.float_stack.pop(), self.float_stack.pop()) {
                    self.bool_stack.push(l <= r);
                }
            }
            Node::And(_, _) => {
                if let (Some(r), Some(l)) = (self.bool_stack.pop(), self.bool_stack.pop()) {
                    self.bool_stack.push(l && r);
                }
            }
            Node::Or(_, _) => {
                if let (Some(r), Some(l)) = (self.bool_stack.pop(), self.bool_stack.pop()) {
                    self.bool_stack.push(l || r);
                }
            }
            Node::Assign(name, _) => {
                if let Some(val) = self.float_stack.pop() {
                    self.env.insert(name.clone(), val);
                    self.float_stack.push(val);
                }
            }
            Node::Spot(_) => {}
            Node::If(_, _, _) => {
                if let (Some(else_v), Some(then_v), Some(cond)) = (
                    self.float_stack.pop(),
                    self.float_stack.pop(),
                    self.bool_stack.pop(),
                ) {
                    self.float_stack.push(if cond { then_v } else { else_v });
                }
            }
            Node::Variable(name, _) => {
                let val = *self.env.get(name).unwrap_or(&0.0);
                self.float_stack.push(val);
            }
        }
    }
}

fn main() {
    let mut env: HashMap<String, f64> = HashMap::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if stdin.read_line(&mut input).unwrap() == 0 { break; }
        let line = input.trim();
        if line.is_empty() { continue; }
        if line == "exit" || line == "quit" { break; }
        match parse_expression(line) {
            Ok(expr) => {
                let mut evaluator = ReplEvaluator::new(&mut env);
                ast::walk_node(&mut evaluator, &expr);
                if let Some(v) = evaluator.float_stack.last() {
                    println!("{}", v);
                } else if let Some(b) = evaluator.bool_stack.last() {
                    println!("{}", b);
                }
            }
            Err(e) => eprintln!("Error: {e:?}"),
        }
    }
}

