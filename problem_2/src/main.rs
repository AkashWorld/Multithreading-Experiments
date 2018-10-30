pub mod graph;
pub mod min_heap;
use min_heap::MinHeap;
fn main() {
    let graph_nodes: Vec<graph::Node> = graph::generate_graph(100);
    graph::mst::compute_prim_mst(&graph_nodes);
}
