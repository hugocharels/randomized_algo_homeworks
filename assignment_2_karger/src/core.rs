use crate::min_cut::UnMulGraph;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct AdjacencyMatrix {
	pub matrix: Vec<Vec<usize>>,
}

impl AdjacencyMatrix {
	pub fn new() -> Self {
		Self { matrix: Vec::new() }
	}

	pub fn add_edge(&mut self, u: usize, v: usize) {
		self.add_vertex_if_not_exists(u);
		self.add_vertex_if_not_exists(v);
		self.matrix[u.max(v)][v.min(u)] += 1;
	}

	pub fn len(&self) -> usize {
		self.matrix.len()
	}

	fn add_vertex_if_not_exists(&mut self, v: usize) {
		for i in 0..v + 1 {
			if self.matrix.len() <= i {
				self.matrix.push(vec![0; i + 1]);
			} else if self.matrix[i].len() <= i {
				self.matrix[i].resize(v + 1, 0);
			}
		}
	}

}

#[derive(Debug, Clone)]
pub struct AdjacencyMatrixGraph {
	pub adj_matrix: AdjacencyMatrix,
}

impl AdjacencyMatrixGraph {
	pub fn new() -> Self {
		Self { adj_matrix: AdjacencyMatrix::new() }
	}

	pub fn add_edge(&mut self, u: usize, v: usize) {
		self.adj_matrix.add_edge(u, v);
	}

	pub fn contract_edge(&mut self, u: usize, v: usize) {
		if self.adj_matrix.len() <= u || self.adj_matrix.len() <= v {
			panic!("Vertex {} or {} does not exist", u, v);
		} else {
			todo!();
		}
	}
}


#[derive(Debug, Clone)]
pub struct VESetGraph {
	pub edge_list: Vec<(usize, usize)>,  // [(u, v), ...]
	pub vertex_set: HashSet<usize>,      // {u, v, ...}
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
		// Remove the edge (u, v) from the edge list
		self.edge_list.retain(|&e| e != (u.min(v), v.max(u)));

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
	}

	fn get_random_edge(&self) -> (usize, usize) {
		let rand_idx = rand::random::<usize>() % self.edge_list.len();
		self.edge_list[rand_idx]
	}

	fn len_vertices(&self) -> usize {
		self.vertex_set.len()
	}

	fn len_edges(&self) -> usize {
		self.edge_list.len()
	}
}

