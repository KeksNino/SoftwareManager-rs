#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
mod rutracker;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    rutracker::search("test");
    let ui = AppWindow::new()?;

    ui.on_request_text_input(|text| {
        let text = text.trim();
        println!("User input: {}", text);
    });

    ui.run()?;

    Ok(())
}
