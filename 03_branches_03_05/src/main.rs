fn main() {
    let x = 2;
    if x > 3 {                      //condition need to be a boolean
        println!("Greater than 3");
    } else {
        println!("Lesser than 3");
    }

    divisibility(18);
}

fn divisibility(x: i32) {
    if x % 4 == 0 {
        println!("X is divisible by 4...");
    } else if x % 3 == 0 {
        println!("X is divisible by 3...");
    } else if x % 2 == 0 {
        println!("X is divisible by 2...");
    } else {
        println!("X isn't divisible by 4, 3, 2...");
    }
}