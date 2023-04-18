// use std::fs::File;
// use std::io::Write;
// use regex::Regex;
// use crate::component::state::StateList;

use super::security::authenticator::{Account, Credential};

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

pub enum CredentialManager {
    App,
    Username,
    Password,
}

const SIGNUP_AUTH: [&str; 2] = ["Username", "Password"];

// A person has an account
enum Auth {
    Account(Account),
}


pub struct User {
    account: Account,
    auth: Auth,
    pub app: CredentialManager,
    pub app_name: String,
    pub app_password: String,
    pub app_secure_password: String,
    pub app_username: String,
    pub login: Login,
    pub login_password: String,
    pub login_secure_password: String,
    pub login_username: String,
    pub notepad: Vec<String>,
    pub scratchpad: String,
    pub secure_password: String,
    pub signed_in: bool,
    pub signup_password: String,
    pub signup_secure_password: String,
    pub signup: SignUp,
    pub signup_username: String,
    pub user_mode: UserMode,
    search_list: Vec<String>,
    search_query: String,
}

// Implement a method for a person to change their account properties
impl User {

    // pub fn new() -> Auth {
    //     Auth::Account(Account::new())
    // }

    /// ... Instantiate a new user with properties
    pub fn new() -> User {
        User {
            account: Account::new(),
            app: CredentialManager::App,
            app_name: String::new(),
            app_secure_password: String::new(),
            app_password: String::new(),
            app_username: String::new(),
            auth: Auth::Account(Account::new()),
            login: Login::Username,
            login_password: String::new(),
            login_secure_password: String::new(),
            login_username: String::new(),
            notepad: Vec::new(),
            scratchpad: String::new(),
            search_list: Vec::new(),
            search_query: String::new(),
            secure_password: String::new(),
            signed_in: false,
            signup_password: String::new(),
            signup_secure_password: String::new(),
            signup: SignUp::Username,
            signup_username: String::new(),
            user_mode: UserMode::Normal,
        }
    }

    /// ... Set the app name
    pub fn set_app_name(&mut self, app_name: String) {
        self.app_name = app_name;
    }

    /// ... Set the app Username
    pub fn set_app_username(&mut self, app_username: String) {
        self.app_username = app_username;
    }

    /// ... Set the app Password
    pub fn set_app_password(&mut self, app_password: String) {
        self.app_password = app_password;
    }

    /// ... Set the app secure password
    pub fn set_app_secure_password(&mut self, app_secure_password: String) {
        self.app_secure_password = app_secure_password;
    }

    /// ... Login to an account using provided credentials
    pub fn login(&mut self, username: String, password: String) -> bool {
        self.account.login(username, password)
    }

    /// ... Set username to an account using provided credentials
    pub fn set_login_username(&mut self, username: String) {
        self.login_username = username;
    }

    /// ... Set password to an account using provided credentials
    pub fn set_login_password(&mut self, password: String) {
        self.login_password = password;
    }

    /// ... Set secure password to an account using provided credentials
    pub fn set_login_secure_password(&mut self, secure_password: String) {
        self.login_secure_password = secure_password;
    }

    /// ... Set username to an account using provided credentials
    pub fn set_signup_username(&mut self, username: String) {
        self.signup_username = username;
    }

    /// ... Set password to an account using provided credentials
    pub fn set_signup_password(&mut self, password: String) {
        self.signup_password = password;
    }

    /// ... Set secure password to an account using provided credentials
    pub fn set_signup_secure_password(&mut self, secure_password: String) {
        self.signup_secure_password = secure_password;
    }

    /// ... Create an account
    pub fn create_account(&mut self, username: String, password: String) -> bool {
        self.account.create_account(username, password)
    }

    /// ... Modify a user's account
    fn change_account(&mut self, username: String, password: String, new_username: String, new_password: String) -> bool {
        self.account.change_account(username, password, new_username, new_password)
    }

    /// ... Modify a user's username
    fn change_account_username(&mut self, username: String, new_username: String) -> bool {
        self.account.change_username(username, new_username)
    }

    /// ... Modify a user's password
    fn change_account_password(&self, password: String, new_password: String) -> bool {
        self.account.change_password(password, new_password)
    }

    // fn login(&mut self, username: String, password: String) -> bool {
    //     match self {
    //         Auth::Account(account) => {
    //             account.validate_account(username, password)
    //         }
    //     }
    // }
    
    // // A person can create an account
    // fn create_account(&mut self, username: String, password: String) -> bool {
    //     match self {
    //         Auth::Account(account) => {
    //             account.create_account(username, password)
    //         }
    //     }
    // }

    // // A person can change their username and password
    // fn change_account(&mut self, username: String, password: String, new_username: String, new_password: String) -> bool {
    //     match self {
    //         User::Account(account) => {
    //             account.change_account(username, password, new_username, new_password)
    //         }
    //     }
    // }

    // // A person can change their username
    // fn change_username(&mut self, username: String, new_username: String) -> bool {
    //     match self {
    //         User::Account(account) => {
    //             account.change_username(username, new_username)
    //         }
    //     }
    // }

    // // A person can change their password
    // fn change_password(&self, password: String, new_password: String) -> bool {
    //     match self {
    //         User::Account(account) => {
    //             account.change_password(password, new_password)
    //         }
    //     }
    // }
}

