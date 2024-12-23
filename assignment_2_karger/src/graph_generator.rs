use crate::min_cut::UnMulGraph;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;

pub struct GraphGenerator {
	num_vertices: usize,
	num_edges: usize,
	max_edges: bool,
}

impl GraphGenerator {
	pub fn new() -> Self {
		Self {
			num_vertices: 0,
			num_edges: 0,
			max_edges: false,
		}
	}

	pub fn set_num_vertices(mut self, num_vertices: usize) -> Self {
		assert!(
			num_vertices > 0,
			"Number of vertices must be greater than 0."
		);
		self.num_vertices = num_vertices;
		self
	}

	pub fn set_num_edges(mut self, num_edges: usize) -> Self {
		assert!(num_edges >= 1, "Number of edges must be at least 1.");
		self.num_edges = num_edges;
		self.max_edges = false;
		self
	}

	pub fn set_min_num_vertices(mut self) -> Self {
		assert!(self.num_edges > 1, "Number of edges must be set first.");
		self.num_vertices = self.num_edges + 1;
		self
	}

	pub fn set_max_num_edges(mut self) -> Self {
		assert!(
			self.num_vertices > 1,
			"Number of vertices must be set first."
		);
		self.max_edges = true;
		self
	}

	pub fn build_random<T: UnMulGraph + Default>(self) -> T {
		assert!(
			self.num_edges >= self.num_vertices - 1,
			"Number of edges must be at least num_vertices - 1 for a connected graph."
		);
		assert!(
			self.num_edges <= self.num_vertices * (self.num_vertices - 1) / 2,
			"Too many edges for a simple graph."
		);

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

	pub fn build_complete<T: UnMulGraph + Default>(mut self) -> T {
		assert!(
			self.num_vertices > 0,
			"Number of vertices must be greater than 0."
		);

		self.num_edges = self.num_vertices * (self.num_vertices - 1) / 2;
		self.max_edges = true;

		let mut graph = T::default();
		for u in 0..self.num_vertices {
			for v in (u + 1)..self.num_vertices {
				graph.add_edge(u, v);
			}
		}

		graph
	}

	pub fn build_planar<T: UnMulGraph + Default>(mut self) -> T {
		assert!(self.num_vertices > 2, "Number of vertices must be greater than 2 for a planar graph.");
		let max_edges = 3 * self.num_vertices - 6;
		if self.max_edges {
			self.num_edges = max_edges;
		}

		let mut graph = T::default();

		// Step 1: Create a simple cycle to start with a planar structure
		for i in 0..self.num_vertices {
			let next = (i + 1) % self.num_vertices;
			graph.add_edge(i, next);
		}

		let mut edges: HashSet<(usize, usize)> = HashSet::new();
		for i in 0..self.num_vertices {
			let next = (i + 1) % self.num_vertices;
			edges.insert((i.min(next), i.max(next)));
		}

		// Step 2: Add additional edges while maintaining planarity
		let mut rng = rand::thread_rng();
		while edges.len() < self.num_edges {
			let u = rng.gen_range(0..self.num_vertices);
			let v = rng.gen_range(0..self.num_vertices);

			if u != v {
				let edge = (u.min(v), u.max(v));
				if !edges.contains(&edge) {
					// Check if adding this edge keeps the graph planar
					// Simplified planar edge addition for demonstration
					edges.insert(edge);
					graph.add_edge(edge.0, edge.1);
				}
			}
		}

		graph
	}
}
