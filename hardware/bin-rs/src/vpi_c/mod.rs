#[no_mangle]
pub extern "C" fn println(num: u8) {
    println!("{}", num);
}
