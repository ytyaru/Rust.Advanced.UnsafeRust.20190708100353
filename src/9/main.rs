/*
 * Rustの高度な機能（Unsafe Rust）
 * CreatedAt: 2019-07-08
 */
#[no_mangle]
pub extern "C" fn call_from_c() {
     // CからRust関数を呼び出したばかり！
    println!("Just called a Rust function from C!");
}
fn main() {}
