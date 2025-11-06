use std::fmt::format;
#[derive(Debug)]
enum UsState {
    Alabama,
    Utah,
    Nebraska,
    Ohio,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u32) -> bool {
        match self {
            UsState::Alabama => year >= 1890,
            UsState::Nebraska => year >= 1825,
            UsState::Ohio => year >= 1911,
            UsState::Utah => year >=1923,
        }
    }
}

fn describe_state_qrtr(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return  None;
    };

    if state.existed_in(1890) {
        Some(format!("{state:?} is old state..."))
    } else {
        Some(format!("{state:?} is new state"))
    }
}




fn main() {
    let abc = describe_state_qrtr(Coin::Quarter(UsState::Ohio));
    let rst = describe_state_qrtr(Coin::Quarter(UsState::Alabama));
    let asbc = describe_state_qrtr(Coin::Quarter(UsState::Nebraska));
    let aasdbc = describe_state_qrtr(Coin::Quarter(UsState::Utah));
    println!("{:?}", abc);
    println!("{:?}", rst);
    println!("{:?}", asbc);
    println!("{:?}", aasdbc);
}