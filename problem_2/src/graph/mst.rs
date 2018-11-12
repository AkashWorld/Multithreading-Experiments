/*
MST Sequential and Threaded
MIT License
Author: Khalid Akash, 2018
*/
use graph::Node;
use graph::NodeEdge;
use min_heap::MinHeap;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::thread;

pub fn compute_sequential_mst(graph_nodes_rc: Rc<Vec<Node>>) -> Vec<Node> {
    let numb_vertices = graph_nodes_rc.len();
    let mut min_span_tree: Vec<Node> = Vec::with_capacity(numb_vertices);
    let mut min_heap: MinHeap = MinHeap::with_capacity(numb_vertices);
    let mut weights: Vec<u32> = Vec::with_capacity(numb_vertices);
    /*Initialize all data structures*/
    let mut placeholder: Node = Node {
        node_id: 0,
        neighbors: Vec::new(),
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
        let current_node: &Node = &graph_nodes_rc[current_node_id as usize];
        /*Query all neighbors (node: u32, edge_weight: u32)*/
        for neighbor in current_node.neighbors.iter() {
            if min_heap.contains(neighbor.node) && weights[neighbor.node as usize] > neighbor.edge {
                weights[neighbor.node as usize] = neighbor.edge;
                min_heap.decrease_key(neighbor.node, neighbor.edge);
                min_span_tree[neighbor.node as usize].neighbors.clear();
                min_span_tree[neighbor.node as usize]
                    .neighbors
                    .push(NodeEdge {
                        node: current_node_id,
                        edge: neighbor.edge,
                    });
            }
        }
    }
    min_span_tree
}

pub fn compute_parallel_mst(graph_nodes: Arc<Vec<Node>>, thread_count: usize) -> Vec<Node> {
    let numb_vertices = graph_nodes.len();
    let mut min_span_tree: Vec<Node> = Vec::with_capacity(numb_vertices);
    let mut min_heap: MinHeap = MinHeap::with_capacity(numb_vertices);
    let mut weights: Vec<u32> = Vec::with_capacity(numb_vertices);
    /*Initialize all data structures*/
    let mut placeholder: Node = Node {
        node_id: 0,
        neighbors: Vec::new(),
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
    let mut current_node_id: u32 = match min_heap.pop() {
        Some(val) => val,
        None => return min_span_tree,
    };
    /*Create atomic reference counts to allow multiple variable
    ownerships between threads. Performance hit due to forced compiler opt disabled*/
    let min_heap_arc = Arc::new(RwLock::new(min_heap));
    let weight_arc = Arc::new(RwLock::new(weights));
    let mst_arc = Arc::new(RwLock::new(min_span_tree));
    loop {
        let mut curr_thread_count = thread_count;
        let current_node: &Node = &graph_nodes[current_node_id as usize];
        let neighbors_count: usize = current_node.neighbors.len();
        let partition = if neighbors_count / curr_thread_count != 0 {
            (neighbors_count + (curr_thread_count - 1)) / curr_thread_count
        } else {
            curr_thread_count = 1;
            neighbors_count
        };
        let mut thread_handles: Vec<thread::JoinHandle<()>> = Vec::new();
        for thread_idx in 0..curr_thread_count {
            let thread_heap_arc = Arc::clone(&min_heap_arc);
            let thread_weight_arc = Arc::clone(&weight_arc);
            let thread_mst_arc = Arc::clone(&mst_arc);
            let thread_graph_arc = Arc::clone(&graph_nodes);
            /*Thread creation*/
            let thread_handle = thread::spawn(move || {
                let current_neighbors = &thread_graph_arc[current_node_id as usize].neighbors;
                let start = thread_idx * partition;
                let mut end = start + partition;
                if end > current_neighbors.len() {
                    end = current_neighbors.len();
                }
                for curr_neighbor in current_neighbors.iter().take(end).skip(start) {
                    if thread_heap_arc.read().unwrap().contains(curr_neighbor.node)
                        && thread_weight_arc.read().unwrap()[curr_neighbor.node as usize]
                            > curr_neighbor.edge
                    {
                        {
                            thread_weight_arc.write().unwrap()[curr_neighbor.node as usize] =
                                curr_neighbor.edge;
                        }
                        {
                            thread_heap_arc
                                .write()
                                .unwrap()
                                .decrease_key(curr_neighbor.node, curr_neighbor.edge);
                        }
                        {
                            let mut mst_w = thread_mst_arc.write().unwrap();
                            mst_w[curr_neighbor.node as usize].neighbors.clear();
                            mst_w[curr_neighbor.node as usize].neighbors.push({
                                NodeEdge {
                                    node: current_node_id,
                                    edge: curr_neighbor.edge,
                                }
                            })
                        }
                    }
                }
            });
            thread_handles.push(thread_handle);
        }
        for elem in thread_handles {
            elem.join().unwrap();
        }
        let mut locked_min_heap = min_heap_arc.write().unwrap();
        current_node_id = match locked_min_heap.pop() {
            Some(val) => val,
            None => break, /*Heap is empty, return break main loop*/
        };
    }
    /*Remove Atomic ref count and Mutex wrappers to return the final MST*/
    //Mutex::into_inner(Arc::try_unwrap(mst_arc).unwrap()).unwrap()
    Arc::try_unwrap(mst_arc).unwrap().into_inner().unwrap()
}

/*Tests*/
#[cfg(test)]
mod tests {
    use super::*;
    use graph;
    fn graph_1_generator() -> Vec<Node> {
        let mut _graph_nodes: Vec<Node> = Vec::new();
        let mut node_0 = Node {
            node_id: 0,
            neighbors: Vec::new(),
        };
        node_0.neighbors.push(NodeEdge { node: 1, edge: 4 });
        node_0.neighbors.push(NodeEdge { node: 7, edge: 8 });
        let mut node_1 = Node {
            node_id: 1,
            neighbors: Vec::new(),
        };
        node_1.neighbors.push(NodeEdge { node: 7, edge: 1 });
        node_1.neighbors.push(NodeEdge { node: 0, edge: 4 });
        node_1.neighbors.push(NodeEdge { node: 2, edge: 8 });
        let mut node_2 = Node {
            node_id: 2,
            neighbors: Vec::new(),
        };
        node_2.neighbors.push(NodeEdge { node: 1, edge: 8 });
        node_2.neighbors.push(NodeEdge { node: 8, edge: 2 });
        node_2.neighbors.push(NodeEdge { node: 5, edge: 4 });
        node_2.neighbors.push(NodeEdge { node: 3, edge: 7 });
        let mut node_3 = Node {
            node_id: 3,
            neighbors: Vec::new(),
        };
        node_3.neighbors.push(NodeEdge { node: 2, edge: 7 });
        node_3.neighbors.push(NodeEdge { node: 5, edge: 1 });
        node_3.neighbors.push(NodeEdge { node: 4, edge: 9 });
        let mut node_4 = Node {
            node_id: 4,
            neighbors: Vec::new(),
        };
        node_4.neighbors.push(NodeEdge { node: 3, edge: 9 });
        node_4.neighbors.push(NodeEdge { node: 5, edge: 1 });
        let mut node_5 = Node {
            node_id: 5,
            neighbors: Vec::new(),
        };
        node_5.neighbors.push(NodeEdge { node: 2, edge: 4 });
        node_5.neighbors.push(NodeEdge { node: 3, edge: 1 });
        node_5.neighbors.push(NodeEdge { node: 4, edge: 1 });
        node_5.neighbors.push(NodeEdge { node: 6, edge: 2 });
        let mut node_6 = Node {
            node_id: 6,
            neighbors: Vec::new(),
        };
        node_6.neighbors.push(NodeEdge { node: 5, edge: 2 });
        node_6.neighbors.push(NodeEdge { node: 8, edge: 6 });
        node_6.neighbors.push(NodeEdge { node: 7, edge: 1 });
        let mut node_7 = Node {
            node_id: 7,
            neighbors: Vec::new(),
        };
        node_7.neighbors.push(NodeEdge { node: 0, edge: 8 });
        node_7.neighbors.push(NodeEdge { node: 1, edge: 1 });
        node_7.neighbors.push(NodeEdge { node: 6, edge: 1 });
        node_7.neighbors.push(NodeEdge { node: 8, edge: 7 });
        let mut node_8 = Node {
            node_id: 8,
            neighbors: Vec::new(),
        };
        node_8.neighbors.push(NodeEdge { node: 2, edge: 2 });
        node_8.neighbors.push(NodeEdge { node: 7, edge: 7 });
        node_8.neighbors.push(NodeEdge { node: 6, edge: 6 });
        _graph_nodes.push(node_0);
        _graph_nodes.push(node_1);
        _graph_nodes.push(node_2);
        _graph_nodes.push(node_3);
        _graph_nodes.push(node_4);
        _graph_nodes.push(node_5);
        _graph_nodes.push(node_6);
        _graph_nodes.push(node_7);
        _graph_nodes.push(node_8);
        _graph_nodes
    }
    fn graph_2_generator() -> Vec<Node> {
        let mut _graph_nodes = Vec::<Node>::new();
        let mut node_a = Node {
            node_id: 0,
            neighbors: Vec::new(),
        };
        node_a.neighbors.push(NodeEdge { node: 1, edge: 5 });
        node_a.neighbors.push(NodeEdge { node: 2, edge: 1 });
        node_a.neighbors.push(NodeEdge { node: 3, edge: 4 });
        let mut node_b = Node {
            node_id: 2,
            neighbors: Vec::new(),
        };
        node_b.neighbors.push(NodeEdge { node: 0, edge: 5 });
        node_b.neighbors.push(NodeEdge { node: 6, edge: 6 });
        let mut node_c = Node {
            node_id: 3,
            neighbors: Vec::new(),
        };
        node_c.neighbors.push(NodeEdge { node: 0, edge: 1 });
        node_c.neighbors.push(NodeEdge { node: 5, edge: 2 });
        node_c.neighbors.push(NodeEdge { node: 4, edge: 3 });
        let mut node_d = Node {
            node_id: 4,
            neighbors: Vec::new(),
        };
        node_d.neighbors.push(NodeEdge { node: 0, edge: 4 });
        node_d.neighbors.push(NodeEdge { node: 2, edge: 3 });
        node_d.neighbors.push(NodeEdge { node: 6, edge: 8 });
        let mut node_e = Node {
            node_id: 5,
            neighbors: Vec::new(),
        };
        node_e.neighbors.push(NodeEdge { node: 2, edge: 2 });
        node_e.neighbors.push(NodeEdge { node: 6, edge: 7 });
        node_e.neighbors.push(NodeEdge { node: 7, edge: 9 });
        let mut node_f = Node {
            node_id: 6,
            neighbors: Vec::new(),
        };
        node_e.neighbors.push(NodeEdge { node: 5, edge: 7 });
        node_e.neighbors.push(NodeEdge { node: 4, edge: 8 });
        node_e.neighbors.push(NodeEdge { node: 1, edge: 6 });
        let mut node_g = Node {
            node_id: 7,
            neighbors: Vec::new(),
        };
        node_g.neighbors.push(NodeEdge { node: 5, edge: 9 });
        _graph_nodes.push(node_a);
        _graph_nodes.push(node_b);
        _graph_nodes.push(node_c);
        _graph_nodes.push(node_d);
        _graph_nodes.push(node_e);
        _graph_nodes.push(node_f);
        _graph_nodes.push(node_g);
        _graph_nodes
    }
    #[test]
    fn sequential_mst() {
        let _graph_nodes = graph_1_generator();
        print_graph(&_graph_nodes);
        let rc_graph_nodes = Rc::new(_graph_nodes);
        let _mst = compute_sequential_mst(rc_graph_nodes);
        print_graph(&_mst);
    }
    #[test]
    fn parallel_mst() {
        let _graph_nodes = graph_1_generator();
        let arc_graph_nodes = Arc::new(_graph_nodes);
        let _mst_parallel = compute_parallel_mst(Arc::clone(&arc_graph_nodes), 2);
        let rc_graph_nodes = Rc::new(Arc::try_unwrap(arc_graph_nodes).unwrap());
        let _mst_seq = compute_sequential_mst(rc_graph_nodes);
        assert_eq!(_mst_parallel.len(), _mst_seq.len());
        for i in 0.._mst_parallel.len() {
            assert!(_mst_parallel[i].is_equal(&_mst_seq[i]));
        }

        let _graph_nodes = graph_2_generator();
        let arc_graph_nodes = Arc::new(_graph_nodes);
        let _mst_parallel = compute_parallel_mst(Arc::clone(&arc_graph_nodes), 2);
        let rc_graph_nodes = Rc::new(Arc::try_unwrap(arc_graph_nodes).unwrap());
        let _mst_seq = compute_sequential_mst(rc_graph_nodes);
        println!("Seq: {:#?}", _mst_seq);
        println!("Parallel: {:#?}", _mst_parallel);
        assert_eq!(_mst_parallel.len(), _mst_seq.len());
        for i in 0.._mst_parallel.len() {
            assert!(_mst_parallel[i].is_equal(&_mst_seq[i]));
        }
    }
}
