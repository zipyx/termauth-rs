pub const MENU: [&str; 3] = ["Sign Up", "Login", "Exit"];
pub const LOGIN: [&str; 2] = ["Username:", "Password:"];
pub const SIGN_UP: [&str; 2] = ["Username:", "Password:"];
pub const LOGS: [&str; 3] = ["Info", "Warning", "Critical"];
pub const EVENTS: [&str; 2] = ["Main", "Submenu"];

pub enum InputMode {
    Normal,
    Insert,
}

pub enum Menu {
    Main,
    Signup,
    Login,
}
