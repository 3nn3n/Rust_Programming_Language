#[derive(Debug)]
enum UsState {
    Alaska, 
    Florida,
    Nebraska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("This quarter is from {:?}", state);
            25
        },
    }
}

fn plus_two(x: Option<u32>) -> Option<u32> {
    match x {
        None => None,
        Some(i) => Some(i + 2),
    }
}

fn main() {
    let value: u8 = value_in_cents(Coin::Dime);
    println!("The value of dime is {}", value);

    let quar_value: u8 = value_in_cents(Coin::Quarter(UsState::Florida));
    println!("The value of quarter is {}", quar_value);

    let grey = Some(9);
    let add_two = plus_two(grey);
    println!(
        "The value is : {:?}",
        add_two
    );

    let none = None;
    println!(
        "{:?}",
        plus_two(none)
    )

}
