use std::{borrow::BorrowMut, error::Error};

use storage::whatsapp_log::{load_log_entries, LogEntry};

pub mod storage;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .filter_module("whatsapp_scrapper", log::LevelFilter::Trace)
        .init();

    // let contact_filenames = vec!["Cancelados", "Clientes"];

    // let all_contacts: Vec<storage::csv::CSVContact> = contact_filenames
    //     .into_iter()
    //     .map(|filename| {
    //         let path = format!("../{filename}.csv");
    //         storage::csv::load_contacts_csv(&path).unwrap()
    //     })
    //     .flatten()
    //     .collect();

    let ws_contacts = storage::whatsapp_log::load_contacts_from_whatsapp_dump(
        r"F:\Projects\delvin\WhatsApp Chat - ENVIOS305 - JAMONERA - ENVIOS RUBY\_chat.txt",
    )?;

    // log::trace!("Got {} contacts", all_contacts.len());
    // log::trace!("Got {} whatsapp entries", ws_contacts.len());

    // for ws_entry in &ws_contacts {
    //     if ws_contacts
    //         .iter()
    //         .any(|contact| contact.phone == ws_entry.phone)
    //     {
    //         // log::trace!("Contact {} already registered!", ws_entry.ci);
    //     }else{
    //         log::trace!("New contact: {}", ws_entry.ci);
    //     }
    // }

    Ok(())
}
