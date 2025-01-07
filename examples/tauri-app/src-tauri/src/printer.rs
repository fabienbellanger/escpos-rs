use escpos::driver::{Driver, NetworkDriver};
use escpos::errors::PrinterError;
use escpos::printer::Printer;
use escpos::utils::{Protocol, RealTimeStatusRequest, RealTimeStatusResponse};
use serde::Serialize;
use std::time::Duration;

pub const PRINTER_ADDR: &str = "192.168.1.248";
pub const PRINTER_PORT: u16 = 9100;

#[derive(Debug, Serialize)]
pub struct CustomError {
    message: String,
}

impl From<PrinterError> for CustomError {
    fn from(error: PrinterError) -> Self {
        CustomError {
            message: format!("[Printer Error] {}", error),
        }
    }
}

#[tauri::command]
pub async fn print_test() -> Result<(), CustomError> {
    let driver = NetworkDriver::open(PRINTER_ADDR, PRINTER_PORT, Some(Duration::from_secs(2)))?;
    let mut printer = Printer::new(driver, Protocol::default(), None);
    printer.init()?.writeln("test")?.feed()?.print_cut()?;

    Ok(())
}

#[tauri::command]
pub async fn printer_status() -> Result<bool, CustomError> {
    let driver = NetworkDriver::open(PRINTER_ADDR, PRINTER_PORT, Some(Duration::from_secs(1)))?;
    Printer::new(driver.clone(), Protocol::default(), None)
        .real_time_status(RealTimeStatusRequest::Printer)?
        .send_status()?;

    let mut buf = [0; 1];
    driver.read(&mut buf)?;

    // Online/Offline status
    let status = RealTimeStatusResponse::parse(RealTimeStatusRequest::Printer, buf[0])?;
    let r = status.get(&RealTimeStatusResponse::Online).unwrap_or(&false);

    Ok(*r)
}
