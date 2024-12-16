use assignment_2_karger::{
	core::VESetGraph,
	data_generator::*,
	graph_builder::GraphBuilder,
	min_cut::{contract, fast_cut},
};

fn main() {
	let graph = GraphBuilder::new()
		.set_num_vertices(5)
		.set_num_edges(8)
		.build_random::<VESetGraph>();
	println!("{:?}", graph);
	println!("Contract Result: {:?}", best_cut(graph.clone(), 5, contract));
	println!("FastCut Result: {:?}", best_cut(graph.clone(), 1, fast_cut));
	// println!("BruteForce Result: {:?}", min_cut(graph.clone()));

	generate_data();
}
