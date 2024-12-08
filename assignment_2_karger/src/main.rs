use crate::min_cut::UnMulGraph;
mod core;
mod min_cut;

fn main() {
	let mut g = core::VESetGraph::new();
	g.add_edge(0, 1);
	g.add_edge(0, 2);
	g.add_edge(1, 2);
	g.add_edge(1, 3);
	g.add_edge(2, 3);
	g.add_edge(0, 1);
	println!("{:?}", g);
	println!("{:?}", min_cut::contract(&mut g));
	println!("{:?}", min_cut::fast_cut(g));
}
