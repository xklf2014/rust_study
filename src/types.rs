pub fn test_casting() {
    let decimal = 65.4321_f32;
    //let integer : u8 = decimal; //expected `u8`, found `f32`
    let integer = decimal as u8;
    let character = integer as char;
    //let character = decimal as char; //only `u8` can be cast as `char`, not `f32`
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);

    //println!("1000 as a u8 is: {}", 1000 as u8); error: literal out of range for `u8`
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    println!("1000 mod 256 is: {}", 1000 % 256);

    println!("128 as a i16: {}", 128 as i16);
    //println!("128 as a i8: {}",128 as i8); //error: literal out of range for `i8`
}

//If no constraint exists, the compiler will use i32 for integers, and f64 for floating-point numbers.
pub fn test_literals() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 0;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

pub fn test_inference() {
    let elem = 5u8;
    let right = true;
    let wrong = false;
    let mut vec = Vec::new();

    vec.push(elem);
    vec.push(right as u8);
    vec.push(wrong as u8);
    println!("{:?}", vec);
}


type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;


pub fn test_aliasing() {
    let nanosconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!("{} nanosecond + {} inches = {} unit?", nanosconds, inches, nanosconds + inches);
}