// 3. Implement a data structure for graphs that allows modification (insertion, deletion).
//  It should be possible to store values at edges and nodes.
//  It might be easiest to use a dictionary of (node, edgelist) to do this.

// node contains a value and list of 'edges' pointing to other nodes
#[allow(dead_code)]
#[derive(Debug)]
struct Node<T> {
    value: T,
    edges: Vec<Edge<T>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Edge<T> {
    value: T,
    from: Node<T>,
    to: Node<T>,
}

fn main() {
    let mut graph: Vec<Node<String>> = Vec::new();
    graph.push(Node {
        value: String::from("something"),
        edges: vec![
            Edge {
                value: String::from("foo"),
                from: Node {
                    value: String::from("node"),
                    edges: Vec::new(),
                },
                to: Node {
                    value: String::from("node2"),
                    edges: Vec::new(),
                },
            },
            Edge {
                value: String::from("bar"),
                from: Node {
                    value: String::from("baz"),
                    edges: Vec::new(),
                },
                to: Node {
                    value: String::from("baz2"),
                    edges: Vec::new(),
                },
            },
        ],
    });
    println!("{:?}", graph);
}
