use std::fmt::Display;

use crate::proto::dtc::*;

impl Display for DTCReqSubfunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DTCReqSubfunction::*;
        match self {
            ReportNumberOfDTCByStatusMask(r) => {
                write!(f, "ReportNumberOfDTCByStatusMask({r})")
            }
            ReportDTCByStatusMask(r) => {
                write!(f, "ReportDTCByStatusMask({r})")
            }
            ReportDTCSnapshotIdentification(r) => {
                write!(f, "ReportDTCSnapshotIdentification({r})")
            }
            ReportDTCSnapshotRecordByDTCNumber(r) => {
                write!(f, "ReportDTCSnapshotRecordByDTCNumber({r})")
            }
            ReportDTCStoredDataByRecordNumber(r) => {
                write!(f, "ReportDTCStoredDataByRecordNumber({r})")
            }
            ReportDTCExtDataRecordByDTCNumber(r) => {
                write!(f, "ReportDTCExtDataRecordByDTCNumber({r})")
            }
            ReportNumberOfDTCBySeverityMaskRecord(r) => {
                write!(f, "ReportNumberOfDTCBySeverityMaskRecord({r})")
            }
            ReportDTCBySeverityMaskRecord(r) => {
                write!(f, "ReportDTCBySeverityMaskRecord({r})")
            }
            ReportSeverityInformationOfDTC(r) => {
                write!(f, "ReportSeverityInformationOfDTC({r})")
            }
            ReportSupportedDTC => write!(f, "ReportSupportedDTC"),
            ReportFirstTestFailedDTC => write!(f, "ReportFirstTestFailedDTC"),
            ReportFirstConfirmedDTC => write!(f, "ReportFirstConfirmedDTC"),
            ReportMostRecentTestFailedDTC => {
                write!(f, "ReportMostRecentTestFailedDTC")
            }
            ReportMostRecentConfirmedDTC => {
                write!(f, "ReportMostRecentConfirmedDTC")
            }
            ReportMirrorMemoryDTCByStatusMask(r) => {
                write!(f, "ReportMirrorMemoryDTCByStatusMask({r})")
            }
            ReportMirrorMemoryDTCExtDataRecordByDTCNumber(r) => {
                write!(f, "ReportMirrorMemoryDTCExtDataRecordByDTCNumber({r})")
            }
            ReportNumberOfMirrorMemoryDTCByStatusMask(r) => {
                write!(f, "ReportNumberOfMirrorMemoryDTCByStatusMask({r})")
            }
            ReportNumberOfEmissionsOBDDTCByStatusMask(r) => {
                write!(f, "ReportNumberOfEmissionsOBDDTCByStatusMask({r})")
            }
            ReportEmissionsOBDDTCByStatusMask(r) => {
                write!(f, "ReportEmissionsOBDDTCByStatusMask({r})")
            }
            ReportDTCFaultDetectionCounter => {
                write!(f, "ReportDTCFaultDetectionCounter")
            }
            ReportDTCWithPermanentStatus => {
                write!(f, "ReportDTCWithPermanentStatus")
            }
            ReportDTCExtDataRecordByRecordNumber(r) => {
                write!(f, "ReportDTCExtDataRecordByRecordNumber({r})")
            }
            ReportUserDefMemoryDTCByStatusMask(r) => {
                write!(f, "ReportUserDefMemoryDTCByStatusMask({r})")
            }
            ReportUserDefMemoryDTCSnapshotRecordByDTCNumber(r) => {
                write!(f, "{r}")
            }
            ReportUserDefMemoryDTCExtDataRecordByDTCNumber(r) => {
                write!(f, "ReportUserDefMemoryDTCExtDataRecordByDTCNumber({r})")
            }
            ReportWWHOBDDTCByMaskRecord(r) => {
                write!(f, "ReportWWHOBDDTCByMaskRecord({r})")
            }
            ReportWWHOBDDTCWithPermanentStatus(r) => {
                write!(f, "ReportWWHOBDDTCWithPermanentStatus({r})")
            }
            _ => Ok(()),
        }
    }
}

impl Display for Dtc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "0x{:02x}{:02x}{:02x}",
            self.dtc[0], self.dtc[1], self.dtc[2]
        )
    }
}

impl Display for ByDTCStatusMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mask={:02x}", self.mask)
    }
}

impl Display for ByDTCMaskRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mask={}", self.mask)
    }
}

impl Display for ByDTCMaskRecordRecordNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mask={}, record=0x{:02x}", self.mask, self.record)
    }
}

impl Display for ByDTCSeverityMaskRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mask=0x{:02x}", self.mask)
    }
}

impl Display for ByFunctionalGroupIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mask=0x{:02x}", self.group)
    }
}

impl Display for ReportDTCStoredDataByRecordNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "record=0x{:02x}", self.record)
    }
}

impl Display for ReportDTCExtDataRecordByRecordNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "record=0x{:02x}", self.record)
    }
}

impl Display for ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mask={}, record=0x{:02x}, memory=0x{:02x}",
            self.mask, self.record, self.memory
        )
    }
}

impl Display for ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mask={}, record=0x{:02x}, memory=0x{:02x}",
            self.mask, self.record, self.memory
        )
    }
}

