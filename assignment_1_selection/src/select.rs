use rand::prelude::SliceRandom;

fn _partition(s: &mut [i32], low: usize, high: usize) -> usize {
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

fn _quick_select(s: &mut [i32], low: usize, high: usize, k: usize) -> i32 {
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

pub fn quick_select(arr: &mut [i32], k: usize) -> i32 {
    // create a list copy so that the original list is not modified
    let mut new_arr = arr.to_vec();
    _quick_select(&mut new_arr, 0, arr.len() - 1, k)
}


pub fn lazy_select<T: Ord + Copy>(s: &mut [T], k: usize) -> T {
    let n = s.len();
    let n_3_4 = (n as f64).powf(0.75) as usize;

    loop {
        // Step 1: Pick n^(3/4) elements randomly with replacement
        let mut rng = rand::thread_rng();
        let mut r: Vec<T> = (0..n_3_4).map(|_| *s.choose(&mut rng).unwrap()).collect();

        // Step 2: Sort R
        r.sort();

        // Step 3: Select a and b
        let x = k as f64 * (n as f64).powf(-0.25);
        let l = (x - (n as f64).sqrt().floor()).max(0.0) as usize;
        let h = (x + (n as f64).sqrt().floor()).min(n_3_4 as f64 - 1f64) as usize;
        let a = r[l];
        let b = r[h];

        let rank_a = s.iter().filter(|&&y| y < a).count();

        // Step 4: Partition S based on a and b
        let mut p: Vec<T> = vec![];
        if k < n_3_4 {
            p = s.iter().filter(|&&y| y <= b).cloned().collect();
        } else if k > n - n_3_4 {
            p = s.iter().filter(|&&y| y >= a).cloned().collect();
        } else {
            p = s.iter().filter(|&&y| a <= y && y <= b).cloned().collect();
        }

        if p.len() <= 4 * n_3_4 + 2 && k - rank_a < p.len() && k - rank_a >= 0 {
            // Step 5: Sort P and find the k-th smallest element
            p.sort();
            return p[k - rank_a];
        }
    }
}
