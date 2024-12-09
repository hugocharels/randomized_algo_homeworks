use crate::min_cut::{contract, fast_cut, min_cut, UnMulGraph};
mod core;
mod min_cut;

// Run `contract` n times and return the best result
pub fn best_contract(graph: impl UnMulGraph + Clone, n: usize) -> usize {
	let mut best_result = usize::MAX;
	for _ in 0..n {
		let result = contract(graph.clone());
		if result < best_result {
			best_result = result;
		}
	}
	best_result
}

// Run `fast_cut` n times and return the best result
pub fn best_fast_cut(graph: impl UnMulGraph + Clone, n: usize) -> usize {
	let mut best_result = usize::MAX;
	for _ in 0..n {
		let result = fast_cut(graph.clone());
		if result < best_result {
			best_result = result;
		}
	}
	best_result
}


fn main() {
	let mut g = core::VESetGraph::new();
	g.add_edge(0, 1);
	g.add_edge(1, 2);
	g.add_edge(2, 3);
	g.add_edge(3, 0);
	g.add_edge(0, 2);
	g.add_edge(1, 3);
	g.add_edge(3, 4);
	g.add_edge(4, 5);
	g.add_edge(5, 6);
	g.add_edge(6, 4);

	println!("{:?}", g);
	println!("Contract Result: {:?}", contract(g.clone()));
	println!("FastCut Result: {:?}", fast_cut(g.clone()));
	println!("BruteForce Result: {:?}", min_cut(g.clone()));
}
