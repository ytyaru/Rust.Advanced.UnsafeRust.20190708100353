/*
 * Rustの高度な機能（Unsafe Rust）
 * CreatedAt: 2019-07-08
 */
unsafe fn dangerous() {}
fn main() {
//    dangerous(); // error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
    unsafe {
        dangerous();
    }
}

