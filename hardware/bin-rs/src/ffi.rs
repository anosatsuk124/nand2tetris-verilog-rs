#[cxx::bridge]
pub mod top {
    unsafe extern "C++" {
        include!("top.h");
        include!("Vtop.h");
        type Vtop;
        fn new_top(a: i32, b: i32) -> UniquePtr<Vtop>;
        fn eval_top(top: &UniquePtr<Vtop>);
        fn get_out(top: &UniquePtr<Vtop>) -> i32;
    }
}
