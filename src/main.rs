#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unused_assignments)]
#[allow(unused_must_use)]
#[allow(unused_parens)]
mod extract;
mod transform_load;
mod query;

fn main() {
    match extract::extract(
        "https://www.basketball-reference.com/leagues/NBA_2023_per_game.html#per_game_stats",
        "nba.csv",
    ) {
        Ok(_) => println!("Extraction successful"),
        Err(e) => eprintln!("Extraction failed: {}", e),
    }

    match transform_load::create_and_load_db("nba.csv", "nba", "w") {
        Ok(_) => println!("Database creation successful"),
        Err(e) => eprintln!("Database creation failed: {}", e),
    }

    match query::query(
        "SELECT RK, Player, MP, FGA, FT, FTA, ORB, DRB FROM nba LIMIT 5".to_string(),
        "nba".to_string()
    ) {
        Ok(_) => println!("Query successful"),
        Err(e) => eprintln!("Query failed: {}", e),
    }
}
