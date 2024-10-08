use rand::Rng;

pub fn quick_select(arr: &mut [i32], k: usize) -> i32 {
    let len = arr.len();
    if len == 1 {
        return arr[0];
    }

    let pivot_idx = rand::thread_rng().gen_range(0..len);
    let pivot = arr[pivot_idx];

    let (left, right): (Vec<i32>, Vec<i32>) = arr.iter().cloned().partition(|&x| x < pivot);
    let left_len = left.len();

    if k < left_len {
        quick_select(&mut left.into_boxed_slice(), k)
    } else if k > left_len {
        quick_select(&mut right.into_boxed_slice(), k - left_len - 1)
    } else {
        pivot
    }
}



