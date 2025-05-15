use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use crate::constraints::{ Team, Game };

// also a new/from function for Edge -- for ease of use
#[derive(Clone)]
struct Edge {
    target: Rc<RefCell<Node>>,
    capacity: Option<u32>
}

struct Node {
    datum: HashSet<Team>,
    edges: Vec<Edge>,
}

impl Node {
    fn new(
        datum: HashSet<Team>
    ) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            datum,
            edges: Vec::new(),
        }))
    }

    // implement a FROM function to build direct from teams

    fn add_edge(
        node: &Rc<RefCell<Node>>,
        target: Rc<RefCell<Node>>,
        capacity: Option<u32>
    ) {
        node.borrow_mut().edges.push(Edge { target, capacity });
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

    // fn first(&self) -> Rc<RefCell<Node>> {
    //     self.edges[0].clone()
    // }
}
fn foo(node: &Node) {
    println!("foo: {:#?}", node.datum);
}

// fn init() -> Rc<RefCell<Node>> {
//     let root = Node::new("A");
//
//     let b = Node::new("B");
//     let c = Node::new("C");
//     let d = Node::new("D");
//     let e = Node::new("E");
//     let f = Node::new("F");
//
//     {
//         let mut mut_root = root.borrow_mut();
//         mut_root.edges.push(b.clone());
//         mut_root.edges.push(c.clone());
//         mut_root.edges.push(d.clone());
//
//         let mut mut_c = c.borrow_mut();
//         mut_c.edges.push(e.clone());
//         mut_c.edges.push(f.clone());
//         mut_c.edges.push(root.clone());
//     }
//
//     root
// }

// pub fn main() {
//     let g = init();
//     let g = g.borrow();
//     g.traverse(&|d| println!("{}", d), &mut HashSet::new());
//     let f = g.first();
//     foo(&*f.borrow());
// }

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
        let team_refs: Vec<&Team> = teams.iter().collect();

        let games: Vec<Game> = Vec::from([
            Game::new(&teams[0], &teams[1], 0),
            Game::new(&teams[0], &teams[2], 0),
            Game::new(&teams[1], &teams[3], 0),
        ]);

        let root = {
            let mut datum = HashSet::new();
            datum.insert(teams[0].clone());
            Node::new(datum.clone())
        };

        let node1 = {
            let mut datum = HashSet::new();
            datum.insert(teams[1].clone());
            Node::new(datum.clone())
        };

        let node2 = {
            let mut datum = HashSet::new();
            datum.insert(teams[2].clone());
            Node::new(datum.clone())
        };

        {
            let mut mut_root = root.borrow_mut();
            mut_root.edges.push({
                let node1_clone = node1.clone();
                Edge {
                    target: node1_clone.clone(),
                    capacity: None
                }
            });
            mut_root.edges.push({
                let node2_clone = node2.clone();
                Edge {
                    target: node2_clone.clone(),
                    capacity: None
                }
            });
        }

        let mut seen = HashSet::new();
        root.borrow().traverse(&|teams| {
            println!("Visiting node with teams: {:?}", teams);
        }, &mut seen);
    }
}
