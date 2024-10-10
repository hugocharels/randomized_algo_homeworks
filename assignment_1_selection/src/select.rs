use rand::prelude::SliceRandom;

fn _partition<T: Ord + Copy>(s: &mut [T], low: usize, high: usize) -> usize {
	let pivot = s[high];
	let mut i = low;
	for j in low..high {
		if s[j] <= pivot {
			s.swap(i, j);
			i += 1;
		}
	}
	s.swap(i, high);
	i
}

fn _quick_select<T: Ord + Copy>(s: &mut [T], low: usize, high: usize, k: usize) -> T {
	if low == high {
		return s[low];
	}
	let pivot_index = _partition(s, low, high);
	if k == pivot_index {
		s[k]
	} else if k < pivot_index {
		_quick_select(s, low, pivot_index - 1, k)
	} else {
		_quick_select(s, pivot_index + 1, high, k)
	}
}

pub fn quick_select<T: Ord + Copy>(arr: &[T], k: usize) -> T {
	// create a list copy so that the original list is not modified
	let mut new_arr = arr.to_vec();
	_quick_select(&mut new_arr, 0, arr.len() - 1, k)
}


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

		let rank_a = s.iter().filter(|&&y| y < a).count();
		let rank_b = s.iter().filter(|&&y| y < b).count();

		if k == rank_a { return a;}
		else if k == rank_b { return b; }

		if rank_a > k || rank_b < k { continue; }

		// Step 4: Partition S based on a and b
		let mut p: Vec<T> = if k < n_1_4 {
			s.iter().filter(|&&y| y <= b).cloned().collect()
		} else if k > n - n_1_4 {
			s.iter().filter(|&&y| y >= a).cloned().collect()
		} else {
			s.iter().filter(|&&y| a <= y && y <= b).cloned().collect()
		};

		if p.len() <= 4 * n_3_4 + 2 {
			// Step 5: Sort P and find the k-th smallest element
			p.sort();
			if k < n_1_4 {
				return p[k];
			}
			return p[k - rank_a];
		}
	}
}
