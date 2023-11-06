mod drop_data;
#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unused_assignments)]
#[allow(unused_must_use)]
#[allow(unused_parens)]
mod extract;
mod query;
mod transform_load;

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
        "SELECT Player, MP, FGA, FT, FTA, ORB, DRB FROM nba LIMIT 5".to_string(),
        query::ConnectionType::DatabaseName("nba".to_string()),
    ) {
        Ok(_) => println!("Query successful\n"),
        Err(e) => eprintln!("Query failed: {}\n", e),
    }
}
