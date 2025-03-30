#![allow(dead_code, unused_variables)]


mod database;
mod auth_utils;

fn authenticate(creds: auth_utils::models::Credentials) -> bool {
    if let database::Status::Interrupted = database::connect_to_database() {
        return false;
    }

    auth_utils::login(creds)
}

fn main() {
    let user1 = auth_utils::models::Credentials {
        username: String::from("user1"),
        password: String::from("password"),
    };
    let user2 = auth_utils::models::Credentials {
        username: String::from("user2"),
        password: String::from("password"),
    };
    let invalid_password = auth_utils::models::Credentials {
        username: String::from("user1"),
        password: String::from("123"),
    };

    println!("user1 login attempt: {}", authenticate(user1));
    println!("user2 login attempt: {}", authenticate(user2));
    println!("invalid password attempt: {}", authenticate(invalid_password));
}
