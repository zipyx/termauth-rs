use std::{io::{Read, Write}, path::Path, fs::File};

use rand::Rng;
use rusqlite::{Connection, ToSql, Statement, Rows, Result, named_params};

/// Database Manager with the following methods and behavior
pub trait DatabaseManager {

    fn new() -> Self;
    fn build_schema(&mut self, name: &str) -> Result<(), rusqlite::Error>;

}

/// Account Manager with the following methods and behavior
pub trait AccountManager {
    fn create_account(&mut self, username: &str, password: &str, salt: [u8; 16]) -> Result<bool, rusqlite::Error>;
    fn update_account_password(&mut self, username: &str, password: &str) -> Result<(), rusqlite::Error>;
    fn get_account(&mut self, username: &str) -> Result<Record, rusqlite::Error>;
}

/// Credential Manager with the following methods and behavior
trait CredentialManager {
    fn create_credential(&mut self, username: &str, password: &str, role: &str) -> Result<(), rusqlite::Error>;
    fn update_credential(&mut self, username: &str, password: &str, role: &str) -> Result<(), rusqlite::Error>;
    fn delete_credential(&mut self, username: &str) -> Result<(), rusqlite::Error>;
    fn get_credential(&mut self, username: &str) -> Result<(), rusqlite::Error>;
    fn get_all_credentials(&mut self, account: &str) -> Result<(), rusqlite::Error>;
}

/// Database struct implementing the Connection trait from crate
pub struct Database {
    connection: Connection,
}

/// DatabaseManager trait implementation for Database struct
impl DatabaseManager for Database {

    /// Create a new Database struct
    fn new() -> Database {
        let connection = Connection::open("database.db").unwrap();
        // let connection = Connection::open_in_memory().unwrap();
        Database { connection }
    }

    /// Build the database schema
    fn build_schema(&mut self, name: &str) -> Result<(), rusqlite::Error> {

        const _DATABASE: &str = 
            "CREATE DATABASE IF NOT EXISTS ?1";

        const TABLE_ACCOUNT: &str = 
            "CREATE TABLE IF NOT EXISTS account (
                id              CHAR(36)                PRIMARY KEY,
                username        VARCHAR(40)             NOT NULL UNIQUE,
                password        VARCHAR(150)            NOT NULL,
                salt            BINARY(150)             NOT NULL,
                logged_in       BOOLEAN                 NOT NULL DEFAULT 0,
                signed_in       BOOLEAN                 NOT NULL DEFAULT 0,
                created_at      DATETIME                NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at      DATETIME                NOT NULL DEFAULT CURRENT_TIMESTAMP
            )";

        const TABLE_CREDENTIALS: &str = 
            "CREATE TABLE IF NOT EXISTS password_manager (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                account         CHAR(36) NOT NULL,
                username        VARCHAR(36) NOT NULL,
                password        BINARY(150) NOT NULL,
                salt            VARCHAR(150) NOT NULL,
                FOREIGN KEY (account) REFERENCES account(id)
            )";

        const _VIEW_ACCOUNT: &str = 
            "CREATE OR REPLACE
                ALGORITHM = UNDEFINED
                SQL SECURITY DEFINER
            VIEW `account_view` AS 
                SELECT 
                    `account`.`id` AS `id`,
                    `account`.`username` AS `username`,
                    `account`.`password` AS `password`,
                    `account`.`salt` AS `salt`,
                    `account`.`logged_in` AS `logged_in`,
                    `account`.`signed_in` AS `signed_in`,
                    `account`.`created_at` AS `created_at`,
                    `account`.`updated_at` AS `updated_at`
            ";

        const _VIEW_CREDENTIALS: &str = 
            "CREATE OR REPLACE
                ALGORITHM = UNDEFINED
                SQL SECURITY DEFINER
            VIEW `password_manager_view` AS 
                SELECT 
                    `password_manager`.`id` AS `id`,
                    `password_manager`.`account` AS `account`,
                    `password_manager`.`username` AS `username`,
                    `password_manager`.`password` AS `password`,
                    `password_manager`.`salt` AS `salt`
            ";

        // self.connection.execute(DATABASE, [name])?;
        self.connection.execute(TABLE_ACCOUNT, [])?;
        self.connection.execute(TABLE_CREDENTIALS, [])?;
        // self.connection.execute(VIEW_ACCOUNT, [])?;
        // self.connection.execute(VIEW_CREDENTIALS, [])?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Record {
    id: String,
    pub username: String,
    pub password: String,
    salt: String,
    pub logged_in: bool,
}

/// AccountManager trait implementation for Database struct
impl AccountManager for Database {

    /// Create a new account in the database
    /// We use a UUID as the primary key for the account 
    /// - username provided by sys 
    /// - password is hashed and salted 
    /// - salt is also stored in the database 
    /// - secure_password (to be removed)
    /// - claim as a JWT token (to be removed)
    /// - logged_in and signed_in are booleans (to be removed - probably)
    /// - created_at and updated_at are timestamps
    fn create_account(&mut self, username: &str, password: &str, salt: [u8; 16]) -> Result<bool, rusqlite::Error> {

        // Convert salt into readable string type
        // let salt_string = match std::str::from_utf8(&salt) {
        //     Ok(v) => v,
        //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        // };

        // let test = format!("{:?}", salt);

        self.build_schema("database.db")?;

        // let salt_string = format!("{:?}", salt);
        let salt_string = String::from_utf8_lossy(&salt).to_string();

        // let salt_string = salt

        // Generate random UUID for user account
        let random_bytes = rand::thread_rng().gen::<[u8; 16]>();
        let id = uuid::Builder::from_bytes(random_bytes).into_uuid().to_string();

        // Write contents to a file (test)
        let path = Path::new("users.txt");
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        let account_credential = format!("Database Service :: \nid: {} \nusername:{}\npassword:{}\nsalt:{}:", id, username, password, salt_string);
        match file.write_all(account_credential.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }

        // Insert query with parameters
        match self.connection.execute(
            "INSERT INTO `account` (`id`, `username`, `password`, `salt`) VALUES (?1, ?2, ?3, ?4)",
            &[&id, username, &password, salt_string.as_str()],
            ) 
            {
                Ok(_) => {
                    Ok(true)
                },
                Err(_) => Ok(false),
            }
    }

    fn update_account_password(&mut self, username: &str, password: &str) -> Result<(), rusqlite::Error> {
        self.connection.execute(
            "UPDATE account SET password = ?1 WHERE username = ?2",
            &[&password, &username],
        )?;

        Ok(())
    }

    fn get_account(&mut self, username: &str) -> Result<Record, rusqlite::Error> {

        // Return multiple records
        // let mut stmt = self.connection.prepare("SELECT * FROM account").unwrap();
        // let result = stmt.query_map([], |row| {
        //     Ok(Record {
        //         id: row.get(0)?,
        //         username: row.get(1)?,
        //         password: row.get(2)?,
        //     })
        // })?;
        // for person in result {
        //     println!("Found person {:?}", person.unwrap());
        //     // return person.unwrap();
        // }
        // let result = stmt.execute(&[&username]);
        // println!("{:?}", result.);

        let mut stmt = self.connection.prepare("SELECT * FROM `account` WHERE `username` = ?1").unwrap();
        let record = stmt.query_row([username], |row| {
            Ok(Record {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                salt: row.get(3)?,
                logged_in: row.get(4)?,
            })
        });
        record

    }

}


impl CredentialManager for Database {
    fn create_credential(&mut self, username: &str, password: &str, role: &str) -> Result<()> {
        self.connection.execute(
            "INSERT INTO credentials (username, password, role) VALUES (?1, ?2, ?3)",
            &[&username, &password, &role],
        )?;

        Ok(())
    }

    fn update_credential(&mut self, username: &str, password: &str, role: &str) -> Result<()> {
        self.connection.execute(
            "UPDATE credentials SET password = ?1, role = ?2 WHERE username = ?3",
            &[&password, &role, &username],
        )?;

        Ok(())
    }

    fn delete_credential(&mut self, username: &str) -> Result<()> {
        self.connection.execute(
            "DELETE FROM credentials WHERE username = ?1",
            &[&username],
        )?;

        Ok(())
    }

    fn get_credential(&mut self, username: &str) -> Result<()> {
        // get just the credential from database based on username 
        let mut stmt = self.connection.prepare("SELECT * FROM `credentials` WHERE `username` = ?1")?;
        stmt.execute(&[&username])?;

        Ok(())
    }

    fn get_all_credentials(&mut self, account: &str) -> Result<()> {
        // get all credentials from database
        let mut stmt = self.connection.prepare("SELECT * FROM `credentials` WHERE `account` = ?1")?;
        stmt.execute(&[&account])?;

        Ok(())
    }
}
