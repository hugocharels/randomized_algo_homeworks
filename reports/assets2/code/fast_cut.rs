fn fast_cut(graph: impl UnMulGraph + Clone) -> usize {
	if graph.len_vertices() <= 6 {
		min_cut(graph)
	} else {
		let t = (1.0 + graph.len_vertices() as f64 / 2.0_f64.sqrt()).ceil() as usize;
		let g1 = contract_t(graph.clone(), t);
		let g2 = contract_t(graph.clone(), t);
		min(fast_cut(g1), fast_cut(g2))
	}
}

fn contract_t(mut graph: impl UnMulGraph + Clone, t: usize) -> impl UnMulGraph + Clone {
	while graph.len_vertices() > t {
		let (u, v) = graph.get_random_edge();
		graph.contract_edge(u, v);
	}
	graph.clone()
}