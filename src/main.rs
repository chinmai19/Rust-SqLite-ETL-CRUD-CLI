mod extract;

fn main() {
    match extract::extract(
        "https://www.basketball-reference.com/leagues/NBA_2023_per_game.html#per_game_stats",
        "nba.csv",
    ) {
        Ok(_) => println!("Extraction successful"),
        Err(e) => eprintln!("Extraction failed: {}", e),
    }
}
