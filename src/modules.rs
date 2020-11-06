mod my_mod {
    fn private_function() {
        println!("called `my_mod::private_function`");
    }

    pub fn function() {
        println!("called `my_mod::function`");
    }

    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`,that\n>");
        private_function();
    }

    pub mod nested{
        pub fn function(){
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function(){
            println!("called `my_mod::nested::private_function()`");
        }

        pub fn public_function_in_my_mod(){
            print!("called `my_mod::nested::public_function_in_my_mod()`,that\n>");
            public_function_in_nested();
        }

        pub(self) fn public_function_in_nested(){
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        pub(super) fn public_function_in_super_mod(){
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod(){
        print!("called `my_mod::call_public_function_in_my_mod()`,that\n>");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    pub(crate) fn public_function_in_crate(){
        println!("called `my_mod::public_function_in_crate()`");
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function(){
            println!("called `my_mod::private_nested::function()`");
        }

        #[allow(dead_code)]
        pub(crate) fn restricted_function(){
            println!("called `my_mod::private_nested::restricted_function()`")
        }
    }
}

fn function(){
    println!("called `function()`");
}

pub fn test_visibility(){
    function();
    my_mod::function();
    // Public items, including those inside nested modules, can be  accessed from outside the parent module.
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();
    //my_mod::nested::public_function_in_my_mod();  //failed to resolve: unresolved import
}

mod my {
    pub struct OpenBox<T>{
        pub contents:T,
    }

    #[allow(dead_code)]
    pub struct CloseBox<T>{
        contents:T,
    }

    impl<T> CloseBox<T>{
        pub fn new(contents:T) -> CloseBox<T>{
            CloseBox{
                contents:contents,
            }
        }
    }
}

pub fn test_struct_visibility(){
    let open_box = my::OpenBox { contents : "public information"};
    println!("the open box contains: {}",open_box.contents);
    let _close_box = my::CloseBox::new("classfiled information");
    //println!("The close box contains: {}",_close_box); //`module::modules.my2::CloseBox<&str>` doesn't implement `std::fmt::Display`
}


pub mod deeply {
    pub mod nested{
        pub fn function(){
            println!("called `deeply::nested::function()`");
        }
    }
}

use deeply::nested::function as other_function;

pub fn test_use_declaration(){
    other_function();

    println!("Entering block");
    {
        use deeply::nested::function as other_function1;
        other_function1();
        println!("Leaving block");
    }
    function();
}

pub mod cool {
    pub fn function(){
        println!("called `cool::function()`");
    }
}

mod my1 {

    fn function(){
        println!("called `modules.my2::function()`");
    }

    mod cool {
        pub fn function(){
            println!("called `modules.my2::cool::function()`");
        }
    }

    pub fn indirect_call(){
        print!("called `modules.my2::indirect_call`,that \n>");
        self::function();
        function();
        self::cool::function();
        super::function();

        {
            use cool::function as root_function;
            root_function();
        }

    }
}

pub fn test_super_and_self(){
    my1::indirect_call();
}

/*
fn function1(){
    println!("called `function1()`");
}

pub fn test_file_hierarchy(){
    my::function();
    function1();
    my::indirect_access();
    my::nested::function();
}*/

extern crate ferris_says;