pub fn test_if_else() {
    let n = -50;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            n * 10
        } else {
            println!(", and is a big number, halve the number");
            n / 2
        };

    println!("{} -> {}", n, big_n);
}

pub fn test_loop() {
    let mut count = 0u32;
    println!("Let's count until infinity!");

    loop {
        count += 1;
        println!("count value is {}", count);
        if count == 3 {
            continue;
        }
        if count == 5 {
            println!("OK,that's enough");
            break;
        }
    }
}

#[allow(unreachable_code)]
pub fn test_nesting_and_lables() {
    'outer: loop {
        println!("entered the outer loop");

        'inner: loop {
            println!("entered the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("exited the outer loop");
}

pub fn test_returning_from_loops() {
    let mut count = 0;
    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2;
        }
    };

    assert_eq!(result, 20);
}

pub fn test_while() {
    let mut n = 1;

    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}

pub fn test_for_and_range() {
    for n in 1..31 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
    println!("----------------------");
    for n in 1..=31 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

pub fn test_iterators() {
    let names = vec!["bob", "frank", "ferris"];
    /** iter
    This borrows each element of the collection through each iteration.
    Thus leaving the collection untouched and available for reuse after the loop.
    */
    for name in names.iter() {//此处为元素借用，循环体用完会归还，可以继续使用
        match name {
            &"frank" => println!("There is a rustacean among us!"),
            _ => println!("hello {}", name),
        }
    }
    println!("{}", names[0]);
    /** into_iter
    This consumes the collection so that on each iteration the exact data is provided.
    Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
    */
    for name in names.into_iter() {
        match name {
            "frank" => println!("There is a rustacean among us!"),
            _ => println!("hello {}", name),
        }
    }
    //println!("{}",names[0]); //borrow of moved value: `names`

    let mut words = vec!["hello", "hi", "fine"];

    /** iter_mut
    This mutably borrows each element of the collection,
    allowing for the collection to be modified in place
    */
    for word in words.iter_mut() {
        *word = match word {
            &mut "fine" => "Fine!Thank you,and you",
            _ => "Hello world",
        }
    }
    println!("words: {:?}", words);
}

pub fn test_match() {
    let number = 13;
    println!("tell me about {}", number);

    match number {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("this is a prime"),
        13..=19 => println!("a teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}

pub fn test_tuples() {
    let pair = (1, 0);
    println!("tell me about {:?}", pair);

    match pair {
        (0, y) => println!("First is `0`,and `y` is {}", y),
        (x, 0) => println!("`x`is {:?} and last is `0`", x),
        _ => println!("it doesn't matter what they are"),
    }
}

enum Color {
    Red,
    Green,
    Blue,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

pub fn test_enum() {
    //let color = Color::RGB(122, 17, 40);
    let color = Color::CMYK(100, 20, 50, 33);
    println!("what color is it");

    match color {
        Color::Red => println!("this color is red"),
        Color::Green => println!("this color is green"),
        Color::Blue => println!("this color is blue"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, Saturation: {}, and Value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, Saturation: {}, and Lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, Magenta: {}, and Yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!("Cyan: {}, Magenta: {}, and Yellow: {},Key(black): {}", c, m, y, k),
    }
}

pub fn test_pointer_and_ref() {
    let refernece = &4;

    match refernece {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *refernece {
        val => println!{ "Got a value via dereferencing: {:?}", val }
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;
    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r =>  println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m =>{
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }

}