fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //　定数
    const MAX_POINTS: u32 = 100_000;

    // シャドーイング
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of z in the inner scope is: {}", y);
    }

    println!("The value of x is: {}", y);

    let spaces = "       ";
    let spaces = spaces.len();

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    // println!("The value of z is: {}", z);
    println!("The value of tup.0 is: {}", tup.0);
    println!("The value of tup.1 is: {}", tup.1);

    let a = [0, 1, 2, 3, 4, 5, 6];
    println!("The value of a[0] is: {}", a[0]);

    let three = [3; 5]; // [3,3,3,3,3]

    another_function(x, 'h');

    let x = {
        let y = 3;
        y + 1
    };
    println!("The value of x is: {}", x);

    let x = five();
    println!("The value of x is: {}", x);

    let number = 3;
    if number < 5 {
        println!("true!");
    } else if number % 3 == 0 {
        println!("else if!!!");
    } else {
        println!("false");
    }

    // loop {
    //     println!("again");
    // }

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    for element in a {
        println!("value is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32, y: char) {
    println!("Hello, another function!");
    println!("Another function value is: {} {}", x, y);
}

fn five() -> i32 {
    5
}
