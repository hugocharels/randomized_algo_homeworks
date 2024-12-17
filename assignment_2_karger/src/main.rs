use assignment_2_karger::data_generator::*;
use rayon::prelude::*;

fn main() {
	// Define the operations and their corresponding file names
	let tasks:Vec<(fn(usize)->usize, &str)>  = vec![
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

	// Run each task in parallel
	tasks.into_par_iter().for_each(|(operation, filename)| {
		println!("Generating data for {}", filename);
		generate_data(
			vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000],
			operation,
			filename,
		);
		println!("Data generated for {}", filename);
	});
}