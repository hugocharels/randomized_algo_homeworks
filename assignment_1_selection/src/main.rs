mod select;
mod integer;

use rand::Rng;
use integer::Integer;

const LIST_SIZE: usize = 10000000;  // Size of each list
const NUM_LISTS: usize = 1;  // Number of lists to create

fn main() {
	let mut rng = rand::thread_rng();

	// Execute QuickSelect and LazySelect on each list
	for i in 0..NUM_LISTS {
		// Create a list of Integers instead of i32 to track comparisons
		let list: Vec<Integer> = (0..LIST_SIZE).map(|_| Integer::new(rng.gen_range(0..10000))).collect();
		let k = rng.gen_range(0..=(LIST_SIZE - 1) / 2);

		// Clear comparisons before running QuickSelect
		Integer::clear_comparisons();
		let q_res = select::quick_select(&list, k);
		let q_comparisons = Integer::get_comparisons();

		// Clear comparisons before running LazySelect
		Integer::clear_comparisons();
		let l_res = select::lazy_select(&list, k);
		let l_comparisons = Integer::get_comparisons();

		// Check if QuickSelect and LazySelect results differ
		if q_res != l_res {
			println!(
				"List {}: QuickSelect and LazySelect results differ for k = {} with q = {} and l = {}",
				i, k, q_res, l_res
			);
		}

		// Print the number of comparisons made in each algorithm call
		println!("QuickSelect comparisons: {}", q_comparisons);
		println!("LazySelect comparisons: {}", l_comparisons);
	}
}
