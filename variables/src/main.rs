use std::io;

fn main() {
    let mut x = 5; //mutable x
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    let y = 7;
    let y = y + 1; //shadowing the previous y variable
    {
        let y = y + 1;
        println!("The value of y inside this local scope is {y}");
    }

    println!("The value of y inside this local scope is {y}");
    array1();


}


fn array1(){
    let a: [i32; 7] = [4, 7, 22, 1, 43, 32, 12];
    println!("Please enter an array index...");

    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line...");

    let index: usize = index
        .trim()
        .parse()
        .expect("Not a number...");

    let element = a[index];
    println!("{element} is found at {index}th position");

}
