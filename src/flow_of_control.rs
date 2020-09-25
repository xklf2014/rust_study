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

    assert_eq!(result,20);
}