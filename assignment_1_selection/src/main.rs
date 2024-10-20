mod select;
mod integer;

use rand::Rng;
use integer::Integer;
use std::fs::File;
use std::time::Instant;
use csv::Writer;


fn gen_csv_comparisons_runtime(start_size: usize, end_size: usize, step: usize, num_lists: usize) -> std::io::Result<()> {
	let mut rng = rand::thread_rng();
	let suffix = start_size.to_string() + "_" + &*end_size.to_string() + "_" + &*step.to_string() + "_" + &*num_lists.to_string();

	// Create a CSV writer for comparisons
	let comparisons_file = File::create("csv/comparisons_results_".to_owned() + &*suffix + ".csv")?;
	let mut comparisons_wtr = Writer::from_writer(comparisons_file);

	// Create a CSV writer for runtime
	let runtime_file = File::create("csv/runtime_results_".to_owned() + &*suffix + ".csv")?;
	let mut runtime_wtr = Writer::from_writer(runtime_file);

	// Write the headers for both CSV files
	comparisons_wtr.write_record(&["List Size", "Algorithm", "Comparisons"])?;
	runtime_wtr.write_record(&["List Size", "Algorithm", "Runtime (ms)"])?;

	// Iterate through different list sizes from 10k to 1M
	for list_size in (start_size..=end_size).step_by(step) {
		// Execute QuickSelect and LazySelect on each list size multiple times
		println!("List size: {}", list_size);
		for it in 0..num_lists {
			// Create a list of Integers to track comparisons
			let list: Vec<Integer> = (0..list_size).map(|_| Integer::new(rng.gen_range(0..(list_size as i32 - 1i32)))).collect();
			let k = rng.gen_range(0..=(list_size - 1));

			// Clear comparisons before running QuickSelect
			Integer::clear_comparisons();
			let start_time_qs = Instant::now(); // Start time for QuickSelect
			let _q_res = select::quick_select(&list, k);
			let runtime_qs = start_time_qs.elapsed().as_millis(); // Runtime for QuickSelect
			let q_comparisons = Integer::get_comparisons();

			// Write QuickSelect comparisons to comparisons CSV
			comparisons_wtr.write_record(&[
				list_size.to_string(),
				"QuickSelect".to_string(),
				q_comparisons.to_string(),
			])?;

			// Write QuickSelect runtime to runtime CSV
			runtime_wtr.write_record(&[
				list_size.to_string(),
				"QuickSelect".to_string(),
				runtime_qs.to_string(),
			])?;

			// Clear comparisons before running LazySelect
			Integer::clear_comparisons();
			let start_time_ls = Instant::now(); // Start time for
			let _l_res = select::lazy_select(&list, k);
			let runtime_ls = start_time_ls.elapsed().as_millis(); // Runtime for
			let l_comparisons = Integer::get_comparisons();

			if _q_res != _l_res {
				println!("Mismatch");
				break;
			}

			// Write LazySelect comparisons to comparisons CSV
			comparisons_wtr.write_record(&[
				list_size.to_string(),
				"LazySelect".to_string(),
				l_comparisons.to_string(),
			])?;

			// Write LazySelect runtime to runtime CSV
			runtime_wtr.write_record(&[
				list_size.to_string(),
				"LazySelect".to_string(),
				runtime_ls.to_string(),
			])?;
		}

		// Flush the CSV writers to ensure all data is written
		comparisons_wtr.flush()?;
		runtime_wtr.flush()?;
	}

	Ok(())
}


fn gen_csv_comparisons_k(size: usize, num_lists: usize) -> std::io::Result<()> {
	let mut rng = rand::thread_rng();
	let suffix = size.to_string() + "_" + &*num_lists.to_string();

	// Create a CSV writer for comparisons
	let comparisons_file = File::create("csv/comparisons_results_k_".to_owned() + &*suffix + ".csv")?;
	let mut comparisons_wtr = Writer::from_writer(comparisons_file);

	// Write the headers for both CSV files
	comparisons_wtr.write_record(&["List Size", "Algorithm", "Comparisons", "K"])?;

	// Iterate through different list sizes from 10k to 1M
	for k in (0..=size-1) {
		// Execute QuickSelect and LazySelect on each list size multiple times
		println!("List size: {}, k: {}", size, k);
		for it in 0..num_lists {
			// Create a list of Integers to track comparisons
			let list: Vec<Integer> = (0..size).map(|_| Integer::new(rng.gen_range(0..(size as i32 - 1i32)))).collect();

			// Clear comparisons before running QuickSelect
			Integer::clear_comparisons();
			let _q_res = select::quick_select(&list, k);
			let q_comparisons = Integer::get_comparisons();

			// Write QuickSelect comparisons to comparisons CSV
			comparisons_wtr.write_record(&[
				size.to_string(),
				"QuickSelect".to_string(),
				q_comparisons.to_string(),
				k.to_string(),
			])?;

			// Clear comparisons before running LazySelect
			Integer::clear_comparisons();
			let _l_res = select::lazy_select(&list, k);
			let l_comparisons = Integer::get_comparisons();

			if _q_res != _l_res {
				println!("Mismatch");
				break;
			}

			// Write LazySelect comparisons to comparisons CSV
			comparisons_wtr.write_record(&[
				size.to_string(),
				"LazySelect".to_string(),
				l_comparisons.to_string(),
				k.to_string(),
			])?;
		}

		// Flush the CSV writers to ensure all data is written
		comparisons_wtr.flush()?;
	}

	Ok(())
}



use rayon::prelude::*;

fn main() {
	let tasks = vec![
		|| gen_csv_comparisons_runtime(5000, 100000, 5000, 100),
		|| gen_csv_comparisons_runtime(50000, 1000000, 50000, 100),
		|| gen_csv_comparisons_runtime(500000, 10000000, 500000, 100),
		|| gen_csv_comparisons_k(10000, 100),
	];

	tasks.into_par_iter().for_each(|task| {
		task().unwrap();
	});
}
