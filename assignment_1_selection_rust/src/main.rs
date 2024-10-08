mod select;

fn main() {
    let mut arr = vec![12, 3, 5, 7, 19, 26, 1, 18];
    let k = 3;
    let result = select::quick_select(&mut arr, k);
    println!("The {}-th smallest element is {}", k, result);
}
