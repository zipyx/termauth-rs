use rusqlite::{Connection, ToSql, Statement, Rows, Result};

trait DatabaseManager {

    fn new() -> Self;
    fn build_schema(&mut self, name: &str) -> Result<()>;

}

trait AccountManager {
    fn create_account(&mut self, username: &str, password: &str, role: &str) -> Result<()>;
    fn update_account(&mut self, username: &str, password: &str, role: &str) -> Result<()>;
    fn update_account_password(&mut self, username: &str, password: &str) -> Result<()>;
    fn delete_account(&mut self, username: &str) -> Result<()>;
    fn get_account(&mut self, username: &str) -> Result<()>;
}

trait CredentialManager {
    fn create_credential(&mut self, username: &str, password: &str, role: &str) -> Result<()>;
    fn update_credential(&mut self, username: &str, password: &str, role: &str) -> Result<()>;
    fn delete_credential(&mut self, username: &str) -> Result<()>;
    fn get_credential(&mut self, username: &str) -> Result<()>;
    fn get_all_credentials(&mut self, account: &str) -> Result<()>;
}

pub struct Database {
    connection: Connection,
}

impl DatabaseManager for Database {
    fn new() -> Self {
        let connection = Connection::open("database.db").unwrap();
        Database { connection }
    }

    fn build_schema(&mut self, name: &str) -> Result<()> {

        const TABLE_ACCOUNT: &str = 
            "CREATE TABLE IF NOT EXISTS accounts (
                id              CHAR(36)                PRIMARY KEY,
                username        VARCHAR(36)             NOT NULL UNIQUE,
                password        VARCHAR(150)            NOT NULL,
                salt            VARCHAR(150)            NOT NULL,
                role            ENUM('devuser', 'you')  NOT NULL DEFAULT 'you',
                secure_password VARCHAR(150)            NOT NULL,
                claim           VARCHAR(250)            NOT NULL,
                logged_in       BOOLEAN                 NOT NULL DEFAULT 0,
                signed_in       BOOLEAN                 NOT NULL DEFAULT 0,
                created_at      DATETIME                NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at      DATETIME                NOT NULL DEFAULT CURRENT_TIMESTAMP
            )";

        const TABLE_CREDENTIALS: &str = 
            "CREATE TABLE IF NOT EXISTS credentials (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                account         CHAR(36) NOT NULL,
                username        VARCHAR(36) NOT NULL,
                password        VARHCAR(150) NOT NULL,
                salt            VARCHAR(150) NOT NULL,
                FOREIGN KEY (account) REFERENCES accounts(id)
                INDEX (account)
            )";

        const VIEW_ACCOUNT: &str = 
            "CREATE OR REPLACE
                ALGORITHM = UNDEFINED
                SQL SECURITY DEFINER
            VIEW `accounts_view` AS 
                SELECT 
                    `accounts`.`id` AS `id`,
                    `accounts`.`username` AS `username`,
                    `accounts`.`password` AS `password`,
                    `accounts`.`salt` AS `salt`,
                    `accounts`.`role` AS `role`,
                    `accounts`.`secure_password` AS `secure_password`,
                    `accounts`.`claim` AS `claim`,
                    `accounts`.`logged_in` AS `logged_in`,
                    `accounts`.`signed_in` AS `signed_in`,
                    `accounts`.`created_at` AS `created_at`,
                    `accounts`.`updated_at` AS `updated_at`
            ";

        const VIEW_CREDENTIALS: &str = 
            "CREATE OR REPLACE
                ALGORITHM = UNDEFINED
                SQL SECURITY DEFINER
            VIEW `credentials_view` AS 
                SELECT 
                    `credentials`.`id` AS `id`,
                    `credentials`.`account` AS `account`,
                    `credentials`.`username` AS `username`,
                    `credentials`.`password` AS `password`,
                    `credentials`.`salt` AS `salt`
            ";

        self.connection.execute(TABLE_ACCOUNT, [])?;
        self.connection.execute(TABLE_CREDENTIALS, [])?;
        self.connection.execute(VIEW_ACCOUNT, [])?;
        self.connection.execute(VIEW_CREDENTIALS, [])?;

        Ok(())
    }
}

impl AccountManager for Database {
    fn create_account(&mut self, username: &str, password: &str, role: &str) -> Result<()> {
        self.connection.execute(
            "INSERT INTO accounts (username, password, role) VALUES (?1, ?2, ?3)",
            &[&username, &password, &role],
        )?;

        Ok(())
    }

    fn update_account(&mut self, username: &str, password: &str, role: &str) -> Result<()> {
        self.connection.execute(
            "UPDATE accounts SET password = ?1, role = ?2 WHERE username = ?3",
            &[&password, &role, &username],
        )?;

        Ok(())
    }

    fn update_account_password(&mut self, username: &str, password: &str) -> Result<()> {
        self.connection.execute(
            "UPDATE accounts SET password = ?1 WHERE username = ?2",
            &[&password, &username],
        )?;

        Ok(())
    }

    fn delete_account(&mut self, username: &str) -> Result<()> {
        self.connection.execute(
            "DELETE FROM accounts WHERE username = ?1",
            &[&username],
        )?;

        Ok(())
    }

    fn get_account(&mut self, username: &str) -> Result<()> {
        // get just the account from database based on username 
        let mut stmt = self.connection.prepare("SELECT * FROM accounts WHERE username = ?1")?;

        Ok(())
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
