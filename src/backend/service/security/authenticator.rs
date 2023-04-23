use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

use bcrypt::{hash_with_salt, verify, DEFAULT_COST};
use rustrict::CensorStr;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;
use rand::Rng;
use super::database::{Database, DatabaseManager, AccountManager};
use super::super::utility::response::Response;

/// Traits of an account verifier
pub trait Verifier {
    fn read_file(&self, file: &str) -> Vec<String>;
    fn read_file_hashset(&self, file: &str) -> HashSet<String>;
    fn validate_account(&self, username: String, password: String) -> bool;
    fn validate_username(&self, username: String) -> bool;
    fn validate_password(&self, password: String) -> Response;
    fn validate_swear_words_regex_pattern_match(&self, text: String) -> bool;
}

/// Traits of an account username
pub trait Username {

    fn set_username(&mut self, username: String, new_username: String) -> bool;
}

/// Traits of an account password
pub trait Password {

    fn generate_salt(&self) -> [u8; 16];
    fn hash_password(&self, password: String) -> String;
    fn compare_password(&self, password: String, hash: String) -> bool;
    fn set_password(&self, password: String, new_password: String) -> bool;
}


/// These traits together are what creates our credential
pub trait Credential: Verifier + Password + Username {

    fn new() -> Self;
    fn login(&mut self, username: String, password: String) -> bool;
    fn create_account(&mut self, username: String, password: String) -> bool;
    fn change_password(&self, password: String, new_password: String) -> bool;
}

/// Account struct 
pub struct Account {
    username: String,
    password: String,
}

/// A password behviour for an account
impl Verifier for Account {

    /// Validate the account against the username and password 
    fn validate_account(&self, username: String, password: String) -> bool {
        // TODO: Pull record from database, and compare password
        // proper validation should be done here
        
        self.validate_username(username) && self.validate_password(password).validity
    }

    /// Validate the username being passed through
    fn validate_username(&self, username: String) -> bool {

        // Implement regex filter from requirements
        let regex_pattern = r"^[a-zA-Z0-9_]+$";
        // Normalize the username being passed
        let normalization = username.nfkd().nfkc().nfd().filter(|c| c.is_alphanumeric()).collect::<String>();
        let re = Regex::new(regex_pattern).unwrap();

        // if first rule of regex filter doesn't work return false immediately as 
        // requirements are not met
        if re.is_match(&normalization) {

            // Speed of (o(n^2)) - not ideal using regex expressions
            // return self.validate_swear_words_regex_pattern_match(normalization)

            // Speed of o(n) - better choice integration with std library
            return normalization.is_inappropriate()
        }
        true
    }

    /// Validate the password being passed through before being stored
    fn validate_password(&self, password: String) -> Response {
        // TODO: Pull password from database, and compare with hash
        // proper validation should be done here
        
        // 9 rules to follow
        // 1 - Password length must be between 8 - 64 characters long
        // 2 - Check passwords against a list of known weak passwords / blacklist
        // 3 - Make special characters optional
        // 4 - Provide the user feedback on password attempts if they fail and why
        // 5 - Do not provide password hints
        // 6 - Implement rate limiting mechanisms to prevent brute force attacks
        // 7 - Use password managers safely, or create one yourself :)
        // 8 - Change password only when neccessary
        // 9 - Store passwords in offline - attack - resistant forms.
        // ===============================================================================
        // Complexity really is about length rather than mix of characters 
        // Password composition allow for printable characters as well as spaces, unicode
        // characters etc. Avoid using "spaces" and tabs in the password.
        // Use weak password text to verify against
        // Do not provide password hints to the user 
        // Implement rate limiting mechanisms to prevent brute force attacks (max 3)
        // Password expiration, store using bcrypt, script, argon2

        // 1 - Check password length between 8 - 64 characters long
        let check_password_length = password.len() >= 8 && password.len() <= 64;

        // 2 - Check passwords against a list of known weak passwords / blacklist
        let weak_passwords = self.read_file_hashset("weakpasswords.txt");
        let breached_passwords = self.read_file_hashset("breachedpasswords.txt");

        // if 1st rule is met, check for second rule and return true if both are met
        if check_password_length {
            if breached_passwords.contains(&password) || weak_passwords.contains(&password) {

                // Password found in blacklist
                return Response {
                    validity: true,
                    message: "Passord is compromised and is available online, use another.".to_string(),
                }
            } else {

                // Password not found in blacklist and meets length requirements
                return Response {
                    validity: false,
                    message: "Password is secure".to_string(),
                }
            }
        } else {

            // Password does not meet length requirements
            return Response {
                validity: false,
                message: "Pasword must be at least 8 characters long".to_string(),
            }
        }

        // 3 - Make special characters optional - Do nothing here
        // 4 - Provide the user feedback on password attempts if they fail and why - Do this in the
        //   UI layer
        // 5 - Do not provide password hints - Do nothing here
        // 6 - Implement rate limiting checks
    }

    /// Read a file and return a vector of strings
    fn read_file(&self, file_name: &str) -> Vec<String> {

        let mut list = Vec::new();
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            list.push(line.unwrap());
        }
        list
    }

    /// Read a file and return a hashset of strings
    fn read_file_hashset(&self, file_name: &str) -> HashSet<String> {

        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);
        let list: HashSet<String> = reader.lines().map(|line| line.unwrap()).collect();

        return list
    }

    /// Validate the usernames against a list of swear words using regex
    fn validate_swear_words_regex_pattern_match(&self, text: String) -> bool {
        let filter_patterns = self.read_file("regex.txt");
        for pattern in filter_patterns {
            let re = Regex::new(pattern.as_str()).unwrap();
            if re.is_match(text.as_str()) {
                return false;
            }
        }
        true
    }
}

/// A password behaviour for an account
/// - This is where the hashing and salting happens
/// - This is where the password is encrypted
/// - This is where the password is compared
impl Password for Account {

    /// Generate the salt for our password hashing function
    fn generate_salt(&self) -> [u8; 16] {

        // Generate random salt, to be stored in database column
        let mut salt: [u8; 16] = [0; 16];
        rand::thread_rng().fill(&mut salt);
        salt
    }

    /// Hash function for our password
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

    /// compare the password with the hash to verify if they match
    fn compare_password(&self, password: String, hash: String) -> bool {
        verify(password, &hash).unwrap()
    }

    /// Set | Change an existing password 
    fn set_password(&self, password: String, new_password: String) -> bool {

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

/// Implement the username behavior of our credential
impl Username for Account {

    /// Set the username of an existing account
    fn set_username(&mut self, username: String, new_username: String) -> bool {
        // TODO: connect database to update record
        // - get salt from database 
        self.username = new_username;
        true
    }
}

/// Connect Database 
struct Db {
    database: Database,
}

/// Implement the credential behavior of our account
impl Credential for Account {

    /// Create a new account
    fn new() -> Self {
        Account {
            username: String::new(),
            password: String::new(),
        }
    }

    /// Login to an existing account
    fn login(&mut self, username: String, password: String) -> bool {
        // self.validate_account(username, password);
        self.username = username;
        self.password = self.hash_password(password);
        true
    }

    /// This is where it all starts, you sign up by creating an account
    /// through this method. Before we actually create the account, what we'll
    /// have to do is validate the username and the password.
    /// - Validate the Username
    /// - Validate the Password
    /// - Create the account
    fn create_account(&mut self, username: String, password: String) -> bool {

        // if self.validate_account(username.to_owned(), password.to_owned()) {

        //     // create account using database query here
        //     // or create account using the offline-status-store such 
        //     // as keyring / TODO: Figure which one
        //     let mut db = Db {
        //         database: Database::new()
        //     };

        //     // - Generate salt,
        //     let salt: [u8; 16] = self.generate_salt();
        //     // - Generate password hash 
        //     let password_hash: String = self.hash_password(password);
        //     // - Store salt and password hash in Database
        //     let result = db.database.create_account(username.as_str(), password_hash.as_str(), salt).unwrap();

        //     return result
        // } else {
        //     false
        // }

        // let account_validity = self.validate_account(username.to_owned(), password.to_owned());
        match self.validate_account(username.to_owned(), password.to_owned()) {
            true => {
                // create account using database query here
                // or create account using the offline-status-store such 
                // as keyring / TODO: Figure which one
                let mut db = Db {
                    database: Database::new()
                };

                // - Generate salt,
                let salt: [u8; 16] = self.generate_salt();
                // - Generate password hash 
                let password_hash: String = self.hash_password(password);
                // - Store salt and password hash in Database
                let result = db.database.create_account(username.as_str(), password_hash.as_str(), salt).unwrap();

                return result
            },
            false => false
        }

    }

    /// Change the password of an existing account
    fn change_password(&self, password: String, new_password: String) -> bool {
        self.set_password(password, new_password);
        true
    }
}
