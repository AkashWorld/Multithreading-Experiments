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

fn profile_sequential_mst(graph_nodes: Rc<Vec<graph::Node>>, prev_timing: &i64) -> i64 {
    let cloned_reference_graph_nodes = Rc::clone(&graph_nodes);
    let start = PreciseTime::now();
    let _mst = graph::mst::compute_sequential_mst(cloned_reference_graph_nodes);
    let end = PreciseTime::now();
    let mut edge_count: usize = 0;
    for elem in graph_nodes.iter() {
        edge_count = edge_count + elem.neighbors.len();
    }
    let operation_type = "[Sequential]".yellow().bold();
    println!(
        "{} MST of {} nodes and {} edges took {} {}",
        operation_type,
        graph_nodes.len(),
        edge_count,
        format!("{}", start.to(end).num_milliseconds()).yellow(),
        "ms".green()
    );
    start.to(end).num_milliseconds()
}

fn profile_parallel_mst(graph_nodes: Arc<Vec<graph::Node>>, thread_count: usize, prev_timing: i64) -> i64 {
    let cloned_reference_graph_nodes = Arc::clone(&graph_nodes);
    let start = PreciseTime::now();
    let _mst = graph::mst::compute_parallel_mst(cloned_reference_graph_nodes, thread_count);
    let end = PreciseTime::now();
    let mut edge_count = 0;
    for elem in graph_nodes.iter() {
        edge_count += elem.neighbors.len();
    }
    let operation_type = format!("[{} Thread(s)]", thread_count).yellow().bold();
    let mut timing_format = format!("{} ms", start.to(end).num_milliseconds());
    if start.to(end).num_milliseconds() > prev_timing {
        timing_format = timing_format.red().to_string();
    } else {
        timing_format = timing_format.green().to_string();
    }
    println!(
        "{} Parallel MST of {} nodes and {} edges took {}",
        operation_type,
        graph_nodes.len(),
        edge_count,
        format!("{}", timing_format)
    );
    start.to(end).num_milliseconds()
}

fn run_tests(graph_nodes: Vec<graph::Node>) {
    let graph_nodes_rc = Rc::new(graph_nodes);
    let graph_nodes_rc_clone = Rc::clone(&graph_nodes_rc);
    let mut timing = profile_sequential_mst(graph_nodes_rc_clone, &0);
    let graph_nodes_arc_0 = Arc::new(Rc::try_unwrap(graph_nodes_rc).unwrap());
    let graph_nodes_arc_1 = Arc::clone(&graph_nodes_arc_0);
    timing = profile_parallel_mst(graph_nodes_arc_1, 1, timing);
    let graph_nodes_arc_2 = Arc::clone(&graph_nodes_arc_0);
    timing = profile_parallel_mst(graph_nodes_arc_2, 2, timing);
    let graph_nodes_arc_3 = Arc::clone(&graph_nodes_arc_0);
    timing = profile_parallel_mst(graph_nodes_arc_3, 4, timing);
    let graph_nodes_arc_4 = Arc::clone(&graph_nodes_arc_0);
    timing = profile_parallel_mst(graph_nodes_arc_4, 8, timing);
    let graph_nodes_arc_5 = Arc::clone(&graph_nodes_arc_0);
    timing = profile_parallel_mst(graph_nodes_arc_5, 16, timing);
}

fn main() {
    println!("{}", "This can take some time...".red().bold());
    let mut thread_count: usize = 4;
    let mut graph_size: usize = 32000;
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let arg_1 = &args[1];
        thread_count = match arg_1.parse::<usize>() {
            Ok(var) => var,
            Err(_) => thread_count,
        };
    } else if args.len() == 3 {
        let arg_1 = &args[1];
        thread_count = match arg_1.parse::<usize>() {
            Ok(var) => var,
            Err(_) => thread_count,
        };
        let arg_2 = &args[2];
        graph_size = match arg_2.parse::<usize>() {
            Ok(var) => var,
            Err(_) => graph_size,
        }
    }
    println!(
        "{}",
        format!(
            "Setting maximum thread count to {} and graph size to {}",
            thread_count, graph_size
        ).blue()
        .bold()
    );
    let mut curr_graph_size: usize = 1000;
    loop {
        run_tests(graph::generate_graph(curr_graph_size, thread_count).unwrap());
        if curr_graph_size == graph_size {
            break;
        }
        curr_graph_size = curr_graph_size * 2;
        if curr_graph_size > graph_size {
            curr_graph_size = graph_size;
        }
    }
}
