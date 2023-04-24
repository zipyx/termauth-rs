/// Response after validation of password
pub struct Response {
    pub validity: bool,
    pub message: String,
}

pub struct Request {
    pub username: String,
    pub password: String,
    pub salt: String,
}
