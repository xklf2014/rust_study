use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

pub fn test_from() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("{}", my_string);

    let num = Number::from(30);
    println!("My number is: {:?}", num);
}

pub fn test_into() {
    let int = 5;
    let num: Number = int.into();
    println!("My number is : {:?}", num);
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

pub fn test_try_from_and_into() {
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

struct Cricle {
    radius: i32,
}

impl fmt::Display for Cricle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

pub fn test_convering_to_string() {
    let circle = Cricle { radius: 6 };
    println!("{}",circle.to_string());
}

pub fn test_parsing_string(){
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum : {:?}",sum);
}