mod drop_data;
mod extract;
mod query;
mod transform_load;
mod update_db;
use std::collections::HashMap;

fn main() {
    match extract::extract(
        "https://www.basketball-reference.com/leagues/NBA_2023_per_game.html#per_game_stats",
        "nba.csv",
    ) {
        Ok(_) => println!("Extraction successful\n"),
        Err(e) => eprintln!("Extraction failed: {}\n", e),
    }

    match transform_load::create_and_load_db("nba.csv", "nba", "w") {
        Ok(_) => println!("Database creation successful\n"),
        Err(e) => eprintln!("Database creation failed: {}\n", e),
    }

    match query::query(
        "SELECT Player, MP, FGA, FT, FTA, ORB, DRB FROM nba LIMIT 5".to_string(),
        query::ConnectionType::DatabaseName("nba".to_string()),
    ) {
        Ok(_) => println!("Query successful \n"),
        Err(e) => eprintln!("Query failed: {} \n", e),
    }

    match drop_data::drop_data(
        drop_data::ConnectionType::DatabaseName("nba".to_string()),
        "nba",
        "Player = 'Steven Adams'",
    ) {
        Ok(_) => println!("Drop successful\n"),
        Err(e) => eprintln!("Drop failed: {} \n", e),
    }

    match query::query(
        "SELECT Player, MP, FGA, FT, FTA, ORB, DRB FROM nba LIMIT 4".to_string(),
        query::ConnectionType::DatabaseName("nba".to_string()),
    ) {
        Ok(_) => println!("Query successful\n"),
        Err(e) => eprintln!("Query failed: {}\n", e),
    }

    let mut update_values = HashMap::new();
    update_values.insert("MP".to_string(), "0".to_string());
    update_values.insert("FGA".to_string(), "0".to_string());
    update_values.insert("FT".to_string(), "0".to_string());
    update_values.insert("FTA".to_string(), "0".to_string());

    match update_db::update_db(
        drop_data::ConnectionType::DatabaseName("nba".to_string()),
        "nba",
        "Player = 'Santi Aldama'",
        update_values,
    ) {
        Ok(_) => println!("Update successful\n"),
        Err(e) => eprintln!("Update failed: {}\n", e),
    }

    match query::query(
        "SELECT Player, MP, FGA, FT, FTA, ORB, DRB FROM nba LIMIT 4".to_string(),
        query::ConnectionType::DatabaseName("nba".to_string()),
    ) {
        Ok(_) => println!("Query successful\n"),
        Err(e) => eprintln!("Query failed: {}\n", e),
    }
}
