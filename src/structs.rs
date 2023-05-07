use std::rc::Rc;
use std::cell::RefCell;


pub struct Edge<'a> {
    pub weight: f32,
    pub dest: Rc<RefCell<&'a mut GraphNode<'a>>>,
}

impl<'a> Edge<'a> {
    pub fn new(weight: f32, dest: &Rc<RefCell<&'a mut GraphNode<'a>>>) -> Self {
        Edge {
            weight: weight,
            dest: Rc::clone(dest),
        }
    }
}

pub struct UndirectedEdge<'a> {
    std: Edge<'a>,
    dts: Edge<'a>,
}

pub struct GraphNode<'a> {
    pub name: String,
    pub pos: Point,
    pub edges: Vec<Edge<'a>>,
}

impl<'a> GraphNode<'a>{
    pub fn new(name: String, x: f32, y: f32, z: f32) -> Self {
        GraphNode{
            name: name,
            pos: Point{x:x, y:y, z:z},
            edges: vec![],
        }
    }

    pub fn move_node(&mut self, x: f32, y: f32, z: f32) {
        self.pos.x = x;
        self.pos.y = y;
        self.pos.z = z;
    }

    pub fn add_edge(&mut self, edge: Edge<'a>) {
        self.edges.push(edge);
    }
}

impl std::fmt::Debug for GraphNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphNode")
            .field("name", &self.name)
            .field("x", &self.pos.x)
            .field("y", &self.pos.y)
            .field("z", &self.pos.z)
            .finish()
    }
}

impl std::cmp::PartialEq for GraphNode<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
            self.pos.x == other.pos.x && 
            self.pos.y == other.pos.y &&
            self.pos.z == other.pos.z
    }
}

pub struct Graph<'a> {
    pub nodes: Vec<Rc<RefCell<&'a mut GraphNode<'a>>>>,
    pub edges: Vec<Edge<'a>>,
}

impl<'a> Graph<'a> {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn build(nodes: Vec<&'a mut GraphNode<'a>>, edges: Vec<Edge<'a>>) -> Self{
        let nodes: Vec<Rc<RefCell<&mut GraphNode<'a>>>> = nodes.into_iter().map(|n| Rc::new(RefCell::new(n))).collect();
        Graph {
            nodes: nodes,
            edges: edges,
        }
    }

    pub fn add_node(&mut self, node: &'a mut GraphNode<'a>) {
        self.nodes.push(Rc::new(RefCell::new(node)));
    }

    fn add_edge_between_points(&mut self, src_point: &Point, dst_point: &Point) {
        let src = self.nearest_node(src_point, None).expect("Error finding source node");
        let dst = self.nearest_node(dst_point, None).expect("Error finding destination node");
        let weight = dist(&src.borrow().pos, &dst.borrow().pos);
        src.borrow_mut().add_edge(Edge::new(weight, &dst));
        dst.borrow_mut().add_edge(Edge::new(weight, &src));
    }

    fn add_edge_between_nodes(&mut self, src_node: &Rc<RefCell<&'a mut GraphNode<'a>>>, dst_node: &Rc<RefCell<&'a mut GraphNode<'a>>>) {
        let weight = dist(&src_node.borrow().pos, &dst_node.borrow().pos);
        src_node.borrow_mut().add_edge(Edge::new(weight, &dst_node));
        dst_node.borrow_mut().add_edge(Edge::new(weight, &src_node));
    }

    fn nearest_node(&self, p: &Point, threshold: Option<f32>) -> Option<&Rc<RefCell<&'a mut GraphNode<'a>>>> {
        let threshold = threshold.unwrap_or(std::f32::EPSILON);
        for node in &self.nodes {
            if dist(&node.borrow().pos, p) < threshold {
                return Some(&node);
            }
        }
        None
    }

    pub fn list_nodes(&self) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        for node in &self.nodes {
            let n = node.borrow();
            v.push(format!("{}: ({}, {}, {})", n.name, n.pos.x, n.pos.y, n.pos.z));
        }
        v
    }
}


#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn dist(p1: &Point, p2: &Point) -> f32 {
    f32::sqrt((p1.x-p2.x).powf(2.0)+(p1.y-p2.y).powf(2.0)+(p1.z+p2.z).powf(2.0))
}
