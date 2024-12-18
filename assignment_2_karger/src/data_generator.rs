use crate::core::VESetGraph;
use crate::graph_builder::GraphBuilder;
use crate::min_cut::{contract, fast_cut, UnMulGraph};
use std::time::Instant;


// Run the given cut function n times and return the best result
pub fn best_cut<F, G>(graph: G, n: usize, cut_fn: F) -> usize
where
	G: UnMulGraph + Clone,
	F: Fn(G) -> usize,
{
	let mut best_result = usize::MAX;
	for _ in 0..n {
		let result = cut_fn(graph.clone());
		if result < best_result {
			best_result = result;
		}
	}
	best_result
}

pub fn generate_data<F>(
	num_vertices: Vec<usize>,
	edge_fn: F,
	csv_file_name: &str,
) where
	F: Fn(usize) -> usize,
{
	// Open a CSV file in write mode
	let mut wtr = csv::Writer::from_path(csv_file_name).expect("Unable to create CSV file");

	// Write the header row
	wtr.write_record(&["algo", "len_vertices", "len_edges", "time", "percentage"])
		.expect("Failed to write header row");

	for &vertices in &num_vertices {
		let edges = edge_fn(vertices);
		let graph = GraphBuilder::new()
			.set_num_vertices(vertices)
			.set_num_edges(edges)
			.build_random::<VESetGraph>();

		println!("Generating data for vertices: {}, edges: {}", vertices, edges);

		// Analyze using FastCut
		let (best_result_fast, time_fast, percentage_fast) = analyze_algorithm(graph.clone(), fast_cut);
		wtr.write_record(&["fast_cut", &vertices.to_string(), &edges.to_string(), &time_fast.to_string(), &percentage_fast.to_string()])
			.expect("Failed to write FastCut row");

		println!("For a graph with {} vertices and {} edges, FastCut took {} µs and found {}", vertices, edges, time_fast, best_result_fast);

		// Analyze using Contract
		let (best_result_contract, time_contract, percentage_contract) = analyze_algorithm(graph.clone(), contract);
		wtr.write_record(&["contract", &vertices.to_string(), &edges.to_string(), &time_contract.to_string(), &percentage_contract.to_string()])
			.expect("Failed to write Contract row");

		println!("For a graph with {} vertices and {} edges, Contract took {} µs and found {}", vertices, edges, time_contract, best_result_contract);

		wtr.flush().expect("Failed to flush CSV writer");
	}
}

fn analyze_algorithm<T, F>(graph: T, algo: F) -> (usize, u128, f64)
where
	T: UnMulGraph + Clone,
	F: Fn(T) -> usize,
{
	let mut time = 0;
	let mut best_result = usize::MAX;
	let mut count: u8 = 0;

	for _ in 0..100 {
		let start = Instant::now();
		let result = best_cut(graph.clone(), 1, &algo);
		time += start.elapsed().as_micros();

		if result < best_result {
			best_result = result;
			count = 1;
		} else if result == best_result {
			count += 1;
		}
	}

	let percentage = (count as f64 / 100.0) * 100.0;
	(best_result, time, percentage)
}