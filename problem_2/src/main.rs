extern crate time;
use time::{Duration, PreciseTime};
pub mod graph;
pub mod min_heap;

fn profile_sequential_mst(count: usize) -> Duration {
    let graph_nodes: Vec<graph::Node> = graph::generate_graph(count);
    let start = PreciseTime::now();
    let _mst = graph::mst::compute_prim_mst(&graph_nodes);
    let end = PreciseTime::now();
    println!("Sequential MST of {} nodes took {} s", count, start.to(end));
    start.to(end)
}

fn main() {
    profile_sequential_mst(1000);
    profile_sequential_mst(2000);
    profile_sequential_mst(4000);
    profile_sequential_mst(8000);
    profile_sequential_mst(16000);
}
