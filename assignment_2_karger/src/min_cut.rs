use std::cmp::min;
use std::collections::HashSet;
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
	fn vertex_set(&self) -> &HashSet<usize>; // Return the set of vertices
}

// Karger's Algorithms for finding the minimum cut of a graph

pub fn contract(mut graph: impl UnMulGraph) -> usize {
	while graph.len_vertices() > 2 {
		let (u, v) = graph.get_random_edge();
		graph.contract_edge(u, v);
	}
	graph.len_edges()
}

fn contract_t(mut graph: impl UnMulGraph + Clone, t: usize) -> impl UnMulGraph + Clone {
	while graph.len_vertices() > t {
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
		min(fast_cut(g1), fast_cut(g2))
	}
}

// Helper function to generate all possible bipartitions of a set
fn bipartitions(set: &HashSet<usize>) -> impl Iterator<Item = (HashSet<usize>, HashSet<usize>)> {
	let elements: Vec<_> = set.iter().cloned().collect();

	(1..(1 << elements.len())).filter_map(move |mask| {
		let mut part1 = HashSet::new();
		let mut part2 = HashSet::new();

		for (i, &elem) in elements.iter().enumerate() {
			if (mask & (1 << i)) != 0 {
				part1.insert(elem);
			} else {
				part2.insert(elem);
			}
		}

		// Ensure that only one of the symmetric pairs is returned
		if !part1.is_empty() && !part2.is_empty() && part1.iter().min() <= part2.iter().min() {
			Some((part1, part2))
		} else {
			None
		}
	})
}

// Brute force algorithm for finding the minimum cut of a graph
pub fn min_cut(graph: impl UnMulGraph + Clone) -> usize {
	let mut min_cut = usize::MAX;
	for (set_a, set_b) in bipartitions(graph.vertex_set()) {
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
