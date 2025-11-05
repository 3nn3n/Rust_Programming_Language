#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn width(&self) -> bool {
        self.width > 10
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> u32 {
        size * size
    }

    fn rectangle4(width: u32, length: u32) -> u32 {
        length * width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 34,
        length: 43,
    };

    let rect2 = Rectangle {
        width: 24,
        length: 33,
    };

    let rect3 = Rectangle {
        width: 42,
        length: 76,
    };

    let sq = Rectangle::square(7);
    let rect4 = Rectangle::rectangle4(9, 9);



    println!(
        "The area of rectangle is {}",
        rect1.area()
    );

    println!(
        "Is tour width greater than 10: {}",
        rect1.width()
    );

    println!(
        "The width is: {}",
        rect1.width
    );

    println!(
        "Can rect1 hold rect2: {}",
        rect1.can_hold(&rect2)
    );

    println!(
        "Can rect1 hold rect3: {}",
        rect1.can_hold(&rect3)
    );

    println!(
        "The square is: {:?}",
        sq
    );

    println!(
        "The area of the rectangle is: {}",
        rect4
    )



}
