mod drop_data;
mod extract;
mod query;
mod transform_load;
mod update_db;
use clap::{App, Arg};
use std::collections::HashMap;

fn main() {
    let matches = App::new("NBA Data Processor")
        .version("1.0")
        .author("Rakeen Rouf <rakeen.rouf@duke.edu>")
        .about("Processes NBA data")
        .arg(
            Arg::with_name("URL")
                .short('u')
                .long("url")
                .value_name("URL")
                .help("Sets the URL to extract data from")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CSV")
                .short('c')
                .long("csv")
                .value_name("CSV")
                .help("Sets the CSV file path")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("DB")
                .short('d')
                .long("db")
                .value_name("DB")
                .help("Sets the database name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("TABLE")
                .short('t')
                .long("table")
                .value_name("TABLE")
                .help("Sets the database table name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("MODE")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .help("Sets the mode for transform and load, overwrite (w) or append")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("QUERYSTRING")
                .short('q')
                .long("QUERYSTRING")
                .value_name("QUERYSTRING")
                .help("Query String")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CONDITION")
                .short('i')
                .long("CONDITION")
                .value_name("CONDITION")
                .help("Condtion to drop/update data")
                .takes_value(true),
        )
        .arg(
            Arg::new("UPDATE_VALUES")
                .short('v')
                .long("values")
                .value_name("UPDATE_VALUES")
                .help("Sets the update values (format: key1=value1,key2=value2,...)")
                .takes_value(true),
        )
        .arg(
            Arg::new("EXTRACT")
                .short('e')
                .long("extract")
                .value_name("EXTRACT")
                .help("Runs the extract function")
                .takes_value(false),
        )
        .arg(
            Arg::new("TRANSFORM")
                .short('h')
                .long("transform")
                .takes_value(false)
                .help("Run the transformation process"),
        )
        .arg(
            Arg::new("QUERY")
                .short('y')
                .long("query")
                .takes_value(false)
                .help("Run the query process"),
        )
        .arg(
            Arg::new("DROP")
                .short('p')
                .long("drop")
                .takes_value(false)
                .help("Run the drop data operation"),
        )
        .arg(
            Arg::new("UPDATE")
                .short('k')
                .long("update")
                .takes_value(false)
                .help("Run the update operation"),
        )
        .get_matches();

    let url = matches.value_of("URL").unwrap_or(
        "https://www.basketball-reference.com/leagues/NBA_2023_per_game.html#per_game_stats",
    );
    let csv = matches.value_of("CSV").unwrap_or("nba.csv");
    let db = matches.value_of("DB").unwrap_or("nba");
    let table = matches.value_of("TABLE").unwrap_or("nba");
    let mode = matches.value_of("MODE").unwrap_or("w");
    let query_string = matches
        .value_of("QUERYSTRING")
        .unwrap_or("SELECT Player, MP, FGA, FT, FTA, ORB, DRB FROM nba LIMIT 4");
    let condition = matches
        .value_of("CONDITION")
        .unwrap_or("Player = 'Steven Adams'");
    let update_vals = matches
        .value_of("UPDATE_VALUES")
        .unwrap_or("MP=0,FGA=0,FT=0,FTA=0");

    if matches.is_present("EXTRACT") {
        match extract::extract(url, csv) {
            Ok(_) => println!("Extraction successful\n"),
            Err(e) => eprintln!("Extraction failed: {}\n", e),
        }
    }

    if matches.is_present("TRANSFORM") {
        match transform_load::create_and_load_db(csv, db, mode) {
            Ok(_) => println!("Database creation successful\n"),
            Err(e) => eprintln!("Database creation failed: {}\n", e),
        }
    }

    if matches.is_present("QUERY") {
        match query::query(
            query_string.to_string(),
            query::ConnectionType::DatabaseName(db.to_string()),
        ) {
            Ok(_) => println!("Query successful \n"),
            Err(e) => eprintln!("Query failed: {} \n", e),
        }
    }

    if matches.is_present("DROP") {
        match drop_data::drop_data(
            drop_data::ConnectionType::DatabaseName(db.to_string()),
            table,
            condition,
        ) {
            Ok(_) => println!("Drop successful\n"),
            Err(e) => eprintln!("Drop failed: {} \n", e),
        }
    }

    // let mut update_values = HashMap::new();
    // update_values.insert("MP".to_string(), "0".to_string());
    // update_values.insert("FGA".to_string(), "0".to_string());
    // update_values.insert("FT".to_string(), "0".to_string());
    // update_values.insert("FTA".to_string(), "0".to_string());

    if matches.is_present("UPDATE") {
        let mut update_values = HashMap::new();
        let values = update_vals;
        for pair in values.split(',') {
            let mut parts = pair.splitn(2, '=');
            if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                update_values.insert(key.to_string(), value.to_string());
            }
        }

        match update_db::update_db(
            drop_data::ConnectionType::DatabaseName(db.to_string()),
            table,
            condition,
            update_values,
        ) {
            Ok(_) => println!("Update successful\n"),
            Err(e) => eprintln!("Update failed: {}\n", e),
        }
    }
}
