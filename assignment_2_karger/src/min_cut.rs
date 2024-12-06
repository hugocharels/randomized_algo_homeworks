// Undirected MultiGraph Interface
pub trait UnMulGraph {
	fn new() -> Self; // Create a new graph
	fn add_edge(&mut self, u: usize, v: usize); // Add edge (u, v)
	fn contract_edge(&mut self, u: usize, v: usize); // Contract edge (u, v)
	fn get_random_edge(&self) -> (usize, usize); // Return a random edge
	fn len_vertices(&self) -> usize; // Return the number of vertices
	fn len_edges(&self) -> usize; // Return the number of edges
	fn min(&self, other: Self) -> Self; // Return the graph with the minimum number of edges
}


// Karger's Algorithms for finding the minimum cut of a graph

fn contract(graph: &mut impl UnMulGraph, t: Option<usize>) -> impl UnMulGraph {
	let t = t.unwrap_or(2);
	while graph.len_vertices() > t {
		let (u, v) = graph.get_random_edge();
		graph.contract_edge(u, v);
	}
	graph
}

fn fast_cut(graph: &mut impl UnMulGraph) -> impl UnMulGraph {
	if graph.len_vertices() <= 6 {
		find_min_cut(graph)
	} else {
		let t = (1.0 + graph.len_vertices() as f64 / 2.0_f64.sqrt()).ceil() as usize;
		let mut g1 = contract(graph.clone(), Some(t));
		let mut g2 = contract(graph.clone(), Some(t));
		fast_cut(&mut g1).min(fast_cut(&mut g2))
	}
}

// Brute force algorithm for finding the minimum cut of a graph
fn find_min_cut(graph: &mut impl UnMulGraph) -> impl UnMulGraph {
	todo!("Implement find_min_cut")
}