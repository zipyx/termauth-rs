use bcrypt::{hash_with_salt, verify, DEFAULT_COST};
use regex::Regex;
use unicode_normalization::UnicodeNormalization;
use super::database::Database;
use rand::Rng;
use uuid::{Uuid, Builder};

// Traits of an account verifier
trait Verifier {
    fn validate_account(&self, username: String, password: String) -> bool;
    fn validate_username(&self, username: String) -> bool;
    fn validate_password(&self, password: String) -> bool;
}

// Traits of an account id
// trait Uuid {
//     fn generate_uuid(&self) -> dyn Uuid;
// }

// Traits of an account username
trait Username {

    fn set_username(&mut self, username: String, new_username: String) -> bool;
}

// Traits of an account password
trait Password {

    fn generate_salt(&self) -> [u8; 16];
    fn hash_password(&self, password: String) -> String;
    fn encrypt_password(&self, password: String) -> String;
    fn compare_password(&self, password: String, hash: String) -> bool;
    fn set_password(&self, password: String, new_password: String) -> bool;
}


// These traits together are what creates our credential
pub trait Credential: Verifier + Password + Username {

    fn new() -> Self;
    // fn validate_account(&mut self, username: String, password: String) -> bool;
    fn login(&mut self, username: String, password: String) -> bool;
    fn create_account(&mut self, username: String, password: String) -> bool;
    fn change_account(&mut self, username: String, password: String, new_username: String, new_password: String) -> bool;
    fn change_username(&mut self, username: String, new_username: String) -> bool;
    fn change_password(&self, password: String, new_password: String) -> bool;
}

// Build our account struct
pub struct Account {
    username: String,
    password: String,
}

/// A password behviour for an account
impl Verifier for Account {

    fn validate_account(&self, username: String, password: String) -> bool {
        // TODO: Pull record from database, and compare password
        // proper validation should be done here
        true
    }

    fn validate_username(&self, username: String) -> bool {
        // TODO: Pull username from database, and compare
        // proper validation should be done here
        let regex_pattern = r"^[a-zA-Z0-9_]+$";
        let normalization = username.nfkd().collect::<String>();
        let re = Regex::new(regex_pattern).unwrap();

        if re.is_match(&normalization) {
            true
        } else {
            false
        }
    }

    fn validate_password(&self, password: String) -> bool {
        // TODO: Pull password from database, and compare with hash
        // proper validation should be done here
        
        // Password length must be between 8 - 64 characters long
        // Complexity really is about length rather than mix of characters 
        // Password composition allow for printable characters as well as spaces, unicode
        // characters etc. Avoid using "spaces" and tabs in the password.
        // Use weak password text to verify against
        // Do not provide password hints to the user 
        // Implement rate limiting mechanisms to prevent brute force attacks (max 3)
        // Password expiration, store using bcrypt, script, argon2

        let check_password_length = password.len() >= 8 && password.len() <= 64;

        true
    }
}

/// A password behaviour for an account
/// - This is where the hashing and salting happens
/// - This is where the password is encrypted
/// - This is where the password is compared
impl Password for Account {

    fn generate_salt(&self) -> [u8; 16] {

        // Generate random salt, to be stored in database column
        let mut salt: [u8; 16] = [0; 16];
        rand::thread_rng().fill(&mut salt);
        salt
    }

    fn hash_password(&self, password: String) -> String {

        const PEPPER: &str = "PkCt&farjdWL2&WTaoddA2u7S4hfxDkbtNFxxU92";

        // Clone the password into bytes to be concatenated with the pepper
        let mut extended_password: String = password
            .clone()
            .as_bytes()
            .iter()
            .map(|x| x.to_string())
            .collect();

        // Connect the password with the pepper 
        extended_password.extend(PEPPER.as_bytes().iter().map(|x| x.to_string()));

        // Convert extended password to bytes and begin hashing with salt
        let hash = hash_with_salt(extended_password.as_bytes(), DEFAULT_COST, self.generate_salt()).unwrap();
        hash.to_string()
    }

    fn encrypt_password(&self, password: String) -> String {
        String::new()
    }

    fn compare_password(&self, password: String, hash: String) -> bool {
        verify(password, &hash).unwrap()
    }

    fn set_password(&self, password: String, new_password: String) -> bool {
        // check and confirm hash matches with pass
        let check = self.validate_password(password.clone());
        let hash = self.hash_password(password);
        let verify_pass = self.compare_password(new_password, hash);

        // if true, update password
        if verify_pass {
            // TODO: connect database to update record
            true
        } else {
            false
        }
    }
}

// Implement the username behavior of our credential
impl Username for Account {
    fn set_username(&mut self, username: String, new_username: String) -> bool {
        // TODO: connect database to update record
        // - get salt from database 
        self.username = new_username;
        true
    }
}

impl Credential for Account {

    fn new() -> Self {
        Account {
            username: String::new(),
            password: String::new(),
        }
    }

    fn login(&mut self, username: String, password: String) -> bool {
        // self.validate_account(username, password);
        self.username = username;
        self.password = self.hash_password(password);
        true
    }

    fn create_account(&mut self, username: String, password: String) -> bool {
        // self.id.
        self.username = username;
        self.password = self.hash_password(password);
        true
    }

    fn change_account(&mut self, username: String, password: String, new_username: String, new_password: String) -> bool {
        self.set_username(username, new_username);
        self.set_password(password, new_password);
        true
    }

    fn change_username(&mut self, username: String, new_username: String) -> bool {
        self.set_username(username, new_username);
        true
    }

    fn change_password(&self, password: String, new_password: String) -> bool {
        self.set_password(password, new_password);
        true
    }
}
