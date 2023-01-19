use std::ffi::c_int;
mod vpi_c;

#[cxx::bridge]
mod top {
    unsafe extern "C++" {
        include!("top.h");
        include!("Vtop.h");
        type Vtop;
        fn new_top(a: i32, b: i32) -> UniquePtr<Vtop>;
        fn eval_top(top: &UniquePtr<Vtop>);
        fn get_out(top: &UniquePtr<Vtop>) -> i32;
    }
}

fn main() {
    let top = top::new_top(1, 1);
    top::eval_top(&top);
    println!("out: {}", top::get_out(&top));
}

#[test]
fn nand_test() {
    let top = top::new_top(1, 0);
    top::eval_top(&top);
    assert_eq!(top::get_out(&top), 1);
}
