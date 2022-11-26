use std::sync::{Arc, Mutex};

struct Graph<'a, T> {
    root: &'a Arc<Mutex<Node<'a,T>>>,
}

fn add_node<'a,T>(g: Graph<'a,T>, edge_val: T, n:&'a Arc<Mutex<Node<'a,T>>>) -> Graph<'a,T> {
    let new_node:&'a Arc<Mutex<Node<'a, T>>> = n;
    let mut m= g.root.lock().unwrap();
    m.edges.push(Edge{ value: edge_val, from: &g.root, to: &new_node });
    g
}

// node contains a value and list of 'edges' pointing to other nodes
#[allow(dead_code)]
#[derive(Debug)]
struct Node<'a,T> {
    value: T,
    edges: Vec<Edge<'a,T>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Edge<'a, T> {
    value: T,
    from: &'a Arc<Mutex<Node<'a, T>>>,
    to: &'a Arc<Mutex<Node<'a,T>>>,
}

fn main() {
    let n: Arc<Mutex<Node<String>>> = Arc::new(
        Mutex::new(Node { value: (&"node_val").to_string(), edges: Vec::new() })
    );
    let g: Graph<String> = Graph { root: &n };
    add_node(g, "edge_val".to_string(), &n);
}
