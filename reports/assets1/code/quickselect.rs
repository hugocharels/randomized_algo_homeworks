pub fn quick_select<T: Ord + Copy>(s: &[T], k: usize) -> T {
	// create a list copy so that the original list is not modified
	let mut new_s = s.to_vec();
	_quick_select(&mut new_s, 0, s.len() - 1, k)
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