// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod printer;

use escpos::driver::UsbDriver;
use escpos::errors::PrinterError;
use escpos::printer::Printer;
use escpos::utils::Protocol;
use printer::{print_test, printer_status};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::Manager;

const VID: u16 = 0x0416;
const PID: u16 = 0x5011;

pub struct MyPrinter {
    port: Printer<UsbDriver>,
}

impl MyPrinter {
    pub fn build(vid: u16, pid: u16) -> Result<Self, PrinterError> {
        let driver = UsbDriver::open(vid, pid, Some(Duration::from_secs(2)))?;
        let printer = Printer::new(driver, Protocol::default(), None);

        Ok(Self { port: printer })
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = Arc::new(Mutex::new(
        MyPrinter::build(VID, PID).expect("error while building printer"),
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
