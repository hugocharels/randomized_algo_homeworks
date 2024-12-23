use crate::core::VESetGraph;
use crate::graph_generator::GraphGenerator;
use crate::min_cut::*;
use csv::Writer;


const NUM_REPEAT: u32 = 100;

pub fn generate_data_success_bounds(num_vertices: &Vec<usize>, csv_file_name: &str) {
	// For each graph size, build a complete and a planar graph
	// Run FastCut and Contract 100 times for each graph
	// Compute the success rate of each algorithm
	// Write the results to a CSV file

	let mut writer = Writer::from_path(csv_file_name).expect("Cannot create CSV file");
	writer.write_record(&["Graph Size", "Graph Type", "FastCut Success Rate", "Contract Success Rate"]).expect("Cannot write to CSV file");

	for &vertices in num_vertices {
		let graph = GraphGenerator::new()
			.set_num_vertices(vertices)
			.build_complete::<VESetGraph>();

		let answer = vertices - 1;
		let mut fastcut_success = 0;
		let mut contract_success = 0;

		for _ in 0..NUM_REPEAT {
			let fastcut_answer = fast_cut(graph.clone());
			let contract_answer = contract(graph.clone());

			if fastcut_answer == answer {
				fastcut_success += 1;
			}
			if contract_answer == answer {
				contract_success += 1;
			}
		}

		println!("Vertices: {}, Graph Type: Complete, FastCut Success Rate: {}, Contract Success Rate: {}", vertices, fastcut_success, contract_success);
		writer.write_record(&[vertices.to_string(), "Complete".to_string(), fastcut_success.to_string(), contract_success.to_string()]).expect("Cannot write to CSV file");
		writer.flush().expect("Cannot write to CSV file");
	}

	for &vertices in num_vertices {
		let graph = GraphGenerator::new()
			.set_num_vertices(vertices)
			.set_max_num_edges()
			.build_planar::<VESetGraph>();

		let mut answer = usize::MAX;
		let mut fastcut_success = 0;
		let mut contract_success = 0;

		for _ in 0..NUM_REPEAT {
			let fastcut_answer = fast_cut(graph.clone());
			let contract_answer = contract(graph.clone());

			if fastcut_answer == answer {
				fastcut_success += 1;
			}
			if contract_answer == answer {
				contract_success += 1;
			}

			if fastcut_answer < answer {
				answer = fastcut_answer;
				fastcut_success = 1;
				contract_success = 0;
			}
			if contract_answer < answer {
				answer = contract_answer;
				fastcut_success = 0;
				contract_success = 1;
			}
		}

		println!("Vertices: {}, Graph Type: Planar, FastCut Success Rate: {}, Contract Success Rate: {}", vertices, fastcut_success, contract_success);
		writer.write_record(&[vertices.to_string(), "Planar".to_string(), fastcut_success.to_string(), contract_success.to_string()]).expect("Cannot write to CSV file");
		writer.flush().expect("Cannot write to CSV file");
	}
}

pub fn generate_data_same_time(num_vertices: &Vec<usize>, csv_file_name: &str) {
	// For each graph size, build a complete and a planar graph
	// Run FastCut 100 times and measure the time
	// Run Contract repeatedly until the elapsed time matches that of FastCut
	// Compute the success rate of each algorithm for the same amount of time
	// Write the results to a CSV file

	// TODO:

	let mut writer = Writer::from_path(csv_file_name).expect("Cannot create CSV file");
	writer.write_record(&["Graph Size", "Graph Type", "FastCut Success Rate", "Contract Success Rate"]).expect("Cannot write to CSV file");

	for &vertices in num_vertices {
		let graph = GraphGenerator::new()
			.set_num_vertices(vertices)
			.build_complete::<VESetGraph>();

		let answer = vertices - 1;
		let mut fastcut_success = 0;
		let mut contract_success = 0;

		let start = std::time::Instant::now();
		for _ in 0..NUM_REPEAT {
			if fast_cut(graph.clone()) == answer {
				fastcut_success += 1;
			}
		}
		let fastcut_time = start.elapsed();

		for _ in 0..NUM_REPEAT {
			let start = std::time::Instant::now();
			while start.elapsed() < fastcut_time / NUM_REPEAT {
				if contract(graph.clone()) == answer {
					contract_success += 1;
					break
				}
			}
		}
		println!("Vertices: {}, Graph Type: Complete, FastCut Success Rate: {}, Contract Success Rate: {}", vertices, fastcut_success, contract_success);
		writer.write_record(&[vertices.to_string(), "Complete".to_string(), fastcut_success.to_string(), contract_success.to_string()]).expect("Cannot write to CSV file");
		writer.flush().expect("Cannot write to CSV file");
	}

	for &vertices in num_vertices {
		let graph = GraphGenerator::new()
			.set_num_vertices(vertices)
			.set_max_num_edges()
			.build_planar::<VESetGraph>();

		let mut answer = usize::MAX;
		let mut fastcut_success = 0;
		let mut contract_success = 0;

		let start = std::time::Instant::now();
		for _ in 0..NUM_REPEAT {
			let fastcut_answer = fast_cut(graph.clone());
			if fastcut_answer == answer {
				fastcut_success += 1;
			} else if fastcut_answer < answer {
				answer = fastcut_answer;
				fastcut_success = 1;
			}
		}
		let fastcut_time = start.elapsed();

		for _ in 0..NUM_REPEAT {
			let start = std::time::Instant::now();
			while start.elapsed() < fastcut_time / NUM_REPEAT {
				if contract(graph.clone()) == answer {
					contract_success += 1;
					break
				}
			}
		}
		println!("Vertices: {}, Graph Type: Planar, FastCut Success Rate: {}, Contract Success Rate: {}", vertices, fastcut_success, contract_success);
		writer.write_record(&[vertices.to_string(), "Planar".to_string(), fastcut_success.to_string(), contract_success.to_string()]).expect("Cannot write to CSV file");
		writer.flush().expect("Cannot write to CSV file");
	}
}
