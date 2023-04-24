use crate::component::state::TabsState;
use super::{security::authenticator::{Account, Credential, Verifier}, utility::constants::{VISITOR, MEMBER}};

/// Enum system user mode containing the following fields for mode behavior
/// - Normal : default mode 
/// - Insert : insert for text behavior mode 
/// - App : app system mode 
/// - Username : username system mode 
/// - Password : password system mode
pub enum UserMode {
    Normal,
    Insert,
    App,
    Username,
    Password,
}

/// Enum signup containing the following fields for mode behaviour
/// - Username : required for username input
/// - Password : required for password input
pub enum SignUp {
    Username,
    Password,
}

/// Enum login containing the following fields for mode behavior
/// - Username : required for username input
/// - Password : required for password input
#[derive(Debug, Clone)]
pub enum Login {
    Username,
    Password,
}

/// Enum credential manager containing the following fields for mode behavior
/// - App : required for app input
/// - Username : required for username input
/// - Password : required for password input
pub enum CredentialManager {
    App,
    Username,
    Password,
}

/// Auth service containing methods for validation and authentication
/// - Account : account object
enum Auth {
    Account(Account),
}

impl Login {

}

/// User Servcie 
/// - A user object with properties and methods
/// - account : account object
/// - app : credential manager object
/// - app_name : app app_name 
/// - app_password : app password 
/// - app_search_list : app search list 
/// - app_search_query : app search query 
/// - app_secure_password : app secure password 
/// - app_username : app username 
/// - login : login object 
/// - login_password : login password 
/// - login_secure_password : login secure password 
/// - login_username : login username 
/// - notepad : notepad 
/// - scratchpad : scratchpad 
/// - secure_password : secure password 
/// - signed_in : signed in 
/// - signup_password : signup password 
/// - signup_secure_password : signup secure password 
/// - signup : signup object 
/// - signup_username : signup username 
/// - tab : tab object 
/// - user_mode : user mode 
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
    login: Login,
    login_username: String,
    login_password: String,
    login_secure_password: String,
    login_error_message: String,
    pub notepad: Vec<String>,
    pub scratchpad: String,
    pub secure_password: String,
    signed_in: bool,
    pub signup: SignUp,
    signup_username: String,
    signup_password: String,
    signup_secure_password: String,
    signup_username_error_message: String,
    signup_password_error_message: String,
    pub tab: TabsState<'a>,
    pub user_mode: UserMode,

}

// Implement a method for a person to change their account properties
impl <'a>User<'a> {

    // ##################################
    // pub fn new() -> Auth {
    //     Auth::Account(Account::new())
    // }
    // ##################################

