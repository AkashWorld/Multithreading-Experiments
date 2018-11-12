/*
MST Graph
MIT License
Author: Khalid Akash, 2018
*/
extern crate colored;
extern crate rand;
use self::colored::*;
use self::rand::Rng;
use std::collections::HashSet;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Write;
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
pub mod mst;

#[derive(Clone, Debug)]
pub struct NodeEdge {
    node: u32,
    edge: u32,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub node_id: u32,
    pub neighbors: Vec<NodeEdge>,
}

impl Node {
    fn add_connection(&mut self, neighbor: u32, edge_weight: u32) -> bool {
        if neighbor != self.node_id {
            self.neighbors.push(NodeEdge {
                node: neighbor,
                edge: edge_weight,
            });
            return true;
        }
        false
    }
    fn is_equal(&self, other: &Node) -> bool {
        if self.node_id != other.node_id {
            return false;
        }
        if self.neighbors.len() != other.neighbors.len() {
            return false;
        }
        for i in 0..self.neighbors.len() {
            if self.neighbors[i].node != other.neighbors[i].node
                || self.neighbors[i].edge != other.neighbors[i].edge
            {
                return false;
            }
        }
        true
    }
}

pub fn generate_graph(graph_size: usize, mut thread_count: usize) -> Option<Vec<Node>> {
    let prompt = format!(
        "========Generating graph of size {} with {} threads========",
        graph_size, thread_count
    ).purple()
    .bold();
    println!("{}", prompt);
    let graph_nodes: Vec<Node> = Vec::with_capacity(graph_size);
    let graph_nodes_arc = Arc::new(RwLock::new(graph_nodes));
    let cond_var = Arc::new((Mutex::new(0), Condvar::new()));
    let mut thread_handles: Vec<thread::JoinHandle<()>> = Vec::with_capacity(thread_count);
    let partition = if graph_size/thread_count > 0 {
        (graph_size + (thread_count - 1)) / thread_count
    } else {
        thread_count = 1;
        graph_size
    };
    for thread_idx in 0..thread_count {
        let mut graph_clone = Arc::clone(&graph_nodes_arc);
        let mut cond_var_clone = Arc::clone(&cond_var);
        let handle = thread::spawn(move || {
            let mut thread_local_graph: Vec<Node> = Vec::with_capacity(partition);
            for i in 0..partition {
                if thread_idx*partition + i >= graph_size {
                    break;
                }
                let _numb_of_neighbors: u32 = rand::thread_rng().gen_range(1, graph_size as u32);
                let mut connected_neighbors: HashSet<u32> = HashSet::new();
                let mut _new_node: Node = Node {
                    node_id: i as u32 + (partition as u32) * (thread_idx as u32),
                    neighbors: Vec::with_capacity(_numb_of_neighbors as usize),
                };
                for _j in 0.._numb_of_neighbors {
                    let _new_length: u32 = rand::thread_rng().gen_range(1, std::u32::MAX as u32);
                    let mut new_neighbor: u32 = rand::thread_rng().gen_range(1, graph_size as u32);
                    if !connected_neighbors.contains(&new_neighbor) {
                        while !_new_node.add_connection(new_neighbor, _new_length) {
                            new_neighbor = rand::thread_rng().gen_range(1, graph_size as u32);
                        }
                        connected_neighbors.insert(new_neighbor);
                    }
                }
                thread_local_graph.push(_new_node);
            }
            /*Condition variable is here to ensure that the right vector
            is appended to the final graph correctly.*/
            let &(ref lock, ref cvar) = &*cond_var_clone;
            let mut condition = lock.lock().unwrap();
            while *condition != thread_idx * partition {
                condition = cvar.wait(condition).unwrap();
            }
            graph_clone.write().unwrap().append(&mut thread_local_graph);
            *condition = thread_idx * partition + partition;
            cvar.notify_all();
        });
        thread_handles.push(handle);
    }
    for elem in thread_handles {
        elem.join().unwrap();
    }
    Option::Some(
        Arc::try_unwrap(graph_nodes_arc)
            .unwrap()
            .into_inner()
            .unwrap(),
    )
}

pub fn output_graph_to_file(_graph_nodes: &Vec<Node>, filename: &str) {
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
        for elem in node.neighbors.iter() {
            let _new_line = format!("{}-{}-{}\n", node.node_id, elem.node, elem.edge);
            let _length = match file.write(_new_line.as_bytes()) {
                Ok(len) => len,
                Err(_) => continue,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_graph_test() {
        let mut test_graph = generate_graph(2000, 4).unwrap();
        assert_eq!(test_graph.len(), 2000);
        for (i, node) in test_graph.iter().enumerate() {
            assert_eq!(i as u32, node.node_id);
        }
        let mut test_graph = generate_graph(2000, 7).unwrap();
        assert_eq!(test_graph.len(), 2000);
        for (i, node) in test_graph.iter().enumerate() {
            assert_eq!(i as u32, node.node_id);
        }
        let mut test_graph = generate_graph(2000, 3).unwrap();
        assert_eq!(test_graph.len(), 2000);
        for (i, node) in test_graph.iter().enumerate() {
            assert_eq!(i as u32, node.node_id);
        }
    }
}
