#[cfg(test)]
mod tests {
	use crate::select::lazy_select;
	use crate::select::quick_select;

	#[test]
	fn quick_select_finds_kth_smallest_element() {
		let arr = vec![3, 1, 2, 5, 4];
		assert_eq!(quick_select(&arr, 2), 3);
	}

	#[test]
	fn quick_select_single_element() {
		let arr = vec![42];
		assert_eq!(quick_select(&arr, 0), 42);
	}

	#[test]
	fn quick_select_k_is_zero() {
		let arr = vec![10, 20, 30, 40, 50];
		assert_eq!(quick_select(&arr, 0), 10);
	}

	#[test]
	fn quick_select_k_is_last_index() {
		let arr = vec![10, 20, 30, 40, 50];
		assert_eq!(quick_select(&arr, 4), 50);
	}

	#[test]
	fn lazy_select_finds_kth_smallest_element() {
		let arr = vec![3, 1, 2, 5, 4];
		assert_eq!(lazy_select(&arr, 2), 3);
	}

	#[test]
	fn lazy_select_single_element() {
		let arr = vec![42];
		assert_eq!(lazy_select(&arr, 0), 42);
	}

	#[test]
	fn lazy_select_k_is_zero() {
		let arr = vec![10, 20, 30, 40, 50];
		assert_eq!(lazy_select(&arr, 0), 10);
	}

	#[test]
	fn lazy_select_k_is_last_index() {
		let arr = vec![10, 20, 30, 40, 50];
		assert_eq!(lazy_select(&arr, 4), 50);
	}
}