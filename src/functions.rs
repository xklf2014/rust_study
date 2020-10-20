use std::iter::Iterator;

pub fn test_function() {
    fizzbuzz_to(100);
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destory(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

pub fn test_methods() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);
    println!("p2.x is {}", square.p2.x);
    println!("p2.y is {}", square.p2.y);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destory();
}

fn function(i: i32) -> i32 { i + 1 }

pub fn test_closures() {
    let closure_annotated = |i: i32| -> i32{ i + 1 };
    let closure_inferred = |i| i + 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated:{}", closure_annotated(i));
    println!("closure_inferred:{}", closure_inferred(i));

    let one = || 1;
    println!("closure return one : {}", one());
}

pub fn test_capturing() {
    use std::mem;

    let color = String::from("green");
    let print = || println!("`color`:{}", color);

    print();

    let _reborrow = &color;
    print();
    //println!("reborrow : {}",_reborrow);

    let _color_moved = color;

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    let _count_borrowed = &mut count;
    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    //println!("{}",movable); //borrow of moved value: `movable`

    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));
}

// Fn: the closure captures by reference (&T)
// FnMut: the closure captures by mutable reference (&mut T)
// FnOnce: the closure captures by value (T)

fn apply<F>(f: F) where
    F: FnOnce() {
    //println!("fnOnce");
    f();
}

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
    //println!("apply_to_3");
    f(3)
}


pub fn test_as_input_parameters() {
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);
        farewell.push_str("!!!");
        println!("then I screamed {}", farewell);
        println!("now I can sleep,zZZZ");
        mem::drop(farewell);
    };

    apply(diary);
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}

fn apply1<F>(f: F) where
    F: FnOnce() {
    f();
}

fn apply2<F>(f: F) where
    F: Fn() {
    f();
}

pub fn test_anonymity() {
    let x = 7;
    let print = || println!("{}", x);
    apply1(print);
    apply2(print);
}

fn call_me<F: Fn()>(f: F) {
    f();
}

fn function1() {
    println!("I'm a funciton!");
}

pub fn test_input_function() {
    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(function1);
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a : {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("this is a : {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("this is a : {}", text)
}

pub fn test_as_output_parameters() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}

/*pub trait Iterator {
    type Item;

    fn any<F>(&mut self, f: F) -> bool where
        F: FnMut(Self::Item) -> bool {}
}*/

pub fn test_iterator() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}

pub fn test_searching_through_iterators() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));

    let vec = vec![1, 9, 3, 3, 13, 2];
    let index_of_first_even_number = vec.iter().position(|x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));
    let index_of_first_negative_number = vec.iter().position(|x| x < &0);
    assert_eq!(index_of_first_negative_number, None);
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

//Higher Order Functions
pub fn test_hos() {
    println!("find the sum of all the squared odd number under 1000");
    let upper = 1000;
    let mut acc = 0;

    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
        //println!("n_squared:{}, and acc is : {}",n_squared,acc);
    }
    println!("imperative style: {}", acc);
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)
            .take_while(|&n_squared| n_squared < upper)
            .filter(|&n_squared| is_odd(n_squared))
            .fold(0, |acc, n_squared| acc + n_squared);
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

fn foo() -> ! {
    panic!("this call never returns!");
}

fn some_fn(){
    ()
}

pub fn test_driverging_functions(){
    let a :() = some_fn();
    println!("this function returns and you can see this line.");

    fn sum_odd_numbers(up_to : u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1{
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }

    println!("Sum of odd numbers up to 9 (excluding): {}",sum_odd_numbers(9));
}

