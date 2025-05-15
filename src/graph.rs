use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;
use crate::constraints::Team;

struct Edge {
    target: Rc<RefCell<Node>>,
    capacity: Option<u32>
}

struct Node {
    datum: &'static HashSet<Team>,
    edges: Vec<Edge>,
}

impl Node {
    fn new(
        datum: &'static HashSet<Team>
    ) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            datum,
            edges: Vec::new(),
        }))
    }

    fn add_edge(
        node: &Rc<RefCell<Node>>,
        target: Rc<RefCell<Node>>,
        capacity: Option<u32>
    ) -> None {
        node.borrow_mut().edges.push(Edge { target, capacity });
    }
}

    // fn traverse<F>(&self, f: &F, seen: &mut HashSet<&'static str>)
    // where F: Fn(&'static str)
    // {
    //     if seen.contains(&self.datum) {
    //         return;
    //     }
    //     f(self.datum);
    //     seen.insert(self.datum);
    //     for n in &self.edges {
    //         n.borrow().traverse(f, seen);
    //     }
    // }
    //
    // fn first(&self) -> Rc<RefCell<Node>> {
    //     self.edges[0].clone()
    // }

// fn foo(node: &Node) {
//     println!("foo: {}", node.datum);
// }

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