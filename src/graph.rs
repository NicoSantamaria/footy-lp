use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use crate::constraints::{ Team, Game };

#[derive(Clone)]
pub enum EdgeKind {
    FromSource,
    InfiniteCapacity,
    ToSink,
}
#[derive(Clone)]
pub struct Edge {
    pub target: Rc<RefCell<Node>>,
    pub capacity: Option<i32>,
    pub kind: EdgeKind
}

pub struct Node {
    pub datum: HashSet<Team>,
    pub edges: Vec<Edge>,
}

impl Node {
    pub fn new(
        datum: HashSet<Team>
    ) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            datum,
            edges: Vec::new(),
        }))
    }

    pub fn from(
        data: impl IntoIterator<Item = Team>
    ) -> Rc<RefCell<Node>> {
        let datum: HashSet<Team> = data.into_iter().collect();
        Node::new(datum)
    }

    fn add_edge(
        node: &Rc<RefCell<Node>>,
        target: Rc<RefCell<Node>>,
        capacity: Option<i32>,
        kind: EdgeKind
    ) {
        node.borrow_mut().edges.push(Edge { target, capacity, kind });
    }

    pub fn traverse<F>(&self, f: &F, seen: &mut HashSet<*const Node>)
    where
        F: Fn(&HashSet<Team>),
    {
        let ptr_self: *const Node = self;
        if seen.contains(&ptr_self) {
            return;
        }

        f(&self.datum);
        seen.insert(ptr_self);

        for edge in &self.edges {
            edge.target.borrow().traverse(f, seen);
        }
    }
}

pub fn foo(node: &Node) {
    println!("foo: {:#?}", node.datum);
}
