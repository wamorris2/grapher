/*struct PathNode<'a> {
    name: String,
    x: f32,
    y: f32,
    z: f32,
    edges: Vec<&'a Edge<'a>>,
    parent: &'a dyn Node,
    in_open_set: bool,
    in_closed_set: bool,
    cost: f32,
    heuristic: f32,
}

impl PartialEq for PathNode<'_> {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl PartialOrd for PathNode<'_> {
    
    fn partial_cmp(&self, other: &PathNode<'_>) -> Option<Ordering> {
        if self.cost > other.cost {
            Some(Ordering::Greater)
        } else if self.cost < other.cost {
            Some(Ordering::Less)
        } else if self.heuristic > other.heuristic {
            Some(Ordering::Greater)
        } else if self.heuristic < other.heuristic  {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}*/

fn main() {
    grapher::run();
}
