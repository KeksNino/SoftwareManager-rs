#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod rutracker;
use slint::{ModelRc, StandardListViewItem, VecModel};
use std::error::Error;
use std::rc::Rc;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_request_text_input(|text| {
        println!("User input: {}", text);
        let _response = rutracker::search(&text);
    });

    let table_vec: Vec<ModelRc<StandardListViewItem>> = vec![];
    let table_model = Rc::new(VecModel::from(table_vec));

    let test = "test";
    ui.set_table_data(table_model.to_owned().into());
    ui.on_add_row({
        move || {
            table_model.push(VecModel::from_slice(&[
                StandardListViewItem::from(slint::SharedString::from(test)),
                //StandardListViewItem::from("<new>"),
            ]));
        }
    });

    ui.run().unwrap();

    ui.run()?;

    Ok(())
}
