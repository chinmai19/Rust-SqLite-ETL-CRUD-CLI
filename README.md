#  Rist SQLite ETL CRUD CLI
#### By Rakeen Rouf 

[![Build binary release](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/release.yml/badge.svg)](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/release.yml)
[![Clippy](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/lint.yml/badge.svg)](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/lint.yml) [![Rustfmt](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/rustfmt.yml) [![Tests](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/tests.yml/badge.svg)](https://github.com/rmr327/Rust-SqLite-ETL-CRUD-CLI/actions/workflows/tests.yml)

## Overview

This project serves as a comprehensive showcase of a `Rust` Command Line Interface (`CLI`) tool tailored for seamless data management. It seamlessly handles the Extract, Transform, and Load (`ETL`) process, efficiently transferring data into a SQLite database. In addition, this repository exemplifies crucial `CRUD` (Create, Read, Update, Delete) operations, offering a holistic understanding of database management.

Explore the power of this custom CLI, empowering users to effortlessly handle data, from acquisition to advanced database operations.

Note: Feel free to customize, enhance, and extend the capabilities of this versatile tool to suit your unique needs.

## Project Architecture

![Alt text](https://user-images.githubusercontent.com/36940292/281111426-7cc28c47-0136-40e0-bdbd-c7d52baf075d.png)

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
#### `Help` 
```
cargo run -- --help
```

#### `Extract` 2022-2023 NBA player stats from the Basketball Reference website
```
cargo run -- --extract -u "https://www.basketball-reference.com/leagues/NBA_2023_per_game.html#per_game_stats"
```

![Alt text](https://user-images.githubusercontent.com/36940292/281097227-eeda0210-ff2f-4c7e-a8e7-c5fc539f7294.png)

#### `Creating` a new local Sqlite database for the 2022 2023 NBA player stats
```
cargo run -- --transform -c "nba.csv" -d "nba" -m "w"
```

![](https://user-images.githubusercontent.com/36940292/281097977-8487c97c-38cb-421d-b84e-7d0a07b01ffc.png)


#### `Reading` data from the database
```
cargo run -- --query -q "SELECT Player, MP, FGA, FT, FTA, ORB, DRB FROM nba LIMIT 4" -d "nba"
```
![Alt text](https://user-images.githubusercontent.com/36940292/281099411-3065794d-4682-45a0-8791-9fcc93cb48bd.png)


#### `Updating` data in the database, where player is Steven Adams
```
cargo run -- --update --values "MP=0,FGA=0,FT=0,FTA=0" --condition "Player = 'Steven Adams'""
```

![Alt text](https://user-images.githubusercontent.com/36940292/281100809-172ae947-c4da-4e34-b6a7-26d99902e1ae.png)


#### `Deleting` all instances where player is Steven Adams
```
cargo run -- --drop -d "nba" --condition "Player = 'Steven Adams'" 
```

![Alt text](https://user-images.githubusercontent.com/36940292/281102605-e4e3c9ec-079f-43a0-be38-ecd3554f05e9.png)

## Performance

This application is written in Rust, which provides several advantages over other languages like Python. Rust typically offers better CPU and GPU performance compared to Python. This is due to Rust being a statically compiled language, which means it compiles directly to machine code, resulting in less runtime overhead. Rust's low-level control and strict memory management contribute to efficient resource utilization, especially in performance-critical applications. Additionally, Rust's ownership system and safe concurrency features allow for effective utilization of multiple CPU cores. While Python has a rich library ecosystem, Rust's performance advantages make it a compelling choice for projects where speed and resource efficiency are paramount.

## Rust Error Handling

Rust's approach to error handling sets it apart from many other programming languages. It employs a type called Result, which can signify either a successful outcome (Ok) or an error (Err). This enforces explicit error handling, reducing the chances of overlooking or ignoring potential issues. Rust encourages pattern matching to handle different error cases, ensuring comprehensive error management. The language also provides the ? operator to streamline error propagation, making code more concise and readable. With its robust error handling mechanisms, Rust promotes code reliability and helps developers create more robust and predictable software.


## Efficiency and Limitations of SQLite and SQL

SQLite and SQL greatly enhance data analysis efficiency. The lightweight nature of SQLite makes it a fast and accessible choice for smaller projects or local applications. Its simplicity and self-contained architecture streamline setup and deployment. However, for larger datasets (~>280 TB) or scenarios requiring concurrent access from multiple users, more robust database systems may be more suitable. Additionally, while SQLite supports most standard SQL operations, it may have limitations in handling very large datasets or complex operations that some enterprise-level databases can manage more effectively.

## Note on Generative AI for Rust development

Generative AI, exemplified by tools like OpenAI's Copilot, has proven to be a valuable asset in writing Rust code. Leveraging Copilot's intelligent code suggestions and completions, I was able to efficiently transform my Python SQLite library into Rust. Copilot's ability to understand context and generate syntactically correct Rust code greatly expedited the conversion process. It provided insightful suggestions, improved code readability, and accelerated overall development. By automating routine tasks and offering helpful hints, generative AI like Copilot empowers developers to transition smoothly between programming languages, ultimately enhancing productivity and the quality of the resulting Rust code.


## Optimized Rust Binary

An optimized Rust binary refers to an executable file produced from Rust source code after undergoing various compiler-level optimizations. These optimizations are applied by the Rust compiler during the compilation process to improve the performance and efficiency of the resulting binary. They can include techniques such as inlining functions, removing dead code, reordering instructions for better CPU pipelining, and other transformations aimed at making the program run faster and consume fewer system resources. The result is a binary file that exhibits enhanced performance characteristics compared to an unoptimized build. These optimized binaries are well-suited for deployment in production environments where efficiency and speed are critical concerns. The below binary file can be downloaded from the actions tab of this repositoty (in artifacts section of the `Build and Package Rust Binary` subtab).

![Alt text](https://user-images.githubusercontent.com/36940292/281120825-56ca1f1e-5cff-4c19-bfc1-7d75762aec22.png)