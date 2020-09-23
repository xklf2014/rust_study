#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

pub fn pTuple() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
    println!("long tuple first value: {}", long_tuple.0);
    //tuple里可以嵌套tuple
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    //println！可以打印12个元素的tuple，超过就会报错`cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,13);
    //println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{},{},{},{}",a,b,c,d);

    let matrix = Matrix(1.1,1.2,2.1,2.2);
    println!("{:?}",matrix);
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}