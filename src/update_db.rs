use crate::drop_data::ConnectionType;
use rusqlite::{Connection, Result};
use std::collections::HashMap;
use std::rc::Rc;

pub fn update_db(
    conn_type: ConnectionType,
    table_name: &str,
    condition: &str,
    update_values: HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = match conn_type {
        ConnectionType::DatabaseName(db_name) => Rc::new(Connection::open(db_name)?),
        ConnectionType::Connection(conn) => conn,
    };

    let mut update_str = String::new();
    for (key, value) in update_values.iter() {
        update_str.push_str(&format!("{} = {}, ", key, value));
    }
    update_str.pop();
    update_str.pop();

    let query = format!(
        "UPDATE {} SET {} WHERE {}",
        table_name, update_str, condition
    );

    conn.execute(&query, [])?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::params;
    use std::collections::HashMap;
    use std::rc::Rc;

    #[test]
    fn test_update_db() -> Result<(), Box<dyn std::error::Error>> {
        // Create a temporary SQLite database
        let conn = Rc::new(Connection::open_in_memory()?);

        // Create a table
        Rc::clone(&conn).execute("CREATE TABLE test (name TEXT, value INTEGER)", params![])?;

        // Insert some data
        Rc::clone(&conn).execute(
            "INSERT INTO test (name, value) VALUES (?, ?)",
            params!["test", 1],
        )?;

        // Prepare data for update
        let mut update_values = HashMap::new();
        update_values.insert("value".to_string(), "2".to_string());

        // Test the update_db function
        update_db(
            ConnectionType::Connection(Rc::clone(&conn)),
            "test",
            "name = 'test'",
            update_values,
        )?;

        // Check the results
        let updated_value: i64 = Rc::clone(&conn).query_row(
            "SELECT value FROM test WHERE name = 'test'",
            params![],
            |row| row.get(0),
        )?;
        assert_eq!(updated_value, 2);

        Ok(())
    }
}
