use crate::ffi::Vnand;
use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    #include "Vnand.h"
    safety!(unsafe_ffi)
    generate_pod!("Vnand")
}
fn main() {
    let mut vnand = Vnand::new("").within_unique_ptr();
    println!("Hello, world!");
}
