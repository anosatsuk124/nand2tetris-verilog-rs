#[cxx::bridge]
pub mod top {
    unsafe extern "C++" {
        include!("top.cpp");
        type Vtop;
        fn new_top(a: i32, b: i32) -> UniquePtr<Vtop>;
        fn eval(top: &UniquePtr<Vtop>);
        fn get_out(top: &UniquePtr<Vtop>) -> i32;
    }
}
