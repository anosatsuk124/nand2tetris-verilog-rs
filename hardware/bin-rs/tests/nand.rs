#![cfg(top = "nand")]

#[cxx::bridge]
mod nand {
    unsafe extern "C++" {
        include!("NAND.cpp");
        type VTOP;
        fn new_nand(a: i32, b: i32) -> UniquePtr<VTOP>;
        fn eval(top: &UniquePtr<VTOP>);
        fn get_out(top: &UniquePtr<VTOP>) -> i32;
    }
}

#[test]
fn nand_test() {
    let nand = nand::new_nand(1, 0);
    nand::eval(&nand);
    assert_eq!(nand::get_out(&nand), 1);
}
