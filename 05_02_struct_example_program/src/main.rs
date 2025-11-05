struct Rectangle {
    width: u32,
    length: u32,
}

#[derive(Debug)]

struct Cuboid1 {
    length: u32,
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct RectangleDebug {
    width: u32,
    length: u32,
}



fn main() {
    let width = 20;
    let length = 30;

    let rect1 = (70, 90);

    let cuboid = (98, 45, 73);

    println!("The area of the rectangle is: {}", 
    area(width, length));

    println!("The area of rectangle by tuple method is: {}",
    area1(rect1));

    println!("The volume of the cube is: {}",
    volume(cuboid));


    let rect3 = Rectangle {
        width: 50,
        length: 90,
    };

    println!("The area of rectangle using struct is: {}",
    area3(&rect3));

    let cuboid4 = Cuboid1 {
        length: 67,
        width: 43,
        height: 91,
    };

    println!("The volume of cuboid is: {}",
    volume2(&cuboid4));

    println!("cuboid4 is {:#?}", cuboid4);


    let scale = 9;
    let rect_12 = RectangleDebug {
        width: dbg!(30 * scale),
        length: 90,
    };

    dbg!(&rect_12);

}

fn area(width: u32, length: u32) -> u32 {
    width * length
}

fn area1(dimesnsions: (u32, u32)) -> u32 {
    dimesnsions.0 * dimesnsions.1
}

fn volume(dimesnsions: (u32, u32, u32)) -> u32 {
    dimesnsions.0 * dimesnsions.1 * dimesnsions.2
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.length
}

fn volume2(dimesnsions: &Cuboid1) -> u32 {
    dimesnsions.length * dimesnsions.width * dimesnsions.height
}
