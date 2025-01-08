mod printer;

use crate::printer::{UsbPrinter, PRINTER_PID, PRINTER_VID};
use printer::{print_test, printer_status};
use std::sync::{Arc, Mutex};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = Arc::new(Mutex::new(
        UsbPrinter::build(PRINTER_VID, PRINTER_PID).expect("error while building printer"),
    ));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup({
            let state = state.clone();
            move |app| {
                app.manage(state);
                Ok(())
            }
        })
        .invoke_handler(tauri::generate_handler![print_test, printer_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
