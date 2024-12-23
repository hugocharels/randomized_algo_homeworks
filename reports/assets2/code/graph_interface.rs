// Undirected MultiGraph Interface
trait UnMulGraph: Debug + Clone {
	fn add_edge(&mut self, u: usize, v: usize); // Add edge (u, v)
	fn contract_edge(&mut self, u: usize, v: usize); // Contract edge (u, v)
	fn get_num_edges(&self, u: usize, v: usize) -> usize; // Return the number of edges between u, v
	fn get_random_edge(&self) -> (usize, usize); // Return a random edge
	fn edge_exists(&self, u: usize, v: usize) -> bool; // Return if the edge belongs to the graph
	fn len_vertices(&self) -> usize; // Return the number of vertices
	fn len_edges(&self) -> usize; // Return the number of edges
}