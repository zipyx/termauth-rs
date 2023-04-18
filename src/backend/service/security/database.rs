use rusqlite::{Connection, ToSql, Statement, Rows, Result};

trait SQLConnection {

    fn set_db_schema(&mut self, name: &str) -> Result<()>;
    fn create_db(&mut self, name: &str) -> Result<()>;
    fn create_db_table(&mut self, sql: &str) -> Result<()>;
    fn insert_db_table(&mut self, sql: &str, params: &[&dyn ToSql]) -> Result<()>;
    fn open_in_memory(&mut self) -> Result<()>;
    fn execute(&mut self, sql: &str, params: &[&dyn ToSql]) -> Result<usize>;
    fn prepare(&mut self, sql: &str) -> Result<Statement>;
    fn query(&mut self, sql: &str, params: &[&dyn ToSql]) -> Result<Rows>;

}

pub struct Database {
    connection: dyn SQLConnection,
}

impl SQLConnection for Database {

    fn set_db_schema(&mut self, name: &str) -> Result<()> { self.connection.set_db_schema(name) } fn create_db(&mut self, name: &str) -> Result<()> {
        self.connection.create_db(name)
    }

    fn create_db_table(&mut self, sql: &str) -> Result<()> {
        self.connection.create_db_table(sql)
    }

    fn insert_db_table(&mut self, sql: &str, params: &[&dyn ToSql]) -> Result<()> {
        self.connection.insert_db_table(sql, params)
    }

    fn open_in_memory(&mut self) -> Result<()> {
        self.connection.open_in_memory()
    }

    fn execute(&mut self, sql: &str, params: &[&dyn ToSql]) -> Result<usize> {
        self.connection.execute(sql, params)
    }

    fn prepare(&mut self, sql: &str) -> Result<Statement> {
        self.connection.prepare(sql)
    }

    fn query(&mut self, sql: &str, params: &[&dyn ToSql]) -> Result<Rows> {
        self.connection.query(sql, params)
    }
}
