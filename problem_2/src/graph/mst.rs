use graph::Node;
use min_heap::MinHeap;
use std::collections::HashSet;

pub fn compute_prim_mst(_graph_nodes: &Vec<Node>) -> Vec<Node> {
    let numb_vertices = _graph_nodes.len();
    let mut min_span_tree: Vec<Node> = Vec::with_capacity(numb_vertices);
    let mut min_heap: MinHeap = MinHeap::with_capacity(numb_vertices);
    let mut weights: Vec<u32> = Vec::with_capacity(numb_vertices);
    /*Initialize all data structures*/
    let mut placeholder: Node = Node {
        node_id: 0,
        neighbors: HashSet::new(),
    };
    min_span_tree.push(placeholder.clone());
    min_heap.push((0, 0));
    weights.push(0);
    for i in 1..numb_vertices {
        placeholder.node_id = i as u32;
        min_span_tree.push(placeholder.clone());
        min_heap.push((i as u32, std::u32::MAX));
        weights.push(std::u32::MAX);
    }
    /*Main loop*/
    while !min_heap.is_empty() {
        let current_node_id: u32 = min_heap.pop().unwrap();
        let current_node: &Node = &_graph_nodes[current_node_id as usize];
        /*Query all neighbors (node: u32, edge_weight: u32)*/
        for neighbor in current_node.neighbors.iter() {
            if min_heap.contains(neighbor.0) && weights[neighbor.0 as usize] > neighbor.1 {
                weights[neighbor.0 as usize] = neighbor.1;
                min_heap.modify(neighbor.0, neighbor.1);
                min_span_tree[neighbor.0 as usize].neighbors.clear();
                min_span_tree[neighbor.0 as usize]
                    .neighbors
                    .insert((current_node_id, neighbor.1));
            }
        }
    }
    min_span_tree
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_tree_mst() {
        let mut _graph_nodes: Vec<Node> = Vec::new();
        let mut node_0 = Node {
            node_id: 0,
            neighbors: HashSet::new(),
        };
        node_0.neighbors.insert((1, 4));
        node_0.neighbors.insert((7, 8));
        let mut node_1 = Node {
            node_id: 1,
            neighbors: HashSet::new(),
        };
        node_1.neighbors.insert((7, 11));
        node_1.neighbors.insert((0, 4));
        node_1.neighbors.insert((2, 8));
        let mut node_2 = Node {
            node_id: 2,
            neighbors: HashSet::new(),
        };
        node_2.neighbors.insert((1, 8));
        node_2.neighbors.insert((8, 2));
        node_2.neighbors.insert((5, 4));
        node_2.neighbors.insert((3, 7));
        let mut node_3 = Node {
            node_id: 3,
            neighbors: HashSet::new(),
        };
        node_3.neighbors.insert((2, 7));
        node_3.neighbors.insert((5, 14));
        node_3.neighbors.insert((4, 9));
        let mut node_4 = Node {
            node_id: 4,
            neighbors: HashSet::new(),
        };
        node_4.neighbors.insert((3, 9));
        node_4.neighbors.insert((5, 10));
        let mut node_5 = Node {
            node_id: 5,
            neighbors: HashSet::new(),
        };
        node_5.neighbors.insert((2, 4));
        node_5.neighbors.insert((3, 14));
        node_5.neighbors.insert((4, 10));
        node_5.neighbors.insert((6, 2));
        let mut node_6 = Node {
            node_id: 6,
            neighbors: HashSet::new(),
        };
        node_6.neighbors.insert((5, 2));
        node_6.neighbors.insert((8, 6));
        node_6.neighbors.insert((7, 1));
        let mut node_7 = Node {
            node_id: 7,
            neighbors: HashSet::new(),
        };
        node_7.neighbors.insert((0, 8));
        node_7.neighbors.insert((1, 11));
        node_7.neighbors.insert((6, 1));
        node_7.neighbors.insert((8, 7));
        let mut node_8 = Node {
            node_id: 8,
            neighbors: HashSet::new(),
        };
        node_8.neighbors.insert((2, 2));
        node_8.neighbors.insert((7, 7));
        node_8.neighbors.insert((6, 6));
        _graph_nodes.push(node_0);
        _graph_nodes.push(node_1);
        _graph_nodes.push(node_2);
        _graph_nodes.push(node_3);
        _graph_nodes.push(node_4);
        _graph_nodes.push(node_5);
        _graph_nodes.push(node_6);
        _graph_nodes.push(node_7);
        _graph_nodes.push(node_8);
        print_graph(&_graph_nodes);
        let _mst = compute_prim_mst(&_graph_nodes);
        print_graph(&_mst);
    }

    #[allow(dead_code)]
    fn print_graph(_graph_nodes: &Vec<Node>) {
        println!("{:#?}", _graph_nodes);
    }
}
