use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::fmt;
#[path = "tuples_test.rs"] mod t;
#[path = "basic_test.rs"] mod b;
#[path = "slicesArrays.rs"] mod sa;
#[path = "custom_types.rs"] mod ctr;


fn main() {
    //b::basicTest();
    //t::pTuple();
    //sa::sclies();
    //ctr::struct_test();
    //ctr::enum_test();
    //ctr::test_alias();
    //ctr::c_like_test();
    ctr::linked_list_test();


}
