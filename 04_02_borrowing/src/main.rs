fn main() {
    let mut s1 = String::from("Nitesh Negi");
    let s2 = calculate_length(&s1);
    println!("The size of string is {}", s2);
    change(&mut s1);
    println!("{s1}");

    let mut r = String::from("Hello");
    let r1 = &r;
    let r2 = &r;
    println!("{} and {}", r1, r2);
    println!("{} and {}", r1, r2);


    let r3 = &mut r;
    println!("{r3}");

    let reference = dangle();
    println!("{}", reference);
}

fn calculate_length(get_string: &String) -> usize {
    let length = get_string.len();
    length
}

fn change(new_string: &mut String) {
    new_string.push_str("...I am fine...");
}

fn dangle() -> String {
    let s = String::from("Nitesh");
    s
}