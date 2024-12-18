use assignment_2_karger::data_generator::*;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

fn main() {

	// Define the operations and their corresponding file names
	let tasks: Vec<(fn(usize) -> usize, &str)> = vec![
		(
			|v: usize| 2 * v,
			"generated_data/hundred_2v.csv",
		),
		(
			|v: usize| 5 * v,
			"generated_data/hundred_5v.csv",
		),
		(
			|v: usize| 10 * v,
			"generated_data/hundred_10v.csv",
		),
	];

	// Set the number of worker threads
	let pool = ThreadPoolBuilder::new()
		.num_threads(tasks.len())
		.build()
		.expect("Failed to configure thread pool");

	// Run each task in parallel
	pool.install(|| {
		tasks.into_par_iter().for_each(|(operation, filename)| {
			println!("Generating data for {}", filename);
			generate_data(
				vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000],
				operation,
				filename,
			);
			println!("Data generated for {}", filename);
		});
	});
}