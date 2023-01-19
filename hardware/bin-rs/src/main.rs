use bin_rs::ffi::top;

fn main() {
    let top = top::new_top(1, 1);
    top::eval_top(&top);
    println!("out: {}", top::get_out(&top));
}
