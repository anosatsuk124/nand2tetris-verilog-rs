use crate::top::eval_top;

#[cxx::bridge]
mod top {
    unsafe extern "C++" {
        include!("top.h");
        include!("Vtop.h");
        type Vtop;
        fn new_top() -> UniquePtr<Vtop>;
        fn eval_top(top: UniquePtr<Vtop>);
    }
}

fn main() {
    let top = top::new_top();
    eval_top(top);
    println!("Hello, world!");
}
