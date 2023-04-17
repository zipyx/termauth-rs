// use std::fs::File;
// use std::io::Write;
// use regex::Regex;
// use crate::component::state::StateList;

pub enum UserMode {
    Normal,
    Insert,
    App,
    Username,
    Password,
}

pub enum SignUp {
    Username,
    Password,
}

pub enum Login {
    Username,
    Password,
}

const SIGNUP_AUTH: [&str; 2] = ["Username", "Password"];

// Implement User Service
pub trait UserService {
    fn new() -> User;
    fn new_account(&mut self, username: String, password: String);
    fn set_username(&mut self, username: String);
    fn set_password(&mut self, password: String);
    fn validate_signup(&self, username: String, password: String) -> bool;
    fn validate_login(&self, username: String, password: String) -> bool;
}

// Implement User Struct
pub struct User {
    username: String,
    password: String,
    signedin: bool,
    loggedin: bool,
}

// Implement User Service For User
impl UserService for User {
    fn new() -> User {
        User {
            username: String::new(),
            password: String::new(),
            signedin: false,
            loggedin: false,
        }
    }

    fn new_account(&mut self, username: String, password: String) {
        self.username = username;
        self.password = password;
    }

    fn set_username(&mut self, username: String) {
        self.username = username;
    }

    fn set_password(&mut self, password: String) {
        self.password = password;
    }

    fn validate_signup(&self, username: String, password: String) -> bool {
        true
    }

    fn validate_login(&self, username: String, password: String) -> bool {
        true
    }

}
