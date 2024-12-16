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

pub fn generate_data() {
	// Open a csv file in write mode
	let mut wtr = csv::Writer::from_path("data.csv").unwrap();

	// Write the header row
	wtr.write_record(&["algo", "len_vertices", "len_edges", "time", "percentage"]).unwrap();

	// List of num of vertices and edges
	let num_vertices = vec![16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192];
	let num_edges = vec![64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768];

	for i in 0..num_vertices.len().min(num_edges.len()) {
		let graph = GraphBuilder::new()
			.set_num_vertices(num_vertices[i])
			.set_num_edges(num_edges[i])
			.build_random::<VESetGraph>();

		println!("Generating data for vertices: {}, edges: {}", num_vertices[i], num_edges[i]);

		let mut time = 0;
		let mut best_result = usize::MAX;
		let mut count: u8 = 0;
		for _ in 0..100 {
			let start = Instant::now();
			let result = best_cut(graph.clone(), 1, fast_cut);
			time += start.elapsed().as_micros();
			if result < best_result {
				best_result = result;
				count = 1;
			} else if result == best_result {
				count += 1;
			}
		}
		let percentage = (count as f64 / 100.0) * 100.0;
		wtr.write_record(&["fast_cut", &num_vertices[i].to_string(), &num_edges[i].to_string(), &time.to_string(), &percentage.to_string()]).unwrap();

		println!("Result for FastCut: {}", best_result);

		time = 0;
		count = 0;
		for _ in 0..100 {
			let start = Instant::now();
			let result = best_cut(graph.clone(), 1, contract);
			time += start.elapsed().as_micros();
			if result < best_result {
				best_result = result;
				count = 1;
			} else if result == best_result {
				count += 1;
			}
		}
		let percentage = (count as f64 / 100.0) * 100.0;
		wtr.write_record(&["contract", &num_vertices[i].to_string(), &num_edges[i].to_string(), &time.to_string(), &percentage.to_string()]).unwrap();

		println!("Result for Contract: {}", best_result);

		wtr.flush().unwrap();
	}
}