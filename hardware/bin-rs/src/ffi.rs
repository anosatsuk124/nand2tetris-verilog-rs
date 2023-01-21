#[cxx::bridge]
pub mod top {
    unsafe extern "C++" {
        include!("TOP.cpp");
        type VTOP;
        fn new_top(a: i32, b: i32) -> UniquePtr<VTOP>;
        fn eval(top: &UniquePtr<VTOP>);
        fn get_out(top: &UniquePtr<VTOP>) -> i32;
    }
}
