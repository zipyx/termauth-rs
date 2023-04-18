use std::fs::File;
use std::io::Write;

pub trait Manager {
    fn new() -> Credential;
    fn new_credential(&mut self, app: String, username: String, password: String);
    fn set_app(&mut self, app: String);
    fn set_username(&mut self, username: String);
    fn set_password(&mut self, password: String);
    fn get_app(&self) -> String;
    fn get_username(&self) -> String;
    fn get_password(&self) -> String;
}

pub struct Credential {
    app: String,
    username: String,
    password: String,
}

impl Manager for Credential {
    fn new() -> Credential {
        Credential {
            app: String::new(),
            username: String::new(),
            password: String::new(),
        }
    }

    fn new_credential(&mut self, app: String, username: String, password: String) {
        self.app = app;
        self.username = username;
        self.password = password;
        let path = "users.txt";
        let mut output = File::create(path).unwrap();
        let creds = format!("[username]: {} - [password]: {}", self.username, self.password);
        write!(output, "{}", creds);
    }

    fn set_app(&mut self, app: String) {
        self.app = app;
    }

    fn set_username(&mut self, username: String) {
        self.username = username;
    }

    fn set_password(&mut self, password: String) {
        self.password = password;
    }

    fn get_app(&self) -> String {
        self.app.clone()
    }

    fn get_username(&self) -> String {
        self.username.clone()
    }

    fn get_password(&self) -> String {
        self.password.clone()
    }
}
