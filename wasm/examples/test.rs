use anyhow::Result;
use mtg_print::print::PrintJob;
use std::{fs::File, io::Write};

fn main() -> Result<()> {
    let cards = [
        // Ledger shredder
        (4, "7ea4b5bc-18a4-45db-a56a-ab3f8bd2fb0d"),
        // Picklock Prankster
        (4, "5ebac73a-1ecf-4e6d-87b1-ea560bfeb064"),
    ];

    let mut job = PrintJob::new();

    for (count, id) in cards {
        let request = reqwest::blocking::get(&format!(
            "https://api.scryfall.com/cards/{id}?format=image&version=png&face=front"
        ))?;
        let bytes = request.bytes()?;

        job.add_card(count, Box::from(bytes.as_ref()));
    }

    let result = job.print()?;

    let mut out = File::create("out.pdf")?;
    out.write_all(&result)?;

    Ok(())
}
