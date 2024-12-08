// Undirected MultiGraph Interface
pub trait UnMulGraph {
	fn new() -> Self; // Create a new graph
	fn add_edge(&mut self, u: usize, v: usize); // Add edge (u, v)
	fn contract_edge(&mut self, u: usize, v: usize); // Contract edge (u, v)
	fn get_random_edge(&self) -> (usize, usize); // Return a random edge
	fn len_vertices(&self) -> usize; // Return the number of vertices
	fn len_edges(&self) -> usize; // Return the number of edges
}


// Karger's Algorithms for finding the minimum cut of a graph

pub fn contract(graph: &mut impl UnMulGraph) -> usize {
	while graph.len_vertices() > 2 {
		let (u, v) = graph.get_random_edge();
		graph.contract_edge(u, v);
	}
	graph.len_vertices()
}

fn contract_t(mut graph: (impl UnMulGraph + Clone), t: usize) -> impl UnMulGraph + Clone {
	while graph.len_vertices() > t {
		let (u, v) = graph.get_random_edge();
		graph.contract_edge(u, v);
	}
	graph.clone()
}

pub fn fast_cut(mut graph: (impl UnMulGraph + Clone)) -> usize {
	if graph.len_vertices() <= 6 {
		find_min_cut(graph)
	} else {
		let t = (1.0 + graph.len_vertices() as f64 / 2.0_f64.sqrt()).ceil() as usize;
		let mut g1 = contract_t(graph.clone(), t);
		let mut g2 = contract_t(graph.clone(), t);
		if g1.len_edges() < g2.len_edges() {
			fast_cut(g1)
		} else {
			fast_cut(g2)
		}
	}
}

// Brute force algorithm for finding the minimum cut of a graph
fn find_min_cut(mut graph: (impl UnMulGraph + Clone)) -> usize {
	// TODO: Implement the brute force algorithm for finding the minimum cut of a graph
	let mut min_cut = usize::MAX;
	let n = graph.len_vertices();
	let n_squared = n * n;
	let n_squared_log_n = (n_squared as f64).log2() as usize;
	for _ in 0..n_squared_log_n {
		let mut g = graph.clone();
		let cut = contract(&mut g);
		if cut < min_cut {
			min_cut = cut;
		}
	}
	min_cut
}