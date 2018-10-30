extern crate rand;
use self::rand::Rng;
use graph::Node;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct NodeEdgePair {
    node: u32,
    edge: u32,
}

/*Defining comparitors for MinHeap (BinaryHeap)*/
impl Ord for NodeEdgePair {
    fn cmp(&self, other: &NodeEdgePair) -> Ordering {
        other.edge.cmp(&self.edge)
    }
}

impl PartialOrd for NodeEdgePair {
    fn partial_cmp(&self, other: &NodeEdgePair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn compute_prim_mst(_graph_nodes: &Vec<Node>) {
    let mut min_span_tree: Vec<Node> = Vec::with_capacity(_graph_nodes.len());
    let mut map: HashMap<u32, u32> = HashMap::new();
    let mut min_heap: BinaryHeap<NodeEdgePair> = BinaryHeap::new();

    /*Fill helper data structures*/
    for (_i, elem) in _graph_nodes.iter().enumerate() {
        let mut new_pair: NodeEdgePair = NodeEdgePair {
            node: elem.node_id,
            edge: std::u32::MAX,
        };
        if _i == 0 {
            new_pair.edge = 0;
        }
        map.insert(elem.node_id, std::u32::MAX);
        min_heap.push(new_pair);
    }
    /*Main loop*/
    while min_span_tree.len() != _graph_nodes.len() {
        let min_node: NodeEdgePair = match min_heap.pop() {
            Some(min) => min,
            None => break,
        };
        let _curr_min_node: &Node = &_graph_nodes[min_node.node as usize];
        for elem in _curr_min_node.connections.iter() {
            /*
                1 - Query all neighbors of this current node
                2 - Compare value of all edges to the current
                    edges stored in the HashMap
                3 - if value of edge is less than current edge, replace in HashMap
                4 - 

        */
        }
    }
}
