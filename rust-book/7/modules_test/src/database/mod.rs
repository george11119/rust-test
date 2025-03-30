use crate::auth_utils;

pub fn connect_to_database() -> Status {
    Status::Connected
}

pub enum Status {
    Connected,
    Interrupted,
}

pub fn get_user() -> auth_utils::models::Credentials {
    auth_utils::models::Credentials {
        username: String::from("user1"),
        password: String::from("password"),
    }
}
