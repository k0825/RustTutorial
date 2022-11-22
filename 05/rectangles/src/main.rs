#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}

struct Rectangle2 {
    width: u64,
    height: u64,
}

impl Rectangle2 {
    fn area(&self) -> u64 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle2) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    let rect2 = Rectangle2 {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    println!("{:?}", rect1);

    let rect3 = Rectangle2 {
        width: 30,
        height: 50,
    };
    let rect4 = Rectangle2 {
        width: 10,
        height: 40,
    };
    let rect5 = Rectangle2 {
        width: 60,
        height: 45,
    };

    println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));
    println!("Can rect3 hold rect5? {}", rect4.can_hold(&rect5));
}

fn area(dimensions: &Rectangle) -> u64 {
    dimensions.width * dimensions.height
}
