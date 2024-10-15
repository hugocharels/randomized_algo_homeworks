mod select;
mod integer;

use rand::Rng;
use integer::Integer;
use std::fs::File;
use std::time::Instant;
use csv::Writer;

const START_LIST_SIZE: usize = 200_000;    // Starting list size (10k)
const END_LIST_SIZE: usize = 10_000_000;   // Ending list size (1M)
const STEP: usize = 200_000;               // Step size (increase by 10k)
const NUM_LISTS: usize = 100;              // Number of lists to create for each list size


fn main() -> std::io::Result<()> {
	let mut rng = rand::thread_rng();

	// Create a CSV writer for comparisons
	let comparisons_file = File::create("comparisons_results.csv")?;
	let mut comparisons_wtr = Writer::from_writer(comparisons_file);

	// Create a CSV writer for runtime
	let runtime_file = File::create("runtime_results.csv")?;
	let mut runtime_wtr = Writer::from_writer(runtime_file);

	// Write the headers for both CSV files
	comparisons_wtr.write_record(&["List Size", "Algorithm", "Comparisons"])?;
	runtime_wtr.write_record(&["List Size", "Algorithm", "Runtime (ms)"])?;

	// Iterate through different list sizes from 10k to 1M
	for list_size in (START_LIST_SIZE..=END_LIST_SIZE).step_by(STEP) {
		// Execute QuickSelect and LazySelect on each list size multiple times
		for it in 0..NUM_LISTS {
			println!("List size: {}, it: {}", list_size, it);
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
			let start_time_ls = Instant::now(); // Start time for LazySelect
			let _l_res = select::lazy_select(&list, k);
			let runtime_ls = start_time_ls.elapsed().as_millis(); // Runtime for LazySelect
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
