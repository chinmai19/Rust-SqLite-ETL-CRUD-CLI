extern crate prettytable;
extern crate rusqlite;

use prettytable::{Cell, Row, Table};
use rusqlite::{params, types::Value, Connection, Result};

pub enum ConnectionType {
    DatabaseName(String),
    #[allow(dead_code)]
    Connection(Connection),
}

pub fn query(query_str: String, conn_type: ConnectionType) -> Result<Vec<Vec<Value>>> {
    let conn = match conn_type {
        ConnectionType::DatabaseName(db_name) => Connection::open(db_name)?,
        ConnectionType::Connection(conn) => conn,
    };

    let mut stmt = conn.prepare(&query_str)?;

    let column_count = stmt.column_names().len();

    let rows: Vec<Vec<Value>> = stmt
        .query_map(params![], |row| {
            let mut values = Vec::new();
            for i in 0..column_count {
                values.push(row.get(i)?);
            }
            Ok(values)
        })?
        .filter_map(Result::ok)
        .collect();

    let column_names = stmt.column_names();

    let mut table = Table::new();
    let header: Vec<Cell> = column_names.iter().map(|name| Cell::new(name)).collect();
    table.add_row(Row::new(header));

    for item in &rows {
        let row: Vec<Cell> = item
            .iter()
            .map(|value| match value {
                Value::Integer(i) => Cell::new(&i.to_string()),
                Value::Real(f) => Cell::new(&f.to_string()),
                Value::Text(s) => Cell::new(s),
                Value::Blob(_) => Cell::new("BLOB"),
                Value::Null => Cell::new("NULL"),
            })
            .collect();
        table.add_row(Row::new(row));
    }

    table.printstd();

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::params;

    #[test]
    fn test_query() -> Result<()> {
        // Create a temporary SQLite database
        let conn = Connection::open_in_memory()?;

        // Create a table
        conn.execute("CREATE TABLE test (name TEXT, value INTEGER)", params![])?;

        // Insert some data
        conn.execute(
            "INSERT INTO test (name, value) VALUES (?, ?)",
            params!["test", 1],
        )?;

        // Test the query function
        let rows = query(
            "SELECT * FROM test".to_string(),
            ConnectionType::Connection(conn),
        )?;

        // Check the results
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0][0], Value::Text("test".to_string()));
        assert_eq!(rows[0][1], Value::Integer(1));

        Ok(())
    }
}
