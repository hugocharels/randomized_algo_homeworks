use crate::min_cut::UnMulGraph;
mod core;
mod min_cut;

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
	println!("Contract Result: {:?}", min_cut::contract(g.clone()));
	println!("FastCut Result: {:?}", min_cut::fast_cut(g.clone()));
	println!("BruteForce Result: {:?}", min_cut::min_cut(g.clone()));
}
