/*
 * Rustの高度な機能（Unsafe Rust）
 * CreatedAt: 2019-07-08
 */
fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}