    /// User Service - Instantiate a new user with properties and methods
    /// - account : account object
    /// - app : credential manager object
    /// - app_name : app name 
    /// - app_password : app password 
    /// - app_search_list : app search list 
    /// - app_search_query : app search query 
    /// - app_secure_password : app secure password 
    /// - app_username : app username 
    /// - auth : auth object 
    /// - login : login object 
    /// - login_password : login password 
    /// - login_secure_password : login secure password 
    /// - login_username : login username 
    /// - notepad : notepad object 
    /// - scratchpad : scratchpad object 
    /// - secure_password : secure password 
    /// - signed_in : signed in status 
    /// - signup_password : signup password 
    /// - signup_secure_password : signup secure password 
    /// - signup : signup object 
    /// - signup_username : signup username 
    /// - tab : tab object 
    /// - user_mode : user mode object
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
            login_username: String::new(),
            login_password: String::new(),
            login_secure_password: String::new(),
            login_error_message: String::new(),
            notepad: Vec::new(),
            scratchpad: String::new(),
            secure_password: String::new(),
            signed_in: false,
            signup: SignUp::Username,
            signup_username: String::new(),
            signup_password: String::new(),
            signup_secure_password: String::new(),
            signup_username_error_message: String::new(),
            signup_password_error_message: String::new(),
            tab: TabsState::new(VISITOR.to_vec()),
            user_mode: UserMode::Normal,
        }
    }

    /// User Service - Getting the user login mode
    pub fn get_login_mode(&mut self) -> Login {
        self.login.clone()
    }

    /// User Service - Setting the login mode for the user
    pub fn set_login_mode(&mut self, login_mode: Login) {
        self.login = login_mode;
    }

    /// User Service - Set the app name for password manager 
    /// - app_name : app name
    pub fn set_app_name(&mut self, app_name: String) {
        self.app_name = app_name;
    }

    /// User Service - Set the app Username for password manager
    /// - app_username : username
    pub fn set_app_username(&mut self, app_username: String) {
        self.app_username = app_username;
    }

    /// User Service - Set the app Password for password manager 
    /// - app_password : password
    pub fn set_app_password(&mut self, app_password: String) {
        self.app_password = app_password;
    }

    /// User Service - Set the app secure password for password manager 
    /// - app_secure_password : secure password
    pub fn set_app_secure_password(&mut self, app_secure_password: String) {
        self.app_secure_password = app_secure_password;
    }

    /// User Service - Get login username at login screen
    pub fn get_login_username(&self) -> String {
        self.login_username.clone()
    }

    /// User Service - Set login username at login screen
    /// - character : char
    pub fn set_login_username(&mut self, character: char) {
        self.login_username.push(character);
    }

    /// User Service - Remove username character 
    pub fn pop_login_username(&mut self) {
        self.login_username.pop();
    }

    /// User Service - Set password at login screen
    /// - password : password
    pub fn set_login_password(&mut self, character: char) {
        self.login_password.push(character);
    }

    /// User Service - Get secure login password at login screen
    pub fn get_login_secure_password(&self) -> String {
        self.login_secure_password.clone()
    }

    /// User Service - Clear login username
    pub fn clear_login_username(&mut self) {
        self.login_username.clear();
    }

    /// User Service - Clear login password 
    pub fn clear_login_password(&mut self) {
        self.login_password.clear();
    }

    /// User Service - Clear login secure password 
    pub fn clear_login_secure_password(&mut self) {
        self.login_secure_password.clear();
    }

    /// User Service - Set secure password to an existing account using provided credentials
    pub fn set_login_secure_password(&mut self, character: char) {
        self.login_secure_password.push(character);
    }

    /// User Service - Get login password at login screen
    pub fn get_login_password(&self) -> String {
        self.login_password.clone()
    }

    /// User Service - Remove password character from an exiting account using provided credentials
    /// - password : password 
    pub fn pop_login_password(&mut self) {
        self.login_password.pop();
    }

    /// User Service - Remove character from secure password
    pub fn pop_login_secure_password(&mut self) {
        self.login_secure_password.pop();
    }

    /// User Service - Set login message
    /// - message : message
    pub fn set_login_error_message(&mut self, message: String) {
        self.login_error_message = message;
    }

    /// User Service - Clear login error message
    pub fn clear_login_error_message(&mut self) {
        self.login_error_message.clear();
    }

    /// User Service - Login to an account using provided credentials
    pub fn login(&mut self, username: String, password: String) -> bool {
        // self.tab = TabsState::new(MEMBER.to_vec());
        // self.account.login(username, password)
        let result = self.account.login(username, password);
        match result.validity {
            true => {
                self.tab = TabsState::new(MEMBER.to_vec());
                self.set_signed_in(true);
                true
            },
            false => {
                self.set_login_error_message(result.message);
                false
            }
        }
    }

    /// User Service - Set username to a temp signup object
    /// - username : username
    pub fn get_signup_username(&mut self) -> String {
        self.signup_username.clone()
    }

    /// User Service - Set username to a temp signup object
    /// - username : username
    pub fn set_signup_username(&mut self, character: char) {
        self.signup_username.push(character);
    }

    /// User Service - Remove username character from a temp signup object
    /// - username : username
    pub fn pop_signup_username(&mut self) {
        self.signup_username.pop();
    }

    /// User Service - Clear signup username
    pub fn clear_signup_username(&mut self) {
        self.signup_username.clear();
    }

    /// User Service - Get signup username error message
    pub fn get_signup_username_error_message(&mut self) -> String {
        self.signup_username_error_message.clone()
    }

    /// User Service - Set signup username error message
    pub fn set_signup_username_error_message(&mut self, message: String) {
        self.signup_username_error_message = message;
    }

    /// User Service - Clear signup username error message
    pub fn clear_signup_username_error_message(&mut self) {
        self.signup_username_error_message.clear();
    }

    /// User Service - Get password to a temp signup object
    /// - password : password
    pub fn get_signup_password(&mut self) -> String {
        self.signup_password.clone()
    }

    /// User Service - Set password to a temp signup object
    /// - password : password
    pub fn set_signup_password(&mut self, characer: char) {
        self.signup_password.push(characer);
    }

    /// User Service - Remove password character from a temp signup object
    pub fn pop_signup_password(&mut self) {
        self.signup_password.pop();
    }

    /// User Service - Clear signup password
    pub fn clear_signup_password(&mut self) {
        self.signup_password.clear();
    }

    /// User Service - Get signup password error message
    pub fn get_signup_password_error_message(&mut self) -> String {
        self.signup_password_error_message.clone()
    }

    /// User Service - Set signup password error message
    pub fn set_signup_password_error_message(&mut self, message: String) {
        self.signup_password_error_message = message;
    }

    /// User Service - Clear signup password error message
    pub fn clear_signup_password_error_message(&mut self) {
        self.signup_password_error_message.clear();
    }

    /// User Service - Get secure password to an account using provided credentials
    /// - secure_password : secure password
    pub fn get_signup_secure_password(&mut self) -> String {
        self.signup_secure_password.clone()
    }

    /// User Service - Set secure password to an account using provided credentials
    /// - secure_password : secure password
    pub fn set_signup_secure_password(&mut self, character: char) {
        self.signup_secure_password.push(character);
    }

    /// User Service - Remove secure password character from an account using provided credentials
    /// - secure_password : secure password
    pub fn pop_signup_secure_password(&mut self) {
        self.signup_secure_password.pop();
    }

    /// User Service - Clear secure password from an account using provided credentials
    pub fn clear_signup_secure_password(&mut self) {
        self.signup_secure_password.clear();
    }

    /// User Service = Get the current signed in state of the user
    pub fn get_signed_in(&self) -> bool {
        self.signed_in
    }

    /// User Service - Set the signed in state of the user
    pub fn set_signed_in(&mut self, signed_in: bool) {
        self.signed_in = signed_in;
    }

    /// User Service - Create an account using provided credentials, the account is then
    /// verified through the authenticator service. once verified - an account is created 
    /// and stored in an in-memory database or secure file.
    /// - username : username 
    /// - password : password
     pub fn create_account(&mut self, username: String, password: String) -> bool {

        let password_response = self.account.validate_password(password.clone());
        let username_response = self.account.validate_username(username.clone());

        match username_response.validity {
            true => {}
            _ => {
                self.clear_signup_username();
                self.set_signup_username_error_message(
                    username_response.message
                );
                return false;
            }
        }

        match password_response.validity {
            true => {}
            _ => {
                self.clear_signup_password();
                self.clear_signup_secure_password();
                self.set_signup_password_error_message(
                    password_response.message
                );
                return false;
            }
        }

        let mut account = Account::new();
        account.create_account(username, password)
    }

    /// User Service - Modify a user's password
    /// - password : current password
    /// - new_password : new password
    fn change_account_password(&self, password: String, new_password: String) -> bool {
        self.account.change_password(password, new_password)
    }

}

