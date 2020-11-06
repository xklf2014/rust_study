mod inaccessible;
pub mod nested;

pub fn function(){
    println!("called `modules.my2::function()`");
}

fn private_function(){
    println!("called `modules.my2::private_function()`");
}

pub fn indirect_access(){
    print!("called `modules.my2::indirect_access()`, that\n> ");
    private_function();
}