use std::error::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CSVContact {
    pub name: String,
    pub groups: Vec<String>,
    pub emails: Vec<String>,
    pub phones: Vec<String>,
}

const GOOGLE_CSV_NAME_COLUMN: usize = 0;
const GOOGLE_CSV_GROUPS_COLUMN: usize = 29;
const GOOGLE_CSV_EMAIL_COLUMN: usize = 31;
const GOOGLE_CSV_PHONE_COLUMN: usize = 33;

pub fn load_contacts_csv(path: &str) -> Result<Vec<CSVContact>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;
    let mut contacts = Vec::new();

    for result in rdr.records() {
        let row = result?;

        contacts.push(CSVContact {
            name: row.get(GOOGLE_CSV_NAME_COLUMN).unwrap().trim().to_string(),
            groups: row
                .get(GOOGLE_CSV_GROUPS_COLUMN)
                .unwrap()
                .split(":::")
                .filter_map(|group| {
                    let group = group.trim();
                    if group.is_empty() || group.starts_with("*") {
                        None
                    } else {
                        Some(group.to_string())
                    }
                })
                .collect(),
            emails: vec![row.get(GOOGLE_CSV_EMAIL_COLUMN).unwrap().trim().to_string()],
            phones: vec![row.get(GOOGLE_CSV_PHONE_COLUMN).unwrap().trim().to_string()],
        });
    }

    Ok(contacts)
}
