use std::rc::Rc;
use std::cell::RefCell;

mod structs;
use crate::structs::{GraphNode, Edge, Graph, Point};

#[cfg(test)]
mod tests {
    use super::*;
    
    // #[test]
    // fn graph_build() {
    //     let graph: Graph = Graph::new();
    // }

    #[test]
    fn node_build() {
        let node: GraphNode = GraphNode::new("name".to_owned(), 1.0, 1.0, 1.0);
        assert_eq!(node.name, "name");
        assert_eq!(node.pos, Point{x:1.0,y:1.0,z:1.0});
    }

    #[test] 
    fn graph_comp() {
        let node: GraphNode = GraphNode::new("name".to_owned(), 1.0, 1.0, 1.0);
        assert_eq!(node, GraphNode::new("name".to_owned(), 1.0, 1.0, 1.0));
        assert_ne!(node, GraphNode::new("not name".to_owned(), 1.0, 1.0, 1.0));
    }

    #[test]
    fn move_test() {
        let mut node: GraphNode = GraphNode::new("name".to_owned(), 1.0, 1.0, 1.0);
        assert_eq!(node, GraphNode::new("name".to_owned(), 1.0, 1.0, 1.0));
        node.move_node(2.0,2.0,2.0);
        assert_eq!(node, GraphNode::new("name".to_owned(), 2.0, 2.0, 2.0));
    }
    

}

pub fn run() {
    let mut graph: Graph = Graph::new();
    while true {
        let mut input = String::new();
        println!("1) Add Node\n2) Add Edge\n3) View Graph\n");
        print!("Enter a Number:");
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            println!("Error reading input {}", e);
        }
        println!();
        match input.trim().to_lowercase().as_str() {
            "1" => {
                let mut name = String::new();
                let mut pos = Point{x:0.0, y:0.0, z:0.0};
                print!("Enter the node's name: ");
                if let Err(e) = std::io::stdin().read_line(&mut name) {
                    println!("Error reading input {}", e);
                }
                println!();
                let mut node = GraphNode::new(name, pos.x, pos.y, pos.z);
                graph.add_node(&mut node);
            },
            "2" => {
                let mut input_idx = String::new();
                for (index, node) in graph.list_nodes().iter().enumerate() {
                    println!("{}) {}", index, node);
                }
                print!("Enter a Number: ");
                if let Err(e) = std::io::stdin().read_line(&mut input_idx) {
                    println!("Error reading input {}", e);
                }
            },
            "3" => {

            },
            "q"|"quit"|"exit" => break,
            _ => println!("Invalid Option"),
        }
    }
    
    let mut node: GraphNode = GraphNode::new("Node 1:".to_owned(), 1.0, 1.0, 1.0);
    let refer: Rc<RefCell<&mut GraphNode>> = Rc::new(RefCell::new(&mut node));
    refer.borrow_mut().add_edge(Edge::new(1.0, &refer));
    println!("{}", refer.borrow().edges.len());
}
