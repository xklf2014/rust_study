use crate::ctr::List::{Nil, Cons};

/**
Rust custom data types are formed mainly through the two keywords:

struct: define a structure
enum: define an enumeration
Constants can also be created via the const and static keywords.
*/
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn struct_test() {
    let name = "Kitty";
    let age = 27;
    let kitty = Person { name, age };

    println!("{:?}", kitty);

    let point: Point = Point { x: 10.5, y: 20.5 };
    println!("point coordinates : ({},{})", point.x, point.y);

    let bottom_right = Point { x: 5.0, ..point };
    println!("second point ({},{})", bottom_right.x, bottom_right.y);

    let Point { x: top_edge, y: left_edge } = point;
    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    println!("_rectangle {:?}", _rectangle);

    let _unit = Unit;
    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}

pub enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

pub fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={},y={}.", x, y);
        }
    }
}

pub fn enum_test() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("modules.my2 text".to_owned());
    let click = WebEvent::Click { x: 20, y: 40 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

//在impl块内可以使用&self代替VeryVerboseEnumOfThingsToDoWithNumbers
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

pub fn test_alias() {
    let x = Operations::Add;
    let y = Operations::Subtract;

    println!("x={:?},y={:?}", x, y);
}

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}
/*
pub fn enum_use_test() {

    let status = Status::Poor;
    let work = Work::Civilian;

    match status {
        Status::Rich => println!("The rich have lots of money"),
        Status::Poor => println!("The poor have no money"),
    }

    match work {
        Work::Civilian => println!("Civilians work"),
        Work::Soldier => println!("Soldier fighting"),
    }
}*/

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub fn c_like_test() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}


enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{},{}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

pub fn linked_list_test() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

static LANGUAGE: &str = "RUST";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

pub fn constant_test() {
    let n = 6;
    println!("this is {}", LANGUAGE);
    println!("the threshold is {}", THRESHOLD);
    println!("{} is {}",n,if is_big(n){"big"}else{"small"});
    //THRESHOLD = 5; //cannot assign to this expression
}

