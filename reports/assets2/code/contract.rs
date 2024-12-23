fn contract(mut graph: impl UnMulGraph) -> usize {
	while graph.len_vertices() > 2 {
		let (u, v) = graph.get_random_edge();
		graph.contract_edge(u, v);
	}
	graph.len_edges()
}