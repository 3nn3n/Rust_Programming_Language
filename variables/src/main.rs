fn main() {
    let mut x = 5; //mutable x
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    let y = 7;
    let y = y + 1; //shadowing the previous y variable
    {
        let y = y + 1;
        println!("The value of y inside this local scope is {y}")
    }

    println!("The value of y inside this local scope is {y}")


}
