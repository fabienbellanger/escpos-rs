use std::{cell::RefCell, ffi::c_void, rc::Rc};

pub use self::windows_printer::WindowsPrinter;
use crate::errors::{PrinterError, Result};
use windows::{
    core::{w, PWSTR},
    Win32::{
        Foundation::HANDLE,
        Graphics::Printing::{
            ClosePrinter, EndDocPrinter, EndPagePrinter, OpenPrinterW, StartDocPrinterW, StartPagePrinter,
            WritePrinter, DOC_INFO_1W,
        },
    },
};

use super::Driver;

mod windows_printer;

#[derive(Debug)]
pub struct WindowsDriver {
    printer_name: PWSTR,
    buffer: Rc<RefCell<Vec<u8>>>,
}

impl WindowsDriver {
    pub fn open(printer: &WindowsPrinter) -> Result<WindowsDriver> {
        Ok(Self {
            printer_name: printer.get_raw_name(),
            buffer: Rc::new(RefCell::new(Vec::new())),
        })
    }

    pub fn write_all(&self) -> Result<()> {
        let mut error = Option::None;
        let mut is_printer_start = false;
        let mut is_doc_start = false;
        let mut is_page_start = false;
        let mut printer_handle = HANDLE(0);
        #[allow(clippy::never_loop)]
        loop {
            unsafe {
                let mut document_name = w!("Raw Document").as_wide().to_vec();
                let mut document_type = w!("Raw").as_wide().to_vec();
                if OpenPrinterW(self.printer_name, &mut printer_handle, None).is_err() {
                    error = Some(PrinterError::Io("Failed to open printer".to_owned()));
                    break;
                }
                is_printer_start = true;

                let document_info = DOC_INFO_1W {
                    pDocName: PWSTR(document_name.as_mut_ptr()),
                    pOutputFile: PWSTR::null(),
                    pDatatype: PWSTR(document_type.as_mut_ptr()),
                };

                if StartDocPrinterW(printer_handle, 1, &document_info) == 0 {
                    error = Some(PrinterError::Io("Failed to start doc".to_owned()));
                    break;
                }
                is_doc_start = true;
                if StartPagePrinter(printer_handle).as_bool() == false {
                    error = Some(PrinterError::Io("Failed to start page".to_owned()));
                    break;
                }
                is_page_start = true;

                let mut written: u32 = 0;
                let buffer = self.buffer.borrow_mut();
                let buffer_len = buffer.len() as u32;

                if !WritePrinter(
                    printer_handle,
                    buffer.as_ptr() as *const c_void,
                    buffer_len,
                    &mut written,
                )
                .as_bool()
                {
                    error = Some(PrinterError::Io("Failed to write to printer".to_owned()));
                    break;
                } else {
                    if written != buffer_len {
                        error = Some(PrinterError::Io("Failed to write all bytes to printer".to_owned()));
                        break;
                    }
                }
            }
            break;
        }
        unsafe {
            if is_page_start {
                let _ = EndPagePrinter(printer_handle);
            }
            if is_doc_start {
                let _ = EndDocPrinter(printer_handle);
            }
            if is_printer_start {
                let _ = ClosePrinter(printer_handle);
            }
        }
        if let Some(err) = error {
            Err(err)
        } else {
            Ok(())
        }
    }
}

impl Driver for WindowsDriver {
    fn name(&self) -> String {
        "Windows Driver".to_owned()
    }

    fn write(&self, data: &[u8]) -> Result<()> {
        let mut buffer = self.buffer.borrow_mut();
        buffer.extend_from_slice(data);
        Ok(())
    }

    fn read(&self, _buf: &mut [u8]) -> Result<usize> {
        Ok(0)
    }

    fn flush(&self) -> Result<()> {
        self.write_all()
    }
}
