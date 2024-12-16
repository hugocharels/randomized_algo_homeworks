use std::fmt::Debug;

// Undirected MultiGraph Interface
pub trait UnMulGraph: Debug + Clone {
	fn add_edge(&mut self, u: usize, v: usize); // Add edge (u, v)
	fn contract_edge(&mut self, u: usize, v: usize); // Contract edge (u, v)
	fn get_num_edges(&self, u: usize, v: usize) -> usize; // Return the number of edges between u, v
	fn get_random_edge(&self) -> (usize, usize); // Return a random edge
	fn edge_exists(&self, u: usize, v: usize) -> bool; // Return if the edge belongs to the graph
	fn len_vertices(&self) -> usize; // Return the number of vertices
	fn len_edges(&self) -> usize; // Return the number of edges
}


// Karger's Algorithms for finding the minimum cut of a graph

pub fn contract(mut graph: impl UnMulGraph) -> usize {
	while graph.len_vertices() > 2 {
		if graph.len_edges() == 0 {
			println!("No more edges (wtf): {}", graph.len_vertices());
		}
		let (u, v) = graph.get_random_edge();
		graph.contract_edge(u, v);
	}
	graph.len_edges()
}

fn contract_t(mut graph: impl UnMulGraph + Clone, t: usize) -> impl UnMulGraph + Clone {
	while graph.len_vertices() > t {
		if graph.len_edges() == 0 {
			println!("No more edges (wtf): {}", graph.len_vertices());
		}
		let (u, v) = graph.get_random_edge();
		graph.contract_edge(u, v);
	}
	graph.clone()
}

pub fn fast_cut(graph: impl UnMulGraph + Clone) -> usize {
	if graph.len_vertices() <= 6 {
		min_cut(graph)
	} else {
		let t = (1.0 + graph.len_vertices() as f64 / 2.0_f64.sqrt()).ceil() as usize;
		let g1 = contract_t(graph.clone(), t);
		let g2 = contract_t(graph.clone(), t);
		if g1.len_edges() < g2.len_edges() {
			fast_cut(g1)
		} else {
			fast_cut(g2)
		}
	}
}

// Brute force algorithm for finding the minimum cut of a graph
pub fn min_cut(graph: impl UnMulGraph + Clone) -> usize {
	let n = graph.len_vertices();
	let mut min_cut = usize::MAX;

	// Generate all possible partitions of the vertices
	// Need the set of vertices to be {0, 1, 2, ..., n - 1}
	let total_partitions: u64 = 1 << n; // 2^n
	for mask in 1..(total_partitions / 2) {
		// Divide vertices into two sets based on the binary representation of the mask
		let mut set_a = Vec::new();
		let mut set_b = Vec::new();

		for i in 0..n {
			if (mask & (1 << i)) != 0 {
				set_a.push(i);
			} else {
				set_b.push(i);
			}
		}
		// Count the edges crossing the partition
		let mut crossing_edges = 0;
		for u in &set_a {
			for v in &set_b {
				// Add the number of edges u, v
				crossing_edges += graph.get_num_edges(*u, *v);
			}
		}
		assert_ne!(crossing_edges, 0, "Crossing edges can't be 0");

		// Update the minimum cut
		if crossing_edges < min_cut {
			min_cut = crossing_edges;
		}
	}

	min_cut
}
