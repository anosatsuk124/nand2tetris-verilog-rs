use bin_rs::ffi::top;

fn main() {
    let top = top::new_top(1, 0);
    top::eval(&top);
    println!("out: {}", convert_to_boolean(top::get_out(&top)));
}

fn convert_to_boolean(v: i32) -> bool {
    v != 0
}
