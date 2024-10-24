pub fn lazy_select<T: Ord + Copy>(s: &[T], k: usize) -> T {
	let n = s.len();
	let n_3_4 = (n as f64).powf(0.75) as usize;
	let n_1_4 = (n as f64).powf(0.25) as usize;
	loop {
		// Step 1: Pick n^(3/4) elements randomly with replacement
		let mut rng = rand::thread_rng();
		let mut r: Vec<T> = (0..n_3_4).map(|_| *s.choose(&mut rng).unwrap()).collect();

		// Step 2: Sort R
		r.sort();

		// Step 3: Select a and b
		let x = k as f64 * (n as f64).powf(-0.25);
		let l = (x - (n as f64).sqrt().floor()).max(1.0) as usize;
		let h = (x + (n as f64).sqrt().ceil()).min(n_3_4 as f64) as usize;
		let a = r[l-1];
		let b = r[h-1];

		// Step 4: Partition S based on a and b and find the rank of a
		let mut p: Vec<T> = Vec::new();
		let index;

		match k {
			_ if k < n_1_4 => {
				p = s.iter().filter(|&&y| y <= b).cloned().collect();
				index = k;
			},
			_ if k > n - n_1_4 => {
				p = s.iter().filter(|&&y| y >= a).cloned().collect();
				index = k - (n - p.len()); // rank of a is equal to n - p.len()
			},
			_ => {
				let mut rank_a = 0;
				for &y in s.iter() {
					if y < a {
						rank_a += 1;
					} else if y <= b {
						p.push(y);
					}
				}
				if rank_a > k { continue; }
				index = k - rank_a;
			}
		}

		if p.len() <= 4 * n_3_4 + 2 && index < p.len() {
			// Step 5: Sort P and find the k-th smallest element
			p.sort();
			return p[index];
		}
	}
}