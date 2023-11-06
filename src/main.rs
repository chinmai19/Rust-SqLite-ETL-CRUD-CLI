mod extract;
mod transform_load;

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
}
