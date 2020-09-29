use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::fmt;
#[path = "tuples_test.rs"] mod t;
#[path = "basic_test.rs"] mod b;
#[path = "slicesArrays.rs"] mod sa;
#[path = "custom_types.rs"] mod ctr;
#[path = "variable_binding.rs"] mod vb;
#[path = "types.rs"] mod ts;
#[path = "conversion.rs"] mod cs;
#[path = "expressions.rs"] mod eps;
#[path = "flow_of_control.rs"] mod fic;

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
    //vb::test_freezing();
    //ts::test_casting();
    //ts::test_literals();
    //ts::test_inference();
    //ts::test_aliasing();
    //cs::test_from();
    //cs::test_into();
    //cs::test_try_from_and_into();
    //cs::test_convering_to_string();
    //cs::test_parsing_string();
    //eps::test_expressions();
    //fic::test_if_else();
    //fic::test_loop();
    //fic::test_nesting_and_lables();
    //fic::test_returning_from_loops();
    //fic::test_while();
    //fic::test_for_and_range();
    //fic::test_iterators();
    //fic::test_match();
    //fic::test_tuples();
    //fic::test_enum();
    fic::test_pointer_and_ref();
}
