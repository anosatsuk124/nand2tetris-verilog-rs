#[no_mangle]
pub extern "C" fn println(num: bool) {
    println!("{}", num);
}
