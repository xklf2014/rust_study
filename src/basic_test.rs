pub fn basicTest(){
    //println!("Hello, world!");
    /*    let stdout = stdout();
        let message = String::from("Hello my first rust");
        let width =message.chars().count();

        let mut writer = BufWriter::new(stdout.lock());
        say(message.as_bytes(),width,&mut writer).unwrap();*/

    println!("hello world");

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
    //print!不会换行
    print!("test print");
    print!("second content");
    println!();
    //执行顺序优先于print
    eprint!("error print");
    eprint!("second content");

    eprintln!("error print,我会换行");

    println!("{} days", 31);
    //{0}可以表示取第一个变量
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    //{object}也可以给变量起名，用{var name}替换变量值
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over");
    //{:b}可以将数字格式化位binary,即0、1
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    //输入内容前显示5位空格
    println!("{number:>width$}", number = 1, width = 6);
    //占位符也可以用0代替
    println!("{number:>0width$}", number = 1, width = 6);
    //预留2个变量位置，但是没有全部赋值编译会报右边的错误：invalid reference to positional argument 1
    //println!("My name is {0}, {1} {0}", "Bond");

    //struct Structure(i32);//类似java的class
    //println!("This struct `{}` won't print...", Structure(3));

    //运算符练习
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    println!("true and false is {}", true && false);
    println!("true or false is {}", true || false);
    //二进制与，或运算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    //移位运算
    // 00000001
    // 00100000
    println!("1 << 5 is {}", 1u32 << 5);
    /**
    十进制(Decimal)	98_222
    十六进制(Hex)	0xff
    八进制(Octal)	0o77
    二进制(Binary)	0b1111_0000
    字节(Byte) (u8 only)	b'A'
    */
    println!("0x80 >> 2 is {}", 0x80u32 >> 2);
    //利用下划线提高可读性
    println!("One million is written as {}", 1_000_000u32);
}