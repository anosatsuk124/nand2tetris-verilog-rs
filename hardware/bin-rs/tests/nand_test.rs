use bin_rs::ffi::top;

#[test]
fn nand_test() {
    let top = top::new_top(1, 0);
    top::eval(&top);
    assert_eq!(top::get_out(&top), 1);
}
