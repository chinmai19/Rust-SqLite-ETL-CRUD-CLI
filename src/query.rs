extern crate rusqlite;
extern crate prettytable;

use rusqlite::{params, Connection, Result, types::Value};
use prettytable::{Table, Row, Cell};


pub fn query(query_str: String, _db_name: String) -> Result<Vec<Vec<Value>>> {
    let conn = Connection::open(_db_name)?;
    let mut stmt = conn.prepare(&query_str)?;

    let column_count = stmt.column_names().len();

    let rows: Vec<Vec<Value>> = stmt.query_map(params![], |row| {
        let mut values = Vec::new();
        for i in 0..column_count {
            values.push(row.get(i)?);
        }
        Ok(values)
    })?.filter_map(Result::ok).collect();

    let column_names = stmt.column_names();

    let mut table = Table::new();
    let header: Vec<Cell> = column_names.iter().map(|name| Cell::new(name)).collect();
    table.add_row(Row::new(header));

    for item in &rows {
        let row: Vec<Cell> = item.iter().map(|value| {
            match value {
                Value::Integer(i) => Cell::new(&i.to_string()),
                Value::Real(f) => Cell::new(&f.to_string()),
                Value::Text(s) => Cell::new(s),
                Value::Blob(_) => Cell::new("BLOB"),
                Value::Null => Cell::new("NULL"),
            }
        }).collect();
        table.add_row(Row::new(row));
    }

    table.printstd();

    Ok(rows)
}