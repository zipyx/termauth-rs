use super::state::StateList;

pub const MENU: [&str; 5] = ["Welcome", "Sign Up", "Login", "Notepad", "Credential Manager"];
pub const SIGNUP_AUTH: [&str; 2] = ["Username", "Password"];
pub const LOGIN_AUTH: [&str; 2] = ["Username", "Password"];
pub const CREDENTIAL: [&str; 3] = ["App", "Username", "Password"];
pub const LOGS: [&str; 3] = ["Info", "Warning", "Critical"];

pub const INSTRUCTIONS: &str = r#"{
    u:      Yank (copy) the username
    y:      Yank (copy) the password
    e:      Modify the record
    j:      Go to next field
    k:      Go to prev field
  Esc:      Exit insert mode 
}"#;

pub enum UserMode {
    Normal,
    Insert,
    SignUp,
    LogIn,
    Credential,
}

// pub enum SignUpAuth {
//     Username,
//     Password,
//     Confirm,
// }

// pub enum LogInAuth {
//     Username,
//     Password,
//     Confirm,
// }

pub struct Account<'a> {
    username: String,
    password: String,
    signedin: bool,
    state: StateList<&'a str>,
}

impl <'a>Account<'a> {
    pub fn new() -> Account<'a> {
        Account {
            username: String::new(),
            password: String::new(),
            signedin: true,
            state: StateList::all_items(SIGNUP_AUTH.to_vec()),
        }
    }

    pub fn on_up(&mut self) {
        self.state.previous();
    }

    pub fn on_down(&mut self) {
        self.state.next();
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

}

// #[derive(Clone)]
pub struct Credential<'a> {
    app: String,
    username: String,
    password: String,
    state: StateList<&'a str>,
}

impl <'a>Credential<'a> {
    pub fn new() -> Credential<'a> {
        Credential {
            app: String::new(),
            username: String::new(),
            password: String::new(),
            state: StateList::all_items(CREDENTIAL.to_vec()),
        }
    }

    pub fn on_up(&mut self) {
        self.state.previous();
    }

    pub fn on_down(&mut self) {
        self.state.next();
    }

}
