extern crate rusqlite;
use rusqlite::{Connection, Result};
use std::rc::Rc;


pub enum ConnectionType {
    DatabaseName(String),
    #[allow(dead_code)]
    Connection(Rc<Connection>),
}


pub fn delete(conn_type: ConnectionType, table_name: &str) -> Result<()> {
    let conn = match conn_type {
        ConnectionType::DatabaseName(db_name) => Rc::new(Connection::open(db_name)?),
        ConnectionType::Connection(conn) => conn,
    };

    let query = format!("DROP TABLE {} ", table_name);
    conn.execute(&query, [])?;

    println!("Table {} , Dropped.", table_name);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::params;
    use std::rc::Rc;

    #[test]
    pub fn test_delete_module() -> Result<(), rusqlite::Error> {
        // Create a temporary SQLite database
        let conn = Rc::new(Connection::open_in_memory()?);

        // Create a table
        println!("table created");
        conn.execute("CREATE TABLE tests (name TEXT, value INTEGER)", params![])?;

        // Insert some data
        conn.execute(
            "INSERT INTO tests (name, value) VALUES (?, ?)",
            params!["test", 1],
        )?;
        delete(
            ConnectionType::Connection(Rc::clone(&conn)),
            "tests",
        )?;
        println!("table created");
        fn table_exists(connection: &Connection, table_name: &str) -> Result<bool, rusqlite::Error> {
            let query = "SELECT 1 FROM sqlite_master WHERE type='table' AND name = ?";
            let mut stmt = connection.prepare(query)?;
    
            let exists = stmt.query_row(params![table_name], |_| Ok(true)).ok();
    
            Ok(exists.is_some())
        }
       
        let exists_before_deletion = table_exists(&conn, "tests")?;
        assert!(!exists_before_deletion, "Table 'test' should exist before deletion");


        let exists_after_deletion = table_exists(&conn, "test")?;
        assert!(!exists_after_deletion, "Table 'test' should not exist after deletion");

        Ok(())
    }
}
