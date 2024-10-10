mod select;
use rand::Rng;

const LIST_SIZE: usize = 1000;  // Size of each list
const NUM_LISTS: usize = 1000;  // Number of lists to create

fn main() {
	let mut rng = rand::thread_rng();
	let mut n = 0;

	// Execute QuickSelect and LazySelect on each list
	for i in 0..NUM_LISTS {
		let list: Vec<i32> = (0..LIST_SIZE).map(|_| rng.gen_range(1..1000)).collect();
		let k = rng.gen_range(0..=LIST_SIZE-1);

		let q_res = select::quick_select(& list, k);
		let l_res = select::lazy_select(& list, k);

		if q_res != l_res {
			n += 1;
			println!("List {}: QuickSelect and LazySelect results differ for k = {} with q={} and l={}", i, k, q_res, l_res);
		}
	}
	println!("Number of lists where QuickSelect and LazySelect results differ: {}/{}", n, NUM_LISTS);
}