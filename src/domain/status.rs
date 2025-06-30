//! Printer status
//!
//! [Epson Documentation](https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/dle_eot.html)

use crate::errors::PrinterError;
use std::collections::HashMap;

/// Printer real-time status
#[derive(Debug, Copy, Clone)]
pub enum RealTimeStatusRequest {
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

impl From<RealTimeStatusRequest> for (u8, u8) {
    fn from(value: RealTimeStatusRequest) -> Self {
        match value {
            RealTimeStatusRequest::Printer => (1, 0),
            RealTimeStatusRequest::OfflineCause => (2, 0),
            RealTimeStatusRequest::ErrorCause => (3, 0),
            RealTimeStatusRequest::RollPaperSensor => (4, 0),
            RealTimeStatusRequest::InkA => (7, 1),
            RealTimeStatusRequest::InkB => (7, 2),
            RealTimeStatusRequest::Peeler => (8, 3),
            RealTimeStatusRequest::Interface => (18, 1),
            RealTimeStatusRequest::DMD => (18, 2),
        }
    }
}

/// Printer real-time status response
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RealTimeStatusResponse {
    // Printer status
    DrawerKickOutConnectorPin3Low,
    Online,
    WaitingForOnlineRecovery,
    PaperFeedButtonPressed,

    // Offline cause status
    CoverClosed,
    PaperFedByPaperFeedButton,
    PrintingStopsDueToPaperEnd,
    ErrorOccurred,

    // Error cause status
    RecoverableErrorOccurred,
    AutocutterErrorOccurred,
    UnrecoverableErrorOccurred,
    AutoRecoverableErrorOccurred,

    // Roll paper sensor status
    RollPaperNearEndSensorPaperAdequate,
    RollPaperEndSensorPaperPresent,

    // Inks (A and B) status
    InkNearEndDetected,
    InkEndDetected,
    InkCartridgeDetected,
    CleaningPerformed,

    // Peeler status
    WaitingForLabelToBeRemoved,
    PaperPresentInLabelPeelingDetector,

    // Interface status
    PrintingMultipleInterfacesEnabled,

    // DM-D status
    DMDTransmissionStatusReady,
}

impl RealTimeStatusResponse {
    /// Check if the pattern is valid, i.e. 0xx1xx10b ([0, 1, x, x, 1, x, x, 0])
    fn is_pattern_valid(data: &[u8]) -> bool {
        if data.len() != 8 {
            return false;
        }

        if data[0] != 0 || data[1] != 1 || data[4] != 1 || data[7] != 0 {
            return false;
        }

        true
    }

