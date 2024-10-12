mod select;
mod integer;

use rand::Rng;
use integer::Integer;
use std::fs::File;
use csv::Writer;

const START_LIST_SIZE: usize = 10_000;    // Starting list size (10k)
const END_LIST_SIZE: usize = 1_000_000;   // Ending list size (1M)
const STEP: usize = 10_000;               // Step size (increase by 10k)
const NUM_LISTS: usize = 100;              // Number of lists to create for each list size

fn gen_csv() -> std::io::Result<()> {
	let mut rng = rand::thread_rng();

	// Create a CSV writer to write to a file
	let file = File::create("../comparison_results.csv")?;
	let mut wtr = Writer::from_writer(file);

	// Write the header
	wtr.write_record(&["List Size", "Algorithm", "Comparisons"])?;

	// Iterate through different list sizes from 10k to 1M
	for list_size in (START_LIST_SIZE..=END_LIST_SIZE).step_by(STEP) {
		println!("List size: {}", list_size);
		// Execute QuickSelect and LazySelect on each list size multiple times
		for _ in 0..NUM_LISTS {
			// Create a list of Integers to track comparisons
			let list: Vec<Integer> = (0..list_size).map(|_| Integer::new(rng.gen_range(0..(list_size as i32 - 1i32)))).collect();
			let k = rng.gen_range(0..=(list_size - 1));

			// Clear comparisons before running QuickSelect
			Integer::clear_comparisons();
			let _q_res = select::quick_select(&list, k);
			let q_comparisons = Integer::get_comparisons();

			// Write QuickSelect data to CSV
			wtr.write_record(&[list_size.to_string(), "QuickSelect".to_string(), q_comparisons.to_string()])?;

			// Clear comparisons before running LazySelect
			Integer::clear_comparisons();
			let _l_res = select::lazy_select(&list, k);
			let l_comparisons = Integer::get_comparisons();

			if _q_res != _l_res {
				println!("Mismatch");
				break;
			}

			// Write LazySelect data to CSV
			wtr.write_record(&[list_size.to_string(), "LazySelect".to_string(), l_comparisons.to_string()])?;
		}
	}

	// Flush the CSV writer to ensure all data is written
	wtr.flush()?;

	Ok(())
}


fn compare() {
	let mut rng = rand::thread_rng();

	// Execute QuickSelect and LazySelect on each list
	for i in 0..NUM_LISTS {
		println!("List {}", i);
		// Create a list of Integers instead of i32 to track comparisons
		let list: Vec<i32> = (0..START_LIST_SIZE).map(|_| rng.gen_range(0..START_LIST_SIZE - 1) as i32).collect();
		let k = rng.gen_range(0..=(START_LIST_SIZE - 1));
		let q_res = select::quick_select(&list, k);
		let l_res = select::lazy_select(&list, k);
		if q_res != l_res {
			println!("Mismatch at list {} and k {}", i, k);
			break;
		}

	}
}

fn main() {
	gen_csv().expect("");
	// compare()
}