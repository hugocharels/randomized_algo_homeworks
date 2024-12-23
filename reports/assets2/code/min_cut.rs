// Brute force algorithm for finding the minimum cut of a graph
fn min_cut(graph: impl UnMulGraph + Clone) -> usize {
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

// Helper function to generate all possible bipartitions of a set
fn bipartitions(set: &HashSet<usize>) -> impl Iterator<Item=(HashSet<usize>, HashSet<usize>)> {
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