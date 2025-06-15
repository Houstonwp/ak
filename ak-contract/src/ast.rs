use std::collections::HashMap;

use crate::visitor::Visitor;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Name {
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
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
    visitor.pre_visit(n);

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
    visitor.post_visit(n);
}

#[cfg(test)]
mod tests {
    use super::{ExprTree, Name, Node, Product, Visitor, walk_node};
    use std::collections::HashMap;

    struct Counter {
        count: usize,
    }

    impl Visitor for Counter {
        fn pre_visit(&mut self, _node: &Node) {
            self.count += 1;
        }
        fn post_visit(&mut self, _node: &Node) {
            // no action needed after visiting children
        }
    }

    fn boxed(n: Node) -> ExprTree {
        Box::new(n)
    }

    #[test]
    fn walk_simple_add() {
        let expr = boxed(Node::Add(
            boxed(Node::Constant(1.0)),
            boxed(Node::Constant(2.0)),
        ));
        let mut v = Counter { count: 0 };
        walk_node(&mut v, &expr);
        assert_eq!(v.count, 3);
    }

    #[test]
    fn walk_if_branch() {
        let expr = boxed(Node::If(
            0,
            boxed(Node::Constant(1.0)),
            boxed(Node::Constant(2.0)),
        ));
        let mut v = Counter { count: 0 };
        walk_node(&mut v, &expr);
        assert_eq!(v.count, 3);
    }

    #[test]
    fn walk_variable() {
        let expr = boxed(Node::Variable(
            Name { value: "x".into() },
            boxed(Node::Add(
                boxed(Node::Constant(1.0)),
                boxed(Node::Constant(2.0)),
            )),
        ));
        let mut v = Counter { count: 0 };
        walk_node(&mut v, &expr);
        assert_eq!(v.count, 4);
    }

    #[test]
    fn product_from_hashmap() {
        let mut map: HashMap<super::DateIndex, super::Event> = HashMap::new();
        map.insert(1, vec![boxed(Node::Constant(1.0))]);
        map.insert(2, vec![boxed(Node::Constant(2.0))]);
        let product = Product::from(map.clone());
        assert_eq!(product.event_dates.len(), 2);
        assert_eq!(product.events.len(), 2);
        let reconstructed: HashMap<super::DateIndex, &super::Event> = product
            .event_dates
            .iter()
            .cloned()
            .zip(product.events.iter())
            .collect();
        for (k, v) in &map {
            let event = reconstructed.get(k).expect("missing event");
            assert_eq!(event.len(), v.len());
        }
    }
}
