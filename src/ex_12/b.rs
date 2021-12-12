use super::super::files;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Hash)]
struct Edge {
    from: String,
    to: String,
    weight: u32,
}

type Node = String;
type EdgeMap = HashMap<Node, Vec<Node>>;
type Path = Vec<Node>;

// a function that checks if this node is the "end"
fn is_end(n: &Node) -> bool {
    n == "end"
}

// a function that checks if we can repeat a given node
fn is_repeatable(n: &Node, visited: &HashMap<Node, u32>) -> bool {
    if n == "start" && visited.contains_key(n) {
        return false;
    }

    // if we haven't seen this node before, it is repeatable
    if !visited.contains_key(n) {
        return true;
    }

    // if the node is a big cave, it is repeatable
    if n.to_uppercase() == *n {
        return true;
    }

    // if it is lowercase, and we have seen this before, it is only repeatable
    // if every lowercase key with a count smaller or equal than one
    for visited_node in visited.keys() {
        if visited_node.to_uppercase() == *visited_node {
            continue;
        }

        let count = visited.get(visited_node).unwrap();
        if count > &1 {
            return false
        }
    }

    true
}

//  a function that generates all paths from a given node to the end
fn traverse(node: &Node, node_map: &EdgeMap, visited: &mut HashMap<Node, u32>) -> Option<Vec<Path>> {
    // if we reached the end, just return the "end" node as the only possible path
    if is_end(node) {
        return Some(vec![vec![node.clone()]]);
    }

    // if we are not supposed to visit this node, skip
    if !is_repeatable(node, visited) {
        return None;
    }
    
    if visited.contains_key(node) {
        let value = visited.get(node).unwrap();
        visited.insert(node.clone(), value + 1);
    } else {
        visited.insert(node.clone(), 1);
    }

    // iterate over the adjacent nodes
    let mut paths:Vec<Path> = vec![];
    for next_node in node_map.get(node).unwrap().iter() {
        // otherwise generate all possible paths from the adjacent node to the "end".
        let next_paths = traverse(next_node, node_map, &mut visited.clone());
        if next_paths.is_none() {
            continue;
        }
        
        let next_paths: Vec<Path> = next_paths.unwrap().iter().map(|next_path| {
            let mut vector = vec![node.clone()];
            for next in next_path.iter() {
                vector.push(next.clone());
            }

            vector
        }).collect();

        // then concatenate all generated paths for this node.
        for next_path in next_paths {
            paths.push(next_path)
        }
    }

    Some(paths)
}

fn get_edges_map(edges: Vec<Edge>) -> EdgeMap {
    let mut node_map:EdgeMap = HashMap::new();

    // create empty vectors for each node
    for edge in edges.iter() {
        let Edge { from: node_a, to: node_b, .. } = edge;
        node_map.insert(node_a.to_string(), vec![]);
        node_map.insert(node_b.to_string(), vec![]);
    }

    // add to node_b into the vector for each node_a
    for edge in edges.iter() {
        let Edge { from: node_a, to: node_b, .. } = edge;
        let nodes = node_map.get_mut(node_a).unwrap();
        nodes.push(node_b.clone());
    }
    
    // add to node_a into the vector for each node_b
    for edge in edges.iter() {
        let Edge { from: node_a, to: node_b, .. } = edge;
        let nodes = node_map.get_mut(node_b).unwrap();
        nodes.push(node_a.clone());
    }

    node_map
}

pub fn run() {
    // first we read the raw lines from the input file
    let raw_lines = files::get_lines(String::from(
        "/Users/rama/Documents/adventofcode/2021/rust/src/ex_12/input.txt",
    ));

    // parse edges
    let edges: Vec<Edge> = raw_lines.iter().map(|line| {
        let (a, b) = line.split_once("-").unwrap();
        Edge {
            from: Node::from(a),
            to: Node::from(b),
            weight: 1,
        }
    }).collect();

    // generate all paths from start to end
    let mut visited: HashMap<Node, u32> = HashMap::new();
    let start_node = Node::from("start");
    let edges_map = get_edges_map(edges);
    let paths = traverse(&start_node, &edges_map, &mut visited).unwrap();
    
    println!("{}", paths.len());
}