// normal struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple struct
struct Point(i32, i32, i32);
struct Color(i32, i32, i32);

// empty struct
struct AlwaysEqual;

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@user.com"),
        sign_in_count: 0,
    };

    println!("user1 username: {}", user1.username);
    user1.username = String::from("haha");
    println!("user1 username: {}", user1.username);

    let mut user2 = build_user(String::from("user2"), String::from("user2@user.com"));
    println!("user2 username: {}", user2.username);
    user2.username = String::from("???");
    println!("user2 username: {}", user2.username);

    // let user3 = User {
    //     active: true,
    //     username: user1.username,
    //     email: String::from("3@user.com"),
    //     sign_in_count: 0,
    // };
    // or:
    let user3 = User {
        email: String::from("3@user.com"),
        ..user1
    };

    println!("user3 username: {}", user3.username);
    // will not work, user3 struct now owns the user1.username string.
    // println!("user1 username: {}", user1.username);
    println!("user1 sign in count: {}", user1.sign_in_count);
    println!("user1 active: {}", user1.active);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
