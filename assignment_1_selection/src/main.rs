mod select;
use rand::Rng;

const LIST_SIZE: usize = 1000;  // Size of each list
const NUM_LISTS: usize = 5;  // Number of lists to create

fn main() {
	// Generate random lists
	let mut lists: Vec<Vec<i32>> = Vec::new();
	let mut rng = rand::thread_rng();

	for _ in 0..NUM_LISTS {
		let list: Vec<i32> = (0..LIST_SIZE).map(|_| rng.gen_range(1..1000)).collect();
		lists.push(list);
	}

	// Execute QuickSelect and LazySelect on each list
	for (i, list) in lists.iter_mut().enumerate() {
		let k = rng.gen_range(1..=LIST_SIZE);

		//println!("List {}: {:?}", i + 1, list);
		println!(
			"The {}-th smallest element in List {} (QuickSelect) is {}",
			k,
			i + 1,
			select::quick_select(&mut list.clone(), k)
		);
		println!(
			"The {}-th smallest element in List {} (LazySelect) is {}",
			k,
			i + 1,
			select::lazy_select(&mut list.clone(), k)
		);
		println!("--------------------------------");
	}
}
