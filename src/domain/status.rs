//! Printer status
//! https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/dle_eot.html

/// Printer real-time status
#[derive(Debug, Copy, Clone)]
pub enum RealTimeStatus {
    Printer,
    OfflineCause,
    ErrorCause,
    RollPaperSensor,
    InkA,
    InkB,
    Peeler,
    Interface,
    DMD,
}

impl From<RealTimeStatus> for (u8, Option<u8>) {
    fn from(value: RealTimeStatus) -> Self {
        match value {
            RealTimeStatus::Printer => (1, None),
            RealTimeStatus::OfflineCause => (2, None),
            RealTimeStatus::ErrorCause => (3, None),
            RealTimeStatus::RollPaperSensor => (4, None),
            RealTimeStatus::InkA => (7, Some(1)),
            RealTimeStatus::InkB => (7, Some(2)),
            RealTimeStatus::Peeler => (8, Some(3)),
            RealTimeStatus::Interface => (18, Some(1)),
            RealTimeStatus::DMD => (18, Some(2)),
        }
    }
}

impl RealTimeStatus {
    pub fn to_str(&self, binary: u8) -> &str {
        let binary = format!("{:08b}", binary)
            .chars()
            .map(|c| c.to_digit(2).unwrap_or(0))
            .rev()
            .collect::<Vec<_>>();
        dbg!(&binary);

        match self {
            &Self::Printer => todo!(),
            _ => todo!(),
        }
    }
}
