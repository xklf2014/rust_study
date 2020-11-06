use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::fmt;

#[path = "tuples_test.rs"]
mod t;
#[path = "basic_test.rs"]
mod b;
#[path = "slicesArrays.rs"]
mod sa;
#[path = "custom_types.rs"]
mod ctr;
#[path = "variable_binding.rs"]
mod vb;
#[path = "types.rs"]
mod ts;
#[path = "conversion.rs"]
mod cs;
#[path = "expressions.rs"]
mod eps;
#[path = "flow_of_control.rs"]
mod fic;
#[path = "functions.rs"]
mod fs;
#[path = "modules.rs"]
mod module;
#[path = "generics.rs"]
mod genercis;
#[path = "implementation.rs"]
mod implementation;
#[path = "traits.rs"]
mod traits;
#[path = "bounds.rs"]
mod bounds;
#[path = "spawn.rs"]
mod spw;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

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
    //fic::test_pointer_and_ref();
    //fic::test_structs();
    //fic::test_guard();
    //fic::test_binding();
    //fic::test_if_let();
    //fic::test_while_let();
    //fs::test_function();
    //fs::test_methods();
    //fs::test_closures();
    //fs::test_capturing();
    //fs::test_as_input_parameters();
    //fs::test_anonymity();
    //fs::test_input_function();
    //fs::test_as_output_parameters();
    //fs::test_iterator();
    //fs::test_searching_through_iterators();
    //fs::test_hos();
    //fs::test_driverging_functions();
    //module::test_visibility();
    //module::test_struct_visibility();
    //module::test_use_declaration();
    //module::test_super_and_self();
    //genercis::test_generic();
    //implementation::test_implementation();
    //traits::test_traits();
    //bounds::test_bounds();
    //bounds::test_empty_bounds();

/*    thread::spawn(spw::spawn_function);

    for i in 0..3 {
        println!("main thread print {}",i);
        thread::sleep(Duration::from_millis(1));
    }*/

    /*thread::spawn(|| {
        for i in 0..5 {
            println!("spwan thread print {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..3 {
        println!("main thread print {}",i);
        thread::sleep(Duration::from_millis(1));
    }

    let inc = |num : i32| -> i32{
        num + 1
    };
    println!("inc = {}",inc(5));*/

    /*let handle = thread::spawn(||{
        for i in 0..5 {
            println!("spawned thread print {}",i);
            thread::sleep(Duration::from_millis(1));
        } 
    });

    for i in 0..3 {
        println!("main thread print {}",i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();*/
    /*
       let s = "hello";

        let handle = thread::spawn(||{
            println!("{}",s);
        });

        handle.join().unwrap();
         会报错   ^^ may outlive borrowed value `s`
        */

    let s = "hello";

    let handle = thread::spawn(move || {
        println!("{}",s);
    });

    handle.join().unwrap();

    let (sender , receiver) = mpsc::channel();

    thread::spawn( move || {
       let val = String::from("Hi");
        sender.send(val).unwrap();
    });

    let received = receiver.recv().unwrap();
    println!("{}",received);


}

