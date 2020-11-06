/*fn printer<T: Display>(t: T) {
    println!("{}", t);
}*/

use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }

#[allow(dead_code)]
struct Triangle { length: f64, height: f64 }

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

pub fn test_bounds() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle { length: 3.0, height: 4.0 };
    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));
}

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red{}
trait Blue{}

impl Red for Cardinal{}
impl Blue for BlueJay{}
fn red<T : Red>(_: &T) -> &'static str {"red"}
fn blue<T : Blue>(_: &T) -> &'static str {"blue"}

pub fn test_empty_bounds(){
    let cardinal = Cardinal;
    let bluejay = BlueJay;
    let turkey = Turkey;
    println!("A cardinal is {}",red(&cardinal));
    println!("A bluejay is {}",blue(&bluejay));
    //println!("A turkey is {}",red(&turkey));
}