// pub const MENU_ADMIN: [&str; 5] = ["Welcome", "Sign Up", "Login", "Notepad", "Credential Manager"];
// pub const MENU_GRANTED: [&str; 3] = ["Welcome", "Notepad", "Credential Manager"];
// pub const MENU_RESTRICTED: [&str; 3] = ["Welcome", "Sign Up", "Login"];
// pub const SIGNUP_AUTH: [&str; 2] = ["Username", "Password"];
// pub const LOGIN_AUTH: [&str; 2] = ["Username", "Password"];
// pub const CREDENTIAL: [&str; 3] = ["App", "Username", "Password"];
// pub const LOGS: [&str; 3] = ["Info", "Warning", "Critical"];

pub const INSTRUCTIONS: &str = r#"{
    u:      Yank (copy) the username
    y:      Yank (copy) the password
    e:      Modify the record
    j:      Go to next field
    k:      Go to prev field
  Esc:      Exit insert mode 
}"#;

pub const ENROLMENT_INSTRUCTIONS: &str = r#"

        1) Enrollment:
        ----------------

            A. Creating your account:

                1. Usernames are not case sensitive
                2. Usernames should only use characters from this set [a-zA-Z0-9_]
                3. Usernames should not use swear words
                4. Users shouldn't be able to bypass rule 3 by substituting numbers for letters.

            B. The application follows the NIST password guidelines for rule setting and are to be 
               enforced within this application. You can find more information about these guidelines
               at the following link: 

               - https://pages.nist.gov/800-63-3/sp800-63b.html#sec5
            
            C. Prohibiting common passwords and weak passwords from a specified file can be found
               in the following text document within this repository.

               - weak.txt (~4KB)
               - breachedpasswords.txt (~6KB)

            D. The password, username, and other appropritate information in relation to this 
               application are to be stored using secure methods.

        2) Verification:
        ----------------

            A. In the verification, there is a prompt, depending on the validity of your verification
               you will either receive a success or failure message. This is to ensure that the 
               steps outlined are being followed.

"#;

// pub enum UserMode {
//     Normal,
//     Insert,
//     Username,
//     Password,
// }

// pub enum SignUp {
//     Username,
//     Password,
// }

// pub enum Login {
//     Username,
//     Password,
// }
