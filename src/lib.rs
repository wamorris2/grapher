mod structs;
use crate::structs::{GraphNode, Graph, Point};
use image::{Luma, GrayImage};

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

    // #[test]
    // fn move_test() {
    //     let mut node: GraphNode = GraphNode::new("name".to_owned(), 1.0, 1.0, 1.0);
    //     assert_eq!(node, GraphNode::new("name".to_owned(), 1.0, 1.0, 1.0));
    //     node.move_node(2.0,2.0,2.0);
    //     assert_eq!(node, GraphNode::new("name".to_owned(), 2.0, 2.0, 2.0));
    // }
    

}

pub fn draw_graph(graph: &Graph, img: &mut image::ImageBuffer<Luma<u8>, Vec<u8>>) 
{
    let (img_width, img_height) = img.dimensions();
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
    for node in graph.nodes.iter() {
        let pos = &node.borrow().pos;
        max_x = if pos.x > max_x { pos.x } else { max_x };
        max_y = if pos.y > max_y { pos.y } else { max_y };
        min_x = if pos.x < min_x { pos.x } else { min_x };
        min_y = if pos.y < min_y { pos.y } else { min_y };
    }
    let range_x = max_x-min_x;
    let range_y = max_y-min_y;
    let node_size: i32 = 3;
    for node in graph.nodes.iter() {
        let pos = &node.borrow().pos;
        let px: f32 = (pos.x - min_x) / range_x * img_width as f32;
        let py: f32 = (pos.y - min_y) / range_y * img_height as f32;
        for x in std::cmp::max(px as i32 -node_size, 0)..std::cmp::min(px as i32 +node_size, img_width as i32) {
            for y in std::cmp::max(py as i32-node_size, 0)..std::cmp::min(py as i32 +node_size, img_height as i32) {
                img.put_pixel(x as u32, y as u32, Luma([255]));
            }
        }
        for edge in node.borrow().edges.iter() {
            println!("Drawing Edge from {} to {}", &node.borrow().name, &edge.dest.borrow().name);
            let dst_pos = &edge.dest.borrow().pos;
            let dx: f32 = (dst_pos.x - min_x) / range_x * img_width as f32;
            let dy: f32 = (dst_pos.y - min_y) / range_y * img_height as f32;
            let m = dy-py/dx-px;
            for x in px as i32..dx as i32 {
                let y = x as f32 *m + px;
                img.put_pixel(x as u32, y as u32, Luma([123]));
            }
        }
    }
}

use std::io::BufRead;

pub fn run() {
    let mut graph: Graph = Graph::new();
    let stdin = std::io::stdin();
    loop {
        let mut input = String::new();
        println!("1) Add Node\n2) Add Edge\n3) View Graph");
        println!("Enter a Number:");
        let mut handle = stdin.lock();
        if let Err(e) = handle.read_line(&mut input) {
            println!("Error reading input {}", e);
        }
        println!();
        match input.trim().to_lowercase().as_str() {
            "1" => {
                let mut name = String::new();
                let mut nums = String::new();
                println!("Enter the node's name: ");
                if let Err(e) = handle.read_line(&mut name) {
                    println!("Error reading input {}", e);
                }
                println!();
                println!("Enter 3 Numbers: ");
                if let Err(e) = handle.read_line(&mut nums) {
                    println!("Error reading input {}", e);
                }
                let mut v = [0.0 as f32; 3];
                for (i, num) in nums.split(" ").enumerate() {
                    v[i] = match num.lines().next().unwrap().parse::<f32>() {
                        Ok(val) => val,
                        Err(_) => {
                            println!("Invalid option");
                            continue;
                        },
                    }
                }
                

                println!();
                graph.add_node(GraphNode::new(name.lines().next().unwrap().to_owned(), v[0], v[1], v[2]));
            },
            "2" => {
                loop { // input loop
                    let mut input_idx = String::new();
                    for (index, node) in graph.list_nodes().iter().enumerate() {
                        println!("{}) {}", index, node);
                    }
                    println!("Enter a Number: ");
                    if let Err(e) = handle.read_line(&mut input_idx) {
                        println!("Error reading input {}", e);
                    }
                    println!("w{}w", input_idx);
                    let src_index: usize = match input_idx.lines().next().unwrap().parse::<i32>() {
                        Ok(val) => val as usize,
                        Err(_) => {
                            println!("Invalid option");
                            continue;
                        },
                    };
                    input_idx = String::new();
                    println!("Enter another Number: ");
                    if let Err(e) = handle.read_line(&mut input_idx) {
                        println!("Error reading input {}", e);
                    }
                    println!("w{}w", input_idx);
                    let dst_index: usize = match input_idx.lines().next().unwrap().parse::<i32>() {
                        Ok(val) => val as usize,
                        Err(_) => {
                            println!("Invalid option");
                            continue;
                        },
                    };
                    graph.add_edge_between_nodes(src_index, dst_index);
                    break;
                }
            },
            "3" => {
                let (img_width, img_height): (i32,i32) = (200, 200);
                let mut img: image::ImageBuffer<Luma<u8>, Vec<u8>> = GrayImage::new(img_width as u32,img_height as u32);
                draw_graph(&graph, &mut img);
                if let Err(e) = img.save("Graph.png") {
                    panic!("Error Saving Image {}", e);
                }
                println!("{:?}", graph);
            },
            "q"|"quit"|"exit" => break,
            _ => println!("Invalid Option"),
        }
    }
}
