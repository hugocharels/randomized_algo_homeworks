use assignment_2_karger::{
	core::VESetGraph,
	graph_builder::GraphBuilder,
	min_cut::{contract, fast_cut, UnMulGraph},
};

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

fn main() {
	let graph = GraphBuilder::new()
		.set_num_vertices(50)
		.set_max_num_edges()
		.build_random::<VESetGraph>();
	// println!("{:?}", graph);
	println!("Contract Result: {:?}", best_cut(graph.clone(), 10, contract));
	println!("FastCut Result: {:?}", best_cut(graph.clone(), 1, fast_cut));
	// println!("BruteForce Result: {:?}", min_cut(graph.clone()));
}
