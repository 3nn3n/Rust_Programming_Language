fn main() {
    let mut s = String::from("Hello");
    s.push_str(", World...!");
    println!("{s}");
    let y = s;
    println!("{y}");

    
    let x = 3;
    let z = x;
    println!("{z}");

    let mut a = String::from("Hey, how are you?");
    a.push_str("...I am fine...");
    take_ownership(a);


    let b = 4;
    make_copy(b);

    let s1 = give_ownership();
    println!("{s1}");

    let s2 = String::from("Taking the ownership and giving back");
    let s3 = take_and_give_back(s2);
    println!("{s3}");

    let s4 = String::from("Nitesh");
    let (s5, len) = calculate_length(s4);
    println!("The length of {s5} is {}", len);
}

fn take_ownership(some_string: String) {
    println!("{some_string}")
}

fn make_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn give_ownership() -> String {
    let some_string = String::from("Yours Nitesh...");
    some_string
}

fn take_and_give_back(some_string: String) -> String {
    some_string
}


fn calculate_length(a_string: String) -> (String, usize) {
    let length: usize = a_string.len();
    (a_string, length)
}