use bcrypt::{hash_with_salt, verify, DEFAULT_COST};
use rand::Rng;

// Traits of an account verifier
trait Verifier {
    fn validate_account(&self, username: String, password: String) -> bool;
    fn validate_username(&self, username: String) -> bool;
    fn validate_password(&self, password: String) -> bool;
}

// Traits of an account password
trait Password {
    fn generate_salt(&self) -> [u8; 16];
    fn hash_password(&self, password: String) -> String;
    fn encrypt_password(&self, password: String) -> String;
    fn compare_password(&self, password: String, hash: String) -> bool;
    fn change_password(&self, password: String, new_password: String) -> bool;
}

// Traits of an account username
trait Username {
    fn change_username(&mut self, username: String, new_username: String) -> bool;
}

// These traits together are what creates our credential
trait Credential: Verifier + Password + Username {}

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
        true
    }

    fn validate_password(&self, password: String) -> bool {
        // TODO: Pull password from database, and compare with hash
        // proper validation should be done here
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

    fn change_password(&self, password: String, new_password: String) -> bool {
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
    fn change_username(&mut self, username: String, new_username: String) -> bool {
        // TODO: connect database to update record
        // - get salt from database 
        self.username = new_username;
        true
    }
}

// A person has an account
enum Person {
    Account(Account),
}

// Implement a method for a person to change their account properties
impl Person {
    
    // A person can create an account
    fn create_account(&mut self, username: String, password: String) -> bool {
        match self {
            Person::Account(account) => {
                account.username = username;
                account.password = account.hash_password(password);
            }
        }
        true
    }

    // A person can change their username and password
    fn change_account(&mut self, username: String, password: String, new_username: String, new_password: String) -> bool {
        match self {
            Person::Account(account) => {
                account.change_username(username, new_username);
                account.change_password(password, new_password);
            }
        }
        true
    }

    // A person can change their username
    fn change_username(&mut self, username: String, new_username: String) -> bool {
        match self {
            Person::Account(account) => {
                account.change_username(username, new_username);
            }
        }
        true
    }

    // A person can change their password
    fn change_password(&self, password: String, new_password: String) -> bool {
        match self {
            Person::Account(account) => {
                account.change_password(password, new_password);
            }
        }
        true
    }
}
