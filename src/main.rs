#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
mod rutracker;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_request_text_input(|text| {
        println!("User input: {}", text);
        let response = rutracker::search(&text);
    });

    ui.run()?;

    Ok(())
}
