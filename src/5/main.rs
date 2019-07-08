/*
 * Rustの高度な機能（Unsafe Rust）
 * CreatedAt: 2019-07-08
 */
fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len);
    (&mut slice[..mid], // error[E0499]: cannot borrow `*slice` as mutable more than once at a time
     &mut slice[mid..])
}
