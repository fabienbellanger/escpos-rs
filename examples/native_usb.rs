use escpos::driver::*;
use escpos::errors::Result;
use escpos::printer::Printer;
use escpos::utils::*;

fn main() -> Result<()> {
    env_logger::init();

    // List USB devices
    for device in nusb::list_devices().unwrap() {
        println!(
            "Bus: {:03} address: {:03} VID: {:04x} PID: {:04x} Manufacturer: {} Product: {} S/N: {}",
            device.bus_number(),
            device.device_address(),
            device.vendor_id(),
            device.product_id(),
            device.manufacturer_string().unwrap_or_default(),
            device.product_string().unwrap_or_default(),
            device.serial_number().unwrap_or_default(),
        );
    }

    let driver = UsbNativeDriver::open(0x0525, 0xa700)?;
    Printer::new(driver, Protocol::default(), None)
        .debug_mode(Some(DebugMode::Dec))
        .init()?
        .writeln("Native USB test")?
        .print_cut()?;

    Ok(())
}
