use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::fmt;
#[path = "tuples_test.rs"] mod t;
#[path = "basic_test.rs"] mod b;

fn main() {
    //b::basicTest();
    t::pTuple();

}
