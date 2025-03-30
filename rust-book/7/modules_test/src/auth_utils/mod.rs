pub mod models;

pub fn login(creds: models::Credentials) -> bool {
    let user_creds = crate::database::get_user();
    creds.username == user_creds.username && creds.password == user_creds.password
}

fn logout() {
    // log user out
}
