use csv::Writer;
use regex::Regex;
use scraper::{Html, Selector};
use std::error::Error;

fn extract_from_html_content(html: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let document = Html::parse_document(html);
    let table_selector = Selector::parse("table").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td, th").unwrap();

    let re = Regex::new(r#"<a href=".*?">(.*?)</a>"#).unwrap();

    let mut writer = Writer::from_path(file_path)?;

    for table in document.select(&table_selector) {
        for row in table.select(&row_selector) {
            let mut record = Vec::new();
            for cell in row.select(&cell_selector) {
                let cell_html = cell.inner_html();
                let cell_text = re
                    .captures(&cell_html)
                    .and_then(|cap| cap.get(1))
                    .map_or(cell_html.clone(), |m| m.as_str().to_string());
                record.push(cell_text);
            }
            writer.write_record(&record)?;
        }
    }

    writer.flush()?;

    Ok(())
}

#[tokio::main]
pub async fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let response: reqwest::Response = reqwest::get(url).await?;
    let html_content: String = response.text().await?;
    extract_from_html_content(&html_content, file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn test_extract_from_html_content() {
        let html = r#"
            <table>
                <tr><th>Player</th><th>Team</th></tr>
                <tr><td><a href="/players/a/achiupr01.html">Precious Achiuwa</a></td><td>Miami Heat</td></tr>
            </table>
        "#;
        let file_path = "test.csv";

        extract_from_html_content(html, file_path).unwrap();

        let contents = fs::read_to_string(file_path).unwrap();
        assert_eq!(contents, "Player,Team\nPrecious Achiuwa,Miami Heat\n");

        fs::remove_file(file_path).unwrap();
    }
}
