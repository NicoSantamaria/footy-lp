use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use crate::constraints::{Team, Game};

#[derive(Clone, Debug)]
pub enum EdgeKind {
    FromSource,
    InfiniteCapacity,
    ToSink,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub target: Rc<RefCell<Node>>,
    pub capacity: Option<i32>,
    pub kind: EdgeKind,
}

#[derive(Debug)]
pub struct Node {
    pub datum: HashSet<Team>,
    pub edges: Vec<Edge>,
}

impl Node {
    pub fn new(datum: HashSet<Team>) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            datum,
            edges: Vec::new(),
        }))
    }

    pub fn from(data: impl IntoIterator<Item = Team>) -> Rc<RefCell<Node>> {
        let datum: HashSet<Team> = data.into_iter().collect();
        Node::new(datum)
    }

    fn add_edge(
        node: &Rc<RefCell<Node>>,
        target: Rc<RefCell<Node>>,
        capacity: Option<i32>,
        kind: EdgeKind,
    ) {
        node.borrow_mut().edges.push(Edge {
            target,
            capacity,
            kind,
        });
    }

    /// Breadth-first traversal
    pub fn traverse_bfs<F>(&self, f: &F, seen: &mut HashSet<*const Node>)
    where
        F: Fn(&Node),
    {
        let mut queue = VecDeque::new();
        let ptr_self = self as *const Node;

        seen.insert(ptr_self);
        queue.push_back(self as *const Node);

        let mut ptr_to_node = |ptr: *const Node| unsafe { &*ptr };

        while let Some(current_ptr) = queue.pop_front() {
            let current = ptr_to_node(current_ptr);
            f(current);

            for edge in &current.edges {
                let target_ptr = &*edge.target.borrow() as *const Node;
                if !seen.contains(&target_ptr) {
                    seen.insert(target_ptr);
                    queue.push_back(target_ptr);
                }
            }
        }
    }
}

/// Top-level function to print nodes and edges using BFS
pub fn foo(start: &Rc<RefCell<Node>>) {
    let mut seen = HashSet::new();

    println!("Visited nodes and edges:");

    start.borrow().traverse_bfs(
        &|node| {
            println!("Node: {:?}", node.datum);
            for edge in &node.edges {
                let target = edge.target.borrow();
                println!(
                    "  └── Edge to: {:?}, capacity: {:?}, kind: {:?}",
                    target.datum,
                    edge.capacity,
                    edge.kind
                );
            }
        },
        &mut seen,
    );
}