    /// Parse the response
    pub fn parse(request: RealTimeStatusRequest, response: u8) -> Result<HashMap<Self, bool>, PrinterError> {
        let binary = format!("{response:08b}")
            .chars()
            .map(|c| c.to_digit(2).unwrap_or(0) as u8)
            .rev()
            .collect::<Vec<_>>();

        if !Self::is_pattern_valid(&binary) {
            return Err(PrinterError::InvalidResponse(format!(
                "invalid response pattern: {response:08b} (0xx1xx10 expected)"
            )));
        }

        let mut result = HashMap::new();
        match request {
            RealTimeStatusRequest::Printer => {
                result.insert(Self::DrawerKickOutConnectorPin3Low, binary[2] == 0);
                result.insert(Self::Online, binary[3] == 0);
                result.insert(Self::WaitingForOnlineRecovery, binary[5] == 1);
                result.insert(Self::PaperFeedButtonPressed, binary[6] == 1);
            }
            RealTimeStatusRequest::OfflineCause => {
                result.insert(Self::CoverClosed, binary[2] == 0);
                result.insert(Self::PaperFedByPaperFeedButton, binary[3] == 1);
                result.insert(Self::PrintingStopsDueToPaperEnd, binary[5] == 1);
                result.insert(Self::ErrorOccurred, binary[6] == 1);
            }
            RealTimeStatusRequest::ErrorCause => {
                result.insert(Self::RecoverableErrorOccurred, binary[2] == 1);
                result.insert(Self::AutocutterErrorOccurred, binary[3] == 1);
                result.insert(Self::UnrecoverableErrorOccurred, binary[5] == 1);
                result.insert(Self::AutoRecoverableErrorOccurred, binary[6] == 1);
            }
            RealTimeStatusRequest::RollPaperSensor => {
                result.insert(
                    Self::RollPaperNearEndSensorPaperAdequate,
                    binary[2] == 0 && binary[3] == 0,
                );
                result.insert(Self::RollPaperEndSensorPaperPresent, binary[5] == 0 && binary[6] == 0);
            }
            RealTimeStatusRequest::InkA => {
                result.insert(Self::InkNearEndDetected, binary[2] == 1);
                result.insert(Self::InkEndDetected, binary[3] == 1);
                result.insert(Self::InkCartridgeDetected, binary[5] == 0);
                result.insert(Self::CleaningPerformed, binary[6] == 1);
            }
            RealTimeStatusRequest::InkB => {
                result.insert(Self::InkNearEndDetected, binary[2] == 1);
                result.insert(Self::InkEndDetected, binary[3] == 1);
                result.insert(Self::InkCartridgeDetected, binary[5] == 0);
            }
            RealTimeStatusRequest::Peeler => {
                result.insert(Self::WaitingForLabelToBeRemoved, binary[2] == 1);
                result.insert(Self::PaperPresentInLabelPeelingDetector, binary[5] == 0);
            }
            RealTimeStatusRequest::Interface => {
                result.insert(Self::PrintingMultipleInterfacesEnabled, binary[2] == 1);
            }
            RealTimeStatusRequest::DMD => {
                result.insert(Self::DMDTransmissionStatusReady, binary[2] == 0);
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_real_time_status_request_to_u8_tuple() {
        let mut result: (u8, u8) = RealTimeStatusRequest::Printer.into();
        assert_eq!(result, (1, 0));

        result = RealTimeStatusRequest::OfflineCause.into();
        assert_eq!(result, (2, 0));

        result = RealTimeStatusRequest::ErrorCause.into();
        assert_eq!(result, (3, 0));

        result = RealTimeStatusRequest::RollPaperSensor.into();
        assert_eq!(result, (4, 0));

        result = RealTimeStatusRequest::InkA.into();
        assert_eq!(result, (7, 1));

        result = RealTimeStatusRequest::InkB.into();
        assert_eq!(result, (7, 2));

        result = RealTimeStatusRequest::Peeler.into();
        assert_eq!(result, (8, 3));

        result = RealTimeStatusRequest::Interface.into();
        assert_eq!(result, (18, 1));

        result = RealTimeStatusRequest::DMD.into();
        assert_eq!(result, (18, 2));
    }

    #[test]
    fn test_is_pattern_valid() {
        let data = [0, 1, 0, 1, 1, 1, 0, 0];
        assert!(RealTimeStatusResponse::is_pattern_valid(&data));

        let data = [0, 1, 0, 1, 1, 1, 0];
        assert!(!RealTimeStatusResponse::is_pattern_valid(&data));

        let data = [0, 1, 0, 1, 1, 1, 0, 1];
        assert!(!RealTimeStatusResponse::is_pattern_valid(&data));
    }

    #[test]
    fn test_parse_real_time_status_response() {
        let response = RealTimeStatusResponse::parse(RealTimeStatusRequest::Printer, 0b00011010).unwrap();
        assert_eq!(response[&RealTimeStatusResponse::DrawerKickOutConnectorPin3Low], true);
        assert_eq!(response[&RealTimeStatusResponse::Online], false);
        assert_eq!(response[&RealTimeStatusResponse::WaitingForOnlineRecovery], false);
        assert_eq!(response[&RealTimeStatusResponse::PaperFeedButtonPressed], false);

        let response = RealTimeStatusResponse::parse(RealTimeStatusRequest::OfflineCause, 0b01011110).unwrap();
        assert_eq!(response[&RealTimeStatusResponse::CoverClosed], false);
        assert_eq!(response[&RealTimeStatusResponse::PaperFedByPaperFeedButton], true);
        assert_eq!(response[&RealTimeStatusResponse::PrintingStopsDueToPaperEnd], false);
        assert_eq!(response[&RealTimeStatusResponse::ErrorOccurred], true);

        let response = RealTimeStatusResponse::parse(RealTimeStatusRequest::ErrorCause, 0b00011010).unwrap();
        assert_eq!(response[&RealTimeStatusResponse::RecoverableErrorOccurred], false);
        assert_eq!(response[&RealTimeStatusResponse::AutocutterErrorOccurred], true);
        assert_eq!(response[&RealTimeStatusResponse::UnrecoverableErrorOccurred], false);
        assert_eq!(response[&RealTimeStatusResponse::AutoRecoverableErrorOccurred], false);

        let response = RealTimeStatusResponse::parse(RealTimeStatusRequest::RollPaperSensor, 0b00010010).unwrap();
        assert_eq!(
            response[&RealTimeStatusResponse::RollPaperNearEndSensorPaperAdequate],
            true
        );
        assert_eq!(response[&RealTimeStatusResponse::RollPaperEndSensorPaperPresent], true);

        let response = RealTimeStatusResponse::parse(RealTimeStatusRequest::InkA, 0b01011010).unwrap();
        assert_eq!(response[&RealTimeStatusResponse::InkNearEndDetected], false);
        assert_eq!(response[&RealTimeStatusResponse::InkEndDetected], true);
        assert_eq!(response[&RealTimeStatusResponse::InkCartridgeDetected], true);
        assert_eq!(response[&RealTimeStatusResponse::CleaningPerformed], true);

        let response = RealTimeStatusResponse::parse(RealTimeStatusRequest::InkB, 0b01011010).unwrap();
        assert_eq!(response[&RealTimeStatusResponse::InkNearEndDetected], false);
        assert_eq!(response[&RealTimeStatusResponse::InkEndDetected], true);
        assert_eq!(response[&RealTimeStatusResponse::InkCartridgeDetected], true);

        let response = RealTimeStatusResponse::parse(RealTimeStatusRequest::Peeler, 0b00010010).unwrap();
        assert_eq!(response[&RealTimeStatusResponse::WaitingForLabelToBeRemoved], false);
        assert_eq!(
            response[&RealTimeStatusResponse::PaperPresentInLabelPeelingDetector],
            true
        );

        let response = RealTimeStatusResponse::parse(RealTimeStatusRequest::Interface, 0b00010010).unwrap();
        assert_eq!(
            response[&RealTimeStatusResponse::PrintingMultipleInterfacesEnabled],
            false
        );

        let response = RealTimeStatusResponse::parse(RealTimeStatusRequest::DMD, 0b00010010).unwrap();
        assert_eq!(response[&RealTimeStatusResponse::DMDTransmissionStatusReady], true);
    }
}