impl Display for DTCRspSubfunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use DTCRspSubfunction::*;
        match self {
            ResponseNumberOfDTCByStatusMask(r) => {
                write!(f, "ResponseNumberOfDTCByStatusMask({r:?})")
            }
            ResponseDTCByStatusMask(r) => {
                write!(f, "ResponseDTCByStatusMask: {r}")
            }
            ResponseDTCSnapshotIdentification(r) => {
                write!(f, "ResponseDTCSnapshotIdentification({r:?})")
            }
            ResponseDTCSnapshotRecordByDTCNumber(r) => {
                write!(f, "ResponseDTCSnapshotRecordByDTCNumber({r:?})")
            }
            ResponseDTCStoredDataByRecordNumber(r) => {
                write!(f, "ResponseDTCStoredDataByRecordNumber({r:?})")
            }
            ResponseDTCExtDataRecordByDTCNumber(r) => {
                write!(f, "ResponseDTCExtDataRecordByDTCNumber({r:?})")
            }
            ResponseNumberOfDTCBySeverityMaskRecord(r) => {
                write!(f, "ResponseNumberOfDTCBySeverityMaskRecord({r:?})")
            }
            ResponseDTCBySeverityMaskRecord(r) => {
                write!(f, "ResponseDTCBySeverityMaskRecord({r:?})")
            }
            ResponseSeverityInformationOfDTC(r) => {
                write!(f, "ResponseSeverityInformationOfDTC({r:?})")
            }
            ResponseSupportedDTC(r) => {
                write!(f, "ResponseSupportedDTC: {r}")
            }
            ResponseFirstTestFailedDTC(r) => {
                write!(f, "ResponseFirstTestFailedDTC: {r}")
            }
            ResponseFirstConfirmedDTC(r) => {
                write!(f, "ResponseFirstConfirmedDTC: {r}")
            }
            ResponseMostRecentTestFailedDTC(r) => {
                write!(f, "ResponseMostRecentTestFailedDTC: {r}")
            }
            ResponseMostRecentConfirmedDTC(r) => {
                write!(f, "ResponseMostRecentConfirmedDTC: {r}")
            }
            ResponseMirrorMemoryDTCByStatusMask(r) => {
                write!(f, "ResponseMirrorMemoryDTCByStatusMask: {r}")
            }
            ResponseMirrorMemoryDTCExtDataRecordByDTCNumber(r) => {
                write!(f, "ResponseMirrorMemoryDTCExtDataRecordByDTCNumber({r:?})")
            }
            ResponseNumberOfMirrorMemoryDTCByStatusMask(r) => {
                write!(f, "ResponseNumberOfMirrorMemoryDTCByStatusMask({r:?})")
            }
            ResponseNumberOfEmissionsOBDDTCByStatusMask(r) => {
                write!(f, "ResponseNumberOfEmissionsOBDDTCByStatusMask({r:?})")
            }
            ResponseEmissionsOBDDTCByStatusMask(r) => {
                write!(f, "ResponseEmissionsOBDDTCByStatusMask: {r}")
            }
            ResponseDTCFaultDetectionCounter(r) => {
                write!(f, "ResponseDTCFaultDetectionCounter({r:?})")
            }
            ResponseDTCWithPermanentStatus(r) => {
                write!(f, "ResponseDTCWithPermanentStatus: {r}")
            }
            ResponseDTCExtDataRecordByRecordNumber(r) => {
                write!(f, "ResponseDTCExtDataRecordByRecordNumber({r:?})")
            }
            ResponseUserDefMemoryDTCByStatusMask(r) => {
                write!(f, "ResponseUserDefMemoryDTCByStatusMask({r:?})")
            }
            ResponseUserDefMemoryDTCSnapshotRecordByDTCNumber(r) => {
                write!(
                    f,
                    "ResponseUserDefMemoryDTCSnapshotRecordByDTCNumber({r:?})"
                )
            }
            ResponseUserDefMemoryDTCExtDataRecordByDTCNumber(r) => {
                write!(f, "ResponseUserDefMemoryDTCExtDataRecordByDTCNumber({r:?})")
            }
            ResponseWWHOBDDTCByMaskRecord(r) => {
                write!(f, "ResponseWWHOBDDTCByMaskRecord({r:?})")
            }
            ResponseWWHOBDDTCWithPermanentStatus(r) => {
                write!(f, "ResponseWWHOBDDTCWithPermanentStatus({r:?})")
            }
            _ => todo!(),
        }
    }
}

impl Display for DtcAndStatusRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dtc {} status=0x{:02x}", self.dtc, self.status)
    }
}

impl Display for GotListDtcAndStatusRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "avail_mask=0x{:02x} :", self.availability_mask)?;
        let mut idx = 1;
        let total = self.dtcs.len();
        for dtc_and_status in &self.dtcs {
            if idx == total {
                write!(f, "\t{idx:>3}. {dtc_and_status}")?;
            } else {
                writeln!(f, "\t{idx:>3}. {dtc_and_status}")?;
            }
            idx += 1;
        }
        Ok(())
    }
}
