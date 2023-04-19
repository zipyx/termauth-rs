use crate::component::state::TabsState;
use super::{security::authenticator::{Account, Credential}, utility::constants::{VISITOR, MEMBER}};

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

// A person has an account
enum Auth {
    Account(Account),
}

// A user has an account
pub struct User<'a> {
    account: Account,
    auth: Auth,
    pub app: CredentialManager,
    pub app_name: String,
    pub app_password: String,
    app_search_list: Vec<String>,
    app_search_query: String,
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
    pub tab: TabsState<'a>,
    pub user_mode: UserMode,
}

// Implement a method for a person to change their account properties
impl <'a>User<'a> {

    // pub fn new() -> Auth {
    //     Auth::Account(Account::new())
    // }

    /// ... Instantiate a new user with properties
    pub fn new() -> User<'a> {

        User {
            account: Account::new(),
            app: CredentialManager::App,
            app_name: String::new(),
            app_search_list: Vec::new(),
            app_search_query: String::new(),
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
            secure_password: String::new(),
            signed_in: false,
            signup_password: String::new(),
            signup_secure_password: String::new(),
            signup: SignUp::Username,
            signup_username: String::new(),
            tab: TabsState::new(VISITOR.to_vec()),
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
        self.tab = TabsState::new(MEMBER.to_vec());
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
        self.tab.index = 0;
        self.user_mode = UserMode::Normal;
        self.tab = TabsState::new(MEMBER.to_vec());
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

}

