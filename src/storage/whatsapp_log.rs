use std::{borrow::BorrowMut, error::Error};

#[derive(Debug, Clone)]
pub struct WSContact {
    pub name: String,
    pub ci: String,
    pub address: String,
    pub municipio: String,
    pub provincia: String,
    pub phone: String,
    // pub content: String,
}

#[derive(Debug, Clone, Default)]
pub struct LogEntry {
    pub date: String,
    pub time: String,
    pub sender: String,
    pub content: String,
}

#[derive(Debug, Clone, Default)]
pub struct Pedido {
    pub ci: Option<String>,
    pub phone: String,
    pub name: String,
    pub address: Option<(String, String, String)>,
    pub content: String,
}

pub fn sanitize_phones(phone_str: &str) -> Vec<String> {
    let phone_regex = regex::Regex::new(r"\+?[\d ]").expect("Can't compile regexp");

    let phones: Vec<String> = phone_str
        .split(|x| x == '-' || x == '/')
        .filter_map(|phone| {
            if !phone_regex.is_match(phone) || phone.chars().any(|c| c.is_alphabetic()) {
                None
            } else {
                Some(phone.trim().replace(" ", "").to_string())
            }
        })
        .collect();

    phones
}

impl Pedido {
    fn parse(text: String) -> Option<Self> {
        let lines = text.split("\n");

        //Phone formats:
        //T: +51 987654321
        //T: 987654321 - 0987654321
        //T: 987654321 / 0987654321
        //T: 987654321 Minerva

        for line in lines {
            if line.starts_with("T: ") {
                let phones = sanitize_phones(line.trim_start_matches("T: "));
                println!("line: {line} | {phones:?}");
            }
        }

        None
    }
}

pub fn load_contacts_from_whatsapp_dump(path: &str) -> Result<Vec<WSContact>, Box<dyn Error>> {
    let regex = regex::RegexBuilder::new(r"T: (.*)")
        .case_insensitive(false)
        .build()?;

    let log_entries: Vec<LogEntry> = load_log_entries(
        r"F:\Projects\delvin\WhatsApp Chat - ENVIOS305 - JAMONERA - ENVIOS RUBY\_chat.txt",
    )?;

    for entry in log_entries {
        if entry.content.contains("T: ") {
            if let Some(pedido) = Pedido::parse(entry.content) {
                // log::trace!("{pedido:?}");
            }
        }
    }

    todo!()
}

pub fn load_log_entries(path: &str) -> Result<Vec<LogEntry>, Box<dyn std::error::Error>> {
    // Create a regex to match log entry headers: [date, time] sender: content
    let header_ex = regex::Regex::new(r"\[([\d/]+), ([\d\:]+)\] (.*): (.*)")?;

    // Read the entire file content as a string
    let raw = std::fs::read_to_string(path)?;

    // Split the content by newline to process each line individually
    let lines = raw.split("\n");

    // Store the last log entry to append multiline messages
    let mut last_entry = None::<LogEntry>;

    // Vector to store all parsed log entries
    let mut entries: Vec<LogEntry> = vec![];

    for line in lines.map(|line| line.trim().trim_end_matches("?<Se editó este mensaje.>")) {
        // Match the line with the header regex
        if let Some(captures) = header_ex.captures(line) {
            // Create a new log entry from the captured groups
            let entry = LogEntry {
                date: captures.get(1).unwrap().as_str().to_string(),
                time: captures.get(2).unwrap().as_str().to_string(),
                sender: captures.get(3).unwrap().as_str().to_string(),
                content: captures.get(4).unwrap().as_str().to_string(),
            };

            // Push the last entry to the entries if it exists
            if let Some(last_entry_content) = last_entry {
                entries.push(last_entry_content);
            }

            // Set the current entry as the last entry
            last_entry = Some(entry);
        } else {
            // Append multiline content to the last entry
            if let Some(entry) = last_entry.borrow_mut() {
                entry.content = format!("{}\n{}", &entry.content, line);
            }
        }
    }

    Ok(entries)
}
