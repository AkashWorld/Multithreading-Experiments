extern crate rand;
use self::rand::Rng;
use std::collections::HashSet;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Write;

pub mod mst;

pub struct Node {
    node_id: u32,
    connections: HashSet<(u32, u32)>,
}

impl Node {
    fn add_connection(&mut self, neighbor: u32, edge_weight: u32) {
        if neighbor != self.node_id && edge_weight > 0 {
            if !self.connections.contains(&(neighbor, edge_weight)) {
                self.connections.insert((neighbor, edge_weight));
            }
        }
    }
}

pub fn generate_graph(graph_size: u32) -> Vec<Node> {
    println!("Generating graph of size {}", graph_size);
    let mut graph_nodes: Vec<Node> = Vec::new();
    for i in 0..graph_size {
        let _numb_of_connections: u32 = rand::thread_rng().gen_range(1, graph_size);
        let mut _new_node: Node = Node {
            node_id: i,
            connections: HashSet::new(),
        };
        for _j in 0.._numb_of_connections {
            let _new_length: u32 = rand::thread_rng().gen_range(1, graph_size * 100);
            let _new_connection: u32 = rand::thread_rng().gen_range(1, graph_size);
            _new_node.add_connection(_new_connection, _new_length);
        }
        graph_nodes.push(_new_node);
    }
    graph_nodes
}

#[allow(dead_code)]
pub fn print_graph(_graph_nodes: &Vec<Node>) {
    for node in _graph_nodes.iter() {
        for elem in node.connections.iter() {
            println!("{}, Neighbor {}  Length {}", node.node_id, elem.0, elem.1);
        }
    }
}

pub fn output_graph_to_file(_graph_nodes: &Vec<Node>, ref filename: &String) {
    let mut file = match File::open(filename) {
        Ok(ret_file) => ret_file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(new_file) => new_file,
                Err(err) => panic!("Error creating new file {}, {:?}", filename, err),
            },
            other_error => panic!("There was a problem opening the file {:?}", other_error),
        },
    };
    for node in _graph_nodes.iter() {
        for elem in node.connections.iter() {
            let _new_line = format!("{}-{}-{}\n", node.node_id, elem.0, elem.1);
            let _length = match file.write(_new_line.as_bytes()) {
                Ok(len) => len,
                Err(_) => continue,
            };
        }
    }
}
