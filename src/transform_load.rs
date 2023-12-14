use csv::Reader;
use regex::Regex;
// use rusqlite::types::Type;
use rusqlite::{params, Connection, Result};
use std::fs::File;
use std::io::BufReader;

// modifies column names to be valid in SQL
fn modify_name(name: String) -> String {
    let mut new_name = name;
    if new_name.contains('%') {
        new_name = new_name.replace('%', "Perc");
    }
    if let Some(first_char) = new_name.chars().next() {
        if first_char.is_ascii_digit() {
            new_name = format!("{}{}", &new_name[1..], &new_name[0..1]);
        }
    }

    if new_name.is_empty() {
        new_name = "ID".to_string();
    };
    let new_name = match new_name.as_str() {
        "Tm" => "Team".to_string(),
        "PTS" => "Points".to_string(),
        "PF" => "Fouls".to_string(),
        "TOV" => "Turnovers".to_string(),
        "BLK" => "Blocks".to_string(),
        "STL" => "Steals".to_string(),
        "AST" => "Assists".to_string(),

        "G" => "Games".to_string(),
        "GS" => "Games_Started".to_string(),
        "MP" => "Minutes_Played_Per_Game".to_string(),
        "FG" => "Field_Goals".to_string(),
        "FGA" => "Field_Goal_Attempts".to_string(),
        "FT" => "Free_Throws".to_string(),
        "FTPerc" => "Free_Throws_Perc".to_string(),
        "FTA" => "Free_Throws_Attempts".to_string(),
        "ORB" => "Offensive_Rebounds".to_string(),
        "DRB" => "Defensive_Rebounds".to_string(),
        "Pos" => "Position".to_string(),
        "P3" => "Field_Goals_3_point".to_string(),
        "PA3" => "Goals_3_point_Attempts".to_string(),
        "P2" => "Field_Goals_2_point".to_string(),
        "PA2" => "Goals_2_point_Attempts".to_string(),
        "PPerc2" => "Perc_2_point".to_string(),
        "eFGPerc" => "Effective_Goal_Perc".to_string(),
        "TRB" => "Rebounbs".to_string(),

        _ => new_name.to_string(),
    };
    println!("{}", new_name);
    new_name
}

pub fn create_and_load_db(
    dataset: &str,
    db_name: &str,
    mode: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(dataset).unwrap();
    let mut rdr = Reader::from_reader(BufReader::new(file));
    let headers = rdr.headers().unwrap().clone();

    // Some column names are not valid in SQL, so we need to adjust them
    let mut column_names: Vec<String> = headers
        .iter()
        .map(|name| modify_name(name.to_string()))
        .collect();

    // Some column names are not valid in SQL, so we need to replace them
    let chars_to_replace = [" ", "/", "-", "(", ")", "&", "'", ",", ".", "+", ":", "\""];
    for char in chars_to_replace.iter() {
        column_names = column_names
            .iter()
            .map(|name| name.replace(char, "_"))
            .collect();
    }

    // Defining column types based on regex
    let mut column_types = vec![];
    for value in rdr.records().next().unwrap().unwrap().iter() {
        if Regex::new(r"^-?\d+(?:\.\d+)?$").unwrap().is_match(value) {
            column_types.push("REAL");
        } else if Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap().is_match(value) {
            column_types.push("DATE");
        } else {
            column_types.push("TEXT");
        }
    }

    let conn = Connection::open(db_name)?;

    // let c = conn.execute_batch("");

    if mode == "w" {
        conn.execute(&format!("DROP TABLE IF EXISTS {}", db_name), params![])?;

        let s = column_names
            .iter()
            .enumerate()
            .map(|(i, name)| format!("{} {}", name, column_types[i]))
            .collect::<Vec<String>>()
            .join(", ");

        conn.execute(&format!("CREATE TABLE {} ({})", db_name, s), params![])?;
    }

    let mut stmt = conn.prepare(&format!(
        "INSERT INTO {} VALUES ({})",
        db_name,
        vec!["?".to_string(); column_names.len()].join(", ")
    ))?;

    for result in rdr.records() {
        let record = result?;
        let values: Vec<String> = record
            .iter()
            .map(|s| {
                if !s.is_empty() {
                    s.to_string()
                } else {
                    "0".to_string()
                }
            })
            .collect();
        let params: Vec<&(dyn rusqlite::ToSql)> =
            values.iter().map(|x| x as &(dyn rusqlite::ToSql)).collect();
        stmt.execute(params.as_slice())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_modify_name() {
        println!("{}", modify_name("1test".to_string()));
        assert_eq!(modify_name("1test".to_string()), "test1");
        assert_eq!(modify_name("%test".to_string()), "Perctest");
        assert_eq!(modify_name("".to_string()), "ID");
    }

    #[test]
    fn test_create_and_load_db() {
        // Create a test dataset
        let test_dataset = "test_dataset.csv";
        fs::write(test_dataset, "column1,column2\nvalue1,value2").unwrap();

        // Create a test database
        let test_db = "test_db";

        // Call the function with the test inputs
        let result = create_and_load_db(test_dataset, test_db, "w");

        // Check if the function succeeded
        assert!(result.is_ok());

        // Clean up
        fs::remove_file(test_dataset).unwrap();
        fs::remove_file(test_db).unwrap();
    }
}
