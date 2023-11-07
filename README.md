#  Rist SQLite ETL CRUD CLI
#### By Rakeen Rouf 

[![Build binary release](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/release.yml/badge.svg)](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/release.yml)
[![Clippy](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/lint.yml/badge.svg)](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/lint.yml) [![Rustfmt](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/rustfmt.yml) [![Tests](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/tests.yml/badge.svg)](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/tests.yml)

## Overview

This project serves as a comprehensive showcase of a `Rust` Command Line Interface (`CLI`) tool tailored for seamless data management. It seamlessly handles the Extract, Transform, and Load (`ETL`) process, efficiently transferring data into a SQLite database. In addition, this repository exemplifies crucial `CRUD` (Create, Read, Update, Delete) operations, offering a holistic understanding of database management.

Explore the power of this custom CLI, empowering users to effortlessly handle data, from acquisition to advanced database operations.

Note: Feel free to customize, enhance, and extend the capabilities of this versatile tool to suit your unique needs.

## Project Architecture

![Alt text](https://user-images.githubusercontent.com/36940292/280939916-ecee2416-e328-46b8-9afd-1b319c981328.png)

## Function Descriptions (Located in ~/mylib)

- `extract_from_html_content(html: &str, file_path: &str) -> Result<(), Box<dyn Error>>` (in extract.rs)

This fucntion extracts table data from the passed url to a csv file. The function parses the HTML content, selects all tables in it, and then iterates over each table's rows and cells. It extracts the text from each cell and writes it to a record in the CSV file. The data must be in an HTML table format. If the function succeeds, it doesn't return anything. If it failes it returns an error. The exact type of the error is determined at runtime. 

- `create_and_load_db(dataset: &str, db_name: &str, mode: &str, ) -> Result<(), Box<dyn std::error::Error>>` (in transform_load.rs)

Function to create a new local SQLite3 database and load data into it. The data is transformed from a CSV file with appropriate format changes made. The function first creates a new SQLite database with the specified name. It then opens the CSV file and reads its contents. For each record in the CSV file, it inserts a new row into the database.

- `update_db(conn_type: ConnectionType, table_name: &str, condition: &str, update_values: HashMap<String, String>, ) -> Result<(), Box<dyn std::error::Error>>` (in update_db.rs)

The function first establishes a connection to the SQLite database. It then constructs an SQL UPDATE statement based on the provided table name, condition, and update values. This statement is executed against the database to update the specified rows. The `update_db` function takes four arguments: `conn_type`, `table_name`, `condition`, and `update_values`. `conn_type` is an enum that specifies the type of connection to the database, `table_name` is the name of the table to update, `condition` is a string that specifies the condition for the rows to update, and `update_values` is a HashMap that maps column names to new values.

- `query(query_str: String, conn_type: ConnectionType) -> Result<Vec<Vec<Value>>>` (in query.rs)

Function to query the database based on the passed query string. If query string is left blank then a default query is performed on the `nba` table. `conn_type` is an enum that specifies the type of connection to the database.

- `drop_data(conn_type: ConnectionType, table_name: &str, condition: &str) -> Result<()>` (in drop_data.rs)

Function to drop data based on the passed condition and table. The `drop_data` function takes three arguments: `conn_type`, `table_name`, and `condition`. `conn_type` is an enum that specifies the type of connection to the database, `table_name` is the name of the table to delete data from, and `condition` is a string that specifies the condition for the rows to delete.

## Usage

The application accepts several command-line arguments to control its behavior. Here's a brief overview:

    -c, --csv <CSV>                    Sets the CSV file path
    -d, --db <DB>                      Sets the database name
    -e, --extract                      Runs the extract function
    -h, --transform                    Run the transformation and load process
        --help                         Print help information
    -i, --CONDITION <CONDITION>        Condtion to drop/update data
    -k, --update                       Run the update operation
    -m, --mode <MODE>                  Sets the mode for transform and load, overwrite (w) or append
    -p, --drop                         Run the drop data operation
    -y, --query                        Run a query
    -q, --QUERYSTRING <QUERYSTRING>    Query String
    -t, --table <TABLE>                Sets the database table name
    -u, --url <URL>                    Sets the URL to extract data from
    -v, --values <UPDATE_VALUES>       Sets the update values (format: key1=value1,key2=value2,...)
    -V, --version                      Print version information

## Example Usage

```
cargo run -- --help
```
This will show you the help menu.

```
cargo run -- --extract -u "https://www.basketball-reference.com/leagues/NBA_2023_per_game.html#per_game_stats"
```

```
cargo run -- --extract -u "https://www.basketball-reference.com/leagues/NBA_2023_per_game.html#per_game_stats"
```

```
cargo run -- --extract -u "https://www.basketball-reference.com/leagues/NBA_2023_per_game.html#per_game_stats"
```

```
cargo run -- --extract -u "https://www.basketball-reference.com/leagues/NBA_2023_per_game.html#per_game_stats"
```


## Performance

This application is written in Rust, which provides several advantages over other languages like Python. Rust programs are generally faster and use less CPU. However, the memory usage can be comparable in some cases, such as when using recursive algorithms that create a large number of stack frames.

### Efficiency and Limitations of SQLite and SQL

SQLite and SQL greatly enhance data analysis efficiency. The lightweight nature of SQLite makes it a fast and accessible choice for smaller projects or local applications. Its simplicity and self-contained architecture streamline setup and deployment. However, for larger datasets (~>280 TB) or scenarios requiring concurrent access from multiple users, more robust database systems may be more suitable. Additionally, while SQLite supports most standard SQL operations, it may have limitations in handling very large datasets or complex operations that some enterprise-level databases can manage more effectively.
