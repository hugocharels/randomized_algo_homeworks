use crate::min_cut::UnMulGraph;
use rand::prelude::IteratorRandom;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct VESetGraph {
	edge_list: Vec<(usize, usize)>, // [(u, v), ...]
	vertex_set: HashSet<usize>, // {u, v, ...}
}

impl VESetGraph {
	fn remap_vertices(&mut self, v: usize) {
		let last_vertex = self.vertex_set.len();
		self.vertex_set.remove(&last_vertex);
		self.vertex_set.insert(v);
		// Remap vertices in the edge list
		for i in 0..self.len_edges() {
			if self.edge_list[i].0 == last_vertex {
				self.edge_list[i].0 = v;
			} else if self.edge_list[i].1 == last_vertex {
				self.edge_list[i].1 = v;
			}
			// Ensure that the first vertex is less than the second vertex
			if self.edge_list[i].0 > self.edge_list[i].1 {
				self.edge_list[i] = (self.edge_list[i].1, self.edge_list[i].0);
			}
		}
	}
}

impl UnMulGraph for VESetGraph {
	fn new() -> Self {
		Self {
			edge_list: Vec::new(),
			vertex_set: HashSet::new(),
		}
	}

	fn add_edge(&mut self, u: usize, v: usize) {
		if u == v {
			panic!("Self-loop is not allowed");
		}
		// Insert vertices into the HashSet
		self.vertex_set.insert(u);
		self.vertex_set.insert(v);
		// Add edge to the edge list
		self.edge_list.push((u.min(v), v.max(u)));
	}

	fn contract_edge(&mut self, u: usize, v: usize) {
		let (u, v) = (u.min(v), u.max(v));

		// Remove the edge (u, v) from the edge list
		self.edge_list.retain(|&e| e != (u, v));

		// Remove vertex `v` from the vertex set
		self.vertex_set.remove(&v);

		// Replace all instances of `v` with `u` in the edge list
		self.edge_list = self.edge_list
			.iter()
			.map(|&e| {
				let (a, b) = e;
				let new_a = if a == v { u } else { a };
				let new_b = if b == v { u } else { b };
				(new_a.min(new_b), new_a.max(new_b))
			})
			.collect();

		// Remove self-loops (edges where both vertices are the same)
		self.edge_list.retain(|&(a, b)| a != b);

		// Remap vertices to ensure that the vertices are contiguous
		if self.len_vertices() != v {
			self.remap_vertices(v);
		}

		// Remove vertex `u` if there are no more edges connected to it
		if !self.edge_list.iter().any(|&(a, b)| a == u || b == u) {
			self.vertex_set.remove(&u);

			// Remap vertices to ensure that the vertices are contiguous
			self.remap_vertices(u);
		}
	}

	fn get_random_edge(&self) -> (usize, usize) {
		self.edge_list.iter().cloned().choose(&mut rand::thread_rng()).expect("No edges available")
	}

	fn edge_exists(&self, u: usize, v: usize) -> bool {
		self.edge_list.contains(&(u.min(v), v.max(u)))
	}

	fn len_vertices(&self) -> usize {
		self.vertex_set.len()
	}

	fn len_edges(&self) -> usize {
		self.edge_list.len()
	}
}
