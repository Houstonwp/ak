use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name {
    pub value: String,
}

pub trait Visitor {
    /// Called on each node as you walk the tree.
    fn visit(&mut self, node: &Node);
}

#[derive(Debug)]
pub enum Node {
    Uplus(ExprTree),
    Uminus(ExprTree),
    Add(ExprTree, ExprTree),
    Sub(ExprTree, ExprTree),
    Mul(ExprTree, ExprTree),
    Div(ExprTree, ExprTree),
    Pow(ExprTree, ExprTree),
    Log(ExprTree),
    Sqrt(ExprTree),
    Max(ExprTree, ExprTree),
    Min(ExprTree, ExprTree),
    Equal(ExprTree, ExprTree),
    Different(ExprTree, ExprTree),
    Superior(ExprTree, ExprTree),
    SuperiorEqual(ExprTree, ExprTree),
    Inferior(ExprTree, ExprTree),
    InferiorEqual(ExprTree, ExprTree),
    And(ExprTree, ExprTree),
    Or(ExprTree, ExprTree),
    Assign(ExprTree, ExprTree),
    Spot(ExprTree),
    If(isize, ExprTree, ExprTree),
    Constant(f64),
    Variable(Name, Box<Node>),
}

pub type DateIndex = isize;
pub type ExprTree = Box<Node>;
pub type Statement = ExprTree;
pub type Event = Vec<Statement>;

pub struct Product {
    pub event_dates: Vec<DateIndex>,
    pub events: Vec<Event>,
}
impl From<HashMap<DateIndex, Event>> for Product {
    fn from(events: HashMap<DateIndex, Event>) -> Self {
        let mut event_dates = Vec::with_capacity(events.len());
        let mut events_vec = Vec::with_capacity(events.len());

        for (date, event) in events {
            event_dates.push(date);
            events_vec.push(event);
        }

        Product {
            event_dates,
            events: events_vec,
        }
    }
}
pub fn walk_node(visitor: &mut impl Visitor, n: &Node) {
    // first, let the visitor process this node
    visitor.visit(n);

    // then recurse into any children
    match n {
        // unary operations and other single-child variants
        Node::Uplus(child)
        | Node::Uminus(child)
        | Node::Log(child)
        | Node::Sqrt(child)
        | Node::Spot(child) => {
            walk_node(visitor, child);
        }

        // binary operations (two children)
        Node::Add(l, r)
        | Node::Sub(l, r)
        | Node::Mul(l, r)
        | Node::Div(l, r)
        | Node::Pow(l, r)
        | Node::Max(l, r)
        | Node::Min(l, r)
        | Node::Equal(l, r)
        | Node::Different(l, r)
        | Node::Superior(l, r)
        | Node::SuperiorEqual(l, r)
        | Node::Inferior(l, r)
        | Node::InferiorEqual(l, r)
        | Node::And(l, r)
        | Node::Or(l, r)
        | Node::Assign(l, r) => {
            walk_node(visitor, l);
            walk_node(visitor, r);
        }

        // the `if` node has an index plus two sub-trees
        Node::If(_, then_branch, else_branch) => {
            walk_node(visitor, then_branch);
            walk_node(visitor, else_branch);
        }

        // variables carry a subtree too
        Node::Variable(_, expr) => {
            walk_node(visitor, expr);
        }

        // constants have no children, so nothing further to do
        Node::Constant(_) => {}
    }
}
