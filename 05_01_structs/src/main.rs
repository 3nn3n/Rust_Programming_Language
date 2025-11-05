struct User {
    active: bool,
    username: String,   
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive (Debug)]

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("someone_new@example.com");
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Sign in count: {}", user1.sign_in_count);
    println!("Active: {}", user1.active);

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    let user2 = build_user(String::from("nick@example.com"), String::from("nick123"));
    println!("Username: {}", user2.username);

    let user3 = User {
        email: String::from("jane@example.com"),
        ..user1
    };
    println!("Username: {}", user3.username);
    println!("Email: {}", user1.email);


    let black = Color(3, 11, 82);
    let origin = Point(5, 7, 9);
    let Color(r, g, b) = black;
    println!("Red: {}, Green: {}, Blue: {}", r, g, b);
    let Point(x, y, z) = origin;
    println!("X: {}, Y: {}, Z: {}", x, y, z);
    println!("value: {}", black.0);
    println!("value2: {}", origin.1);

    let globe = AlwaysEqual;
    println!("{:?}", globe);

}