use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::fmt;
#[path = "tuples_test.rs"] mod t;
#[path = "basic_test.rs"] mod b;
#[path = "slicesArrays.rs"] mod sa;
#[path = "custom_types.rs"] mod ctr;
#[path = "variable_binding.rs"] mod vb;


fn main() {
    //b::basicTest();
    //t::pTuple();
    //sa::sclies();
    //ctr::struct_test();
    //ctr::enum_test();
    //ctr::test_alias();
    //ctr::c_like_test();
    //ctr::linked_list_test();
    //ctr::constant_test();
    //vb::test_let();
    //vb::test_mutability();
    //vb::test_scope_and_shadowing();
    //vb::test_declare_first();
    vb::test_freezing();

}
