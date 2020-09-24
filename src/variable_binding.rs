pub fn test_let() {
    let an_integer = 1u32;
    let a_bool = true;
    let unit = ();

    let copy_integer = an_integer;

    println!("An integer: {:?}", copy_integer);
    println!("A boolean: {:?}", a_bool);
    println!("meet the unit value: {:?}", unit);

    let un_used_variable = 3u32;
    let noisy_unused_variable = 2u32;
}

pub fn test_mutability() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);
    //_immutable_binding += 1;  //Cannot assign twice to immutable variable [E0384]
}

pub fn test_scope_and_shadowing() {
    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }

    //println!("outer short: {}",short_lived_binding); //cannot find value `short_lived_binding` in this scope
    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}

pub fn test_declare_first() {
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a_binding : {}", a_binding);

    let another_binding;
    //println!("another binding: {}", another_binding);//use of possibly-uninitialized `another_binding`
    another_binding = 1;
    println!("another binding: {}", another_binding);
}

pub fn test_freezing(){
    let mut _mutable_integer = 7i32;
    {
        let _mutable_integer = _mutable_integer;
        //_mutable_integer = 50; //Cannot assign twice to immutable variable
    }

    _mutable_integer = 10;
    println!("mutable integer : {}",_mutable_integer);
}