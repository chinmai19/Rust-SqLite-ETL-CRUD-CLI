extern crate rusqlite;
use rusqlite::{Connection, Result};
use std::rc::Rc;

pub enum ConnectionType {
    DatabaseName(String),
    #[allow(dead_code)]
    Connection(Rc<Connection>),
}

pub fn drop_data(conn_type: ConnectionType, table_name: &str, condition: &str) -> Result<()> {
    let conn = match conn_type {
        ConnectionType::DatabaseName(db_name) => Rc::new(Connection::open(db_name)?),
        ConnectionType::Connection(conn) => conn,
    };

    let query = format!("DELETE FROM {} WHERE {}", table_name, condition);
    conn.execute(&query, [])?;

    println!("Table {} , with conditio {}.", table_name, condition);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::params;
    use std::rc::Rc;

    #[test]
    fn test_drop_data() -> Result<(), rusqlite::Error> {
        // Create a temporary SQLite database
        let conn = Rc::new(Connection::open_in_memory()?);

        // Create a table
        conn.execute("CREATE TABLE test (name TEXT, value INTEGER)", params![])?;

        // Insert some data
        conn.execute(
            "INSERT INTO test (name, value) VALUES (?, ?)",
            params!["test", 1],
        )?;

        // Test the drop_data function
        drop_data(
            ConnectionType::Connection(Rc::clone(&conn)),
            "test",
            "value = 1",
        )?;

        // Check the results
        let count: i64 =
            Rc::clone(&conn).query_row("SELECT COUNT(*) FROM test", params![], |row| row.get(0))?;
        assert_eq!(count, 0);

        Ok(())
    }
}
