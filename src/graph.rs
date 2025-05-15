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
    pub capacity: Option<u32>,
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
        capacity: Option<u32>,
        kind: EdgeKind
    ) {
        node.borrow_mut().edges.push(Edge { target, capacity, kind });
    }

    fn traverse<F>(&self, f: &F, seen: &mut HashSet<*const Node>)
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

fn foo(node: &Node) {
    println!("foo: {:#?}", node.datum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph() {
        let teams: Vec<Team> = Vec::from([
            Team { name: "Argentina".into(), points: 0 },
            Team { name: "Mexico".into(), points: 1 },
            Team { name: "Poland".into(), points: 4 },
            Team { name: "Saudi Arabia".into(), points: 3 }
        ]);

        let root = Node::from([teams[0].clone()]);
        let node1 = Node::from([teams[1].clone()]);
        let node2 = Node::from([teams[2].clone()]);

        {
            let mut mut_root = root.borrow_mut();
            mut_root.edges.push({
                Edge {
                    target: node1.clone(),
                    capacity: None,
                    kind: EdgeKind::FromSource
                }
            });
            mut_root.edges.push({
                Edge {
                    target: node2.clone(),
                    capacity: None,
                    kind: EdgeKind::FromSource
                }
            });
        }

        let mut seen = HashSet::new();
        root.borrow().traverse(&|teams| {
            println!("Visiting node with teams: {:?}", teams);
        }, &mut seen);
    }
}
