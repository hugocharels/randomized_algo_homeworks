use crate::min_cut::UnMulGraph;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;

pub struct GraphBuilder {
	num_vertices: usize,
	num_edges: usize,
}

impl GraphBuilder {
	pub fn new() -> Self {
		Self {
			num_vertices: 0,
			num_edges: 0,
		}
	}

	pub fn set_num_vertices(mut self, num_vertices: usize) -> Self {
		assert!(num_vertices > 0, "Number of vertices must be greater than 0.");
		self.num_vertices = num_vertices;
		self
	}

	pub fn set_num_edges(mut self, num_edges: usize) -> Self {
		assert!(num_edges >= 1, "Number of edges must be at least 1.");
		self.num_edges = num_edges;
		self
	}

	pub fn set_min_num_vertices(mut self) -> Self {
		assert!(self.num_edges > 1, "Number of edges must be set first.");
		self.num_vertices = self.num_edges + 1;
		self
	}

	pub fn set_max_num_edges(mut self) -> Self {
		assert!(self.num_vertices > 1, "Number of vertices must be set first.");
		self.num_edges = self.num_vertices * (self.num_vertices - 1) / 2;
		self
	}

	pub fn build_random<T: UnMulGraph + Default>(self) -> T {
		assert!(self.num_edges >= self.num_vertices - 1, "Number of edges must be at least num_vertices - 1 for a connected graph.");
		assert!(self.num_edges <= self.num_vertices * (self.num_vertices - 1) / 2, "Too many edges for a simple graph.");

		let mut graph = T::default();
		let mut rng = rand::thread_rng();

		// Step 1: Create a spanning tree to ensure connectivity
		let mut connected: Vec<usize> = (0..self.num_vertices).collect();
		connected.shuffle(&mut rng);

		let mut edges: HashSet<(usize, usize)> = HashSet::new();
		for i in 1..self.num_vertices {
			let u = connected[i - 1];
			let v = connected[i];
			graph.add_edge(u, v);
			edges.insert((u.min(v), u.max(v)));
		}

		// Step 2: Add remaining edges randomly while avoiding loops and parallel edges
		while edges.len() < self.num_edges {
			let u = rng.gen_range(0..self.num_vertices);
			let v = rng.gen_range(0..self.num_vertices);

			if u != v {
				let edge = (u.min(v), u.max(v));
				if edges.insert(edge) {
					graph.add_edge(edge.0, edge.1);
				}
			}
		}

		graph
	}
}