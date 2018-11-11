/*
MST Benchmark
MIT License
Author: Khalid Akash, 2018
*/
extern crate time;
use std::env;
use std::rc::Rc;
use std::sync::Arc;
use time::{Duration, PreciseTime};
extern crate colored;
use colored::*;
pub mod graph;
pub mod min_heap;

fn profile_sequential_mst(graph_nodes: Rc<Vec<graph::Node>>) -> Duration {
    let cloned_reference_graph_nodes = Rc::clone(&graph_nodes);
    let start = PreciseTime::now();
    let _mst = graph::mst::compute_sequential_mst(cloned_reference_graph_nodes);
    let end = PreciseTime::now();
    let operation_type = "[Sequential]".yellow().bold();
    println!(
        "{} MST of {} nodes took {} {}",
        operation_type,
        graph_nodes.len(),
        format!("{}",start.to(end).num_milliseconds()).green(),
        "ms".green()
    );
    start.to(end)
}

fn profile_parallel_mst(graph_nodes: Arc<Vec<graph::Node>>, thread_count: usize) -> Duration {
    let cloned_reference_graph_nodes = Arc::clone(&graph_nodes);
    let start = PreciseTime::now();
    let _mst = graph::mst::compute_parallel_mst(cloned_reference_graph_nodes, thread_count);
    let end = PreciseTime::now();
    let operation_type = format!("[{} Thread(s)]", thread_count).yellow().bold();
    println!(
        "{} Parallel MST of {} nodes took {} {}",
        operation_type,
        graph_nodes.len(),
        format!("{}",start.to(end).num_milliseconds()).green(),
        "ms".green()
    );
    start.to(end)
}

fn run_tests(graph_nodes: Vec<graph::Node>) {
    let graph_nodes_rc = Rc::new(graph_nodes);
    let graph_nodes_rc_clone = Rc::clone(&graph_nodes_rc);
    profile_sequential_mst(graph_nodes_rc_clone);
    let graph_nodes_arc_0 = Arc::new(Rc::try_unwrap(graph_nodes_rc).unwrap());
    let graph_nodes_arc_1 = Arc::clone(&graph_nodes_arc_0);
    profile_parallel_mst(graph_nodes_arc_1, 1);
    let graph_nodes_arc_2 = Arc::clone(&graph_nodes_arc_0);
    profile_parallel_mst(graph_nodes_arc_2, 2);
    let graph_nodes_arc_3 = Arc::clone(&graph_nodes_arc_0);
    profile_parallel_mst(graph_nodes_arc_3, 4);
    let graph_nodes_arc_4 = Arc::clone(&graph_nodes_arc_0);
    profile_parallel_mst(graph_nodes_arc_4, 8);
    let graph_nodes_arc_5 = Arc::clone(&graph_nodes_arc_0);
    profile_parallel_mst(graph_nodes_arc_5, 16);
}

fn main() {
    println!("This can take some time...");
    let mut thread_count: usize = 4;
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let arg_1 = &args[1];
        thread_count = match arg_1.parse::<usize>() {
            Ok(var) => var,
            Err(_) => thread_count,
        };
    }
    run_tests(graph::generate_graph(1000, thread_count).unwrap());
    run_tests(graph::generate_graph(2000, thread_count).unwrap());
    run_tests(graph::generate_graph(4000, thread_count).unwrap());
    run_tests(graph::generate_graph(8000, thread_count).unwrap());
}