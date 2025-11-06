enum IpAddreKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddreKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddreKind::V4,
    address: String::from("127.0.0.1"),

}

let loopBack = IpAddr {
    kind: IpAddreKind::V6,
    address: String::from("::1")
}

let six = IpAddreKind::V6;

fn Route(ipkind: IpAddreKind) {}

fn main() {

Route(IpAddreKind::V4);
Route(IpAddreKind:V6);

}



// or we can write like

enum IpAddress2 {
    V4(String),
    V6(String),
}

let home1 = IpAddress2::V4(String::from("127.0.0.1"));
let loopBack1 = IpAddress2::V6(String::from("::1"));

//other things...

emum IpAddress3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home2 = IpAddress3::V4(127, 0, 0, 1);
let loopBack2 = IpAddress3::V6("::12");


//few more....

struct GetLost {
    a : String,
    b : u32,
}

struct Hair {
    a : String,
    b: bool,
}

enum Hell0 {
    V4(GetLost),
    V6(Hair),
}


// Another exapmle...

enum Sandesh {
    Quit,
    Move {x : i32, y : i32},
    Write(String),
    Color(i32, i32, i32),
}