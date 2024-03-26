use escpos::printer::Printer;
use escpos::utils::*;
use escpos::{driver::*, errors::Result};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    env_logger::init();

    let driver = NetworkDriver::open("192.168.1.248", 9100)?;
    // let driver = ConsoleDriver::open(true);
    // let driver = UsbDriver::open(0x0525, 0xa700, None)?;

    loop {
        Printer::new(driver.clone(), Protocol::default(), None)
            .debug_mode(Some(DebugMode::Dec))
            .real_time_status(RealTimeStatusRequest::Printer)?
            .real_time_status(RealTimeStatusRequest::RollPaperSensor)?
            .send_status()?;

        // From Epson documentation: if this command must be transmitted continuously,
        // it is possible to transmit up to 4 commands at once.
        let mut buf = [0; 2];
        driver.read(&mut buf)?;

        // Online/Offline status
        let status = RealTimeStatusResponse::parse(RealTimeStatusRequest::Printer, buf[0])?;
        println!(
            "Printer online: {}",
            status.get(&RealTimeStatusResponse::Online).unwrap_or(&false)
        );

        // Roll paper near-end sensor status
        let status = RealTimeStatusResponse::parse(RealTimeStatusRequest::RollPaperSensor, buf[1])?;
        println!(
            "Roll paper near-end sensor => paper near-end:  {}",
            status
                .get(&RealTimeStatusResponse::RollPaperNearEndSensorPaperNearEnd)
                .unwrap_or(&false)
        );

        sleep(Duration::from_secs(10));
    }
}
