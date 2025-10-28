fn main() {
    println!("Hello, world!");
    second_function(6);
    converter(273, "Celcius".to_string());
    nine(6);
    let a = add(7);
    println!("The value of a is {a}")
}

fn second_function(x: i32) {
    println!("Second function has a parameter x which has a value {x}")
}

fn converter(value: i32, unit: String) -> () {
    let a = println!("{value} kelvin in 0 {unit}");
    a
}

fn nine (x: i32) -> () {
    let a = println!("The x is {x}");  //statement
    a           //expression
}

fn add(x: i32) -> i32 {
    x + 1               //expression
}