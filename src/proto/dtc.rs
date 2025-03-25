use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Default)]
/// Diagnostic troubleshooting code response
pub struct ReadDTCRsp {
    /// One of all diagnostic responses
    pub sub: DTCRspSubfunction,
}

#[derive(Clone, PartialEq, Debug, Default)]
/// Diagnostic troubleshooting code request
pub struct ReadDTCReq {
    /// One of all diagnostic requests
    pub sub: DTCReqSubfunction,
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
/// DTC code
pub struct Dtc {
    /// DTC value
    pub dtc: [u8; 3],
}

/// DTC status mask
pub type DTCStatusMask = u8;
/// DTC mask
pub type DTCMaskRecord = Dtc;
/// Snapshot record number
pub type DTCSnapshotRecordNumber = u8;
/// Stored data record number
pub type DTCStoredDataRecordNumber = u8;
/// Ext data record number
pub type DTCExtDataRecordNumber = u8;
/// Severity mask
pub type DTCSeverityMaskRecord = u16;
/// Memory selection
pub type MemorySelection = u8;
/// Functional group identifier
pub type FunctionalGroupIdentifier = u8;
/// DTC severity
pub type DTCSeverity = u8;
/// Function unit
pub type DTCFunctionalUnit = u8;

impl Dtc {
    /// Create a DTC code
    #[must_use]
    pub fn new(dtc_high_byte: u8, dtc_middle_byte: u8, dtc_low_byte: u8) -> Dtc {
        Dtc {
            dtc: [dtc_high_byte, dtc_middle_byte, dtc_low_byte],
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
#[repr(u8)]
#[allow(missing_docs)]
pub enum DTCReqSubfunction {
    #[default]
    Reserved1 = 0x00, // 0x00
    ReportNumberOfDTCByStatusMask(ReportNumberOfDTCByStatusMask) = 0x01, // 0x01
    ReportDTCByStatusMask(ReportDTCByStatusMask),                        // 0x02
    ReportDTCSnapshotIdentification(ReportDTCSnapshotIdentification),    // 0x03
    ReportDTCSnapshotRecordByDTCNumber(ReportDTCSnapshotRecordByDTCNumber), // 0x04
    ReportDTCStoredDataByRecordNumber(ReportDTCStoredDataByRecordNumber), // 0x05
    ReportDTCExtDataRecordByDTCNumber(ReportDTCExtDataRecordByDTCNumber), // 0x06
    ReportNumberOfDTCBySeverityMaskRecord(ReportNumberOfDTCBySeverityMaskRecord), // 0x07
    ReportDTCBySeverityMaskRecord(ReportDTCBySeverityMaskRecord),        // 0x08
    ReportSeverityInformationOfDTC(ReportSeverityInformationOfDTC),      // 0x09
    ReportSupportedDTC,                                                  // 0x0a
    ReportFirstTestFailedDTC,                                            // 0x0b
    ReportFirstConfirmedDTC,                                             // 0x0c
    ReportMostRecentTestFailedDTC,                                       // 0x0d
    ReportMostRecentConfirmedDTC,                                        // 0x0e
    ReportMirrorMemoryDTCByStatusMask(ReportMirrorMemoryDTCByStatusMask), // 0x0f
    ReportMirrorMemoryDTCExtDataRecordByDTCNumber(ReportMirrorMemoryDTCExtDataRecordByDTCNumber), // 0x10
    ReportNumberOfMirrorMemoryDTCByStatusMask(ReportNumberOfMirrorMemoryDTCByStatusMask), // 0x11
    ReportNumberOfEmissionsOBDDTCByStatusMask(ReportNumberOfEmissionsOBDDTCByStatusMask), // 0x12
    ReportEmissionsOBDDTCByStatusMask(ReportEmissionsOBDDTCByStatusMask),                 // 0x13
    ReportDTCFaultDetectionCounter,                                                       // 0x14
    ReportDTCWithPermanentStatus,                                                         // 0x15
    ReportDTCExtDataRecordByRecordNumber(ReportDTCExtDataRecordByRecordNumber),           // 0x16
    ReportUserDefMemoryDTCByStatusMask(ReportUserDefMemoryDTCByStatusMask),               // 0x17
    ReportUserDefMemoryDTCSnapshotRecordByDTCNumber(
        ReportUserDefMemoryDTCSnapshotRecordByDTCNumber,
    ), // 0x18
    ReportUserDefMemoryDTCExtDataRecordByDTCNumber(ReportUserDefMemoryDTCExtDataRecordByDTCNumber), // 0x19
    Reserved2,                                                              // 0x1a-0x41
    ReportWWHOBDDTCByMaskRecord(ReportWWHOBDDTCByMaskRecord),               // 0x42
    Reserved3,                                                              // 0x43-0x54,
    ReportWWHOBDDTCWithPermanentStatus(ReportWWHOBDDTCWithPermanentStatus), // 0x55
    Reserved4,                                                              // 0x56-0x7f
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Filter of status bitmask
pub struct ByDTCStatusMask {
    /// Mask to filter
    pub mask: DTCStatusMask,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Filter of DTC bitmask
pub struct ByDTCMaskRecord {
    /// Mask to filter
    pub mask: DTCMaskRecord,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Filter by DTC record bitmask and snapshot record number
pub struct ByDTCMaskRecordRecordNumber {
    /// Mask of the record
    pub mask: DTCMaskRecord,
    /// Mask of the snapshot record number
    pub record: DTCSnapshotRecordNumber,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Filter by DTC severity record
pub struct ByDTCSeverityMaskRecord {
    /// Mask of the severity
    pub mask: DTCSeverityMaskRecord,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Filter by functional group identifier
pub struct ByFunctionalGroupIdentifier {
    /// Functional Group Identifier
    pub group: FunctionalGroupIdentifier,
}

#[allow(missing_docs)]
pub type ReportNumberOfDTCByStatusMask = ByDTCStatusMask;
#[allow(missing_docs)]
pub type ReportDTCByStatusMask = ByDTCStatusMask;
#[allow(missing_docs)]
pub type ReportDTCSnapshotIdentification = ByDTCMaskRecordRecordNumber;
#[allow(missing_docs)]
pub type ReportDTCSnapshotRecordByDTCNumber = ByDTCMaskRecordRecordNumber;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Filter stored data by record number
pub struct ReportDTCStoredDataByRecordNumber {
    /// Record
    pub record: DTCStoredDataRecordNumber,
}

#[allow(missing_docs)]
pub type ReportDTCExtDataRecordByDTCNumber = ByDTCMaskRecord;
#[allow(missing_docs)]
pub type ReportNumberOfDTCBySeverityMaskRecord = ByDTCSeverityMaskRecord;
#[allow(missing_docs)]
pub type ReportDTCBySeverityMaskRecord = ByDTCSeverityMaskRecord;
#[allow(missing_docs)]
pub type ReportSeverityInformationOfDTC = ByDTCMaskRecord;
#[allow(missing_docs)]
pub type ReportMirrorMemoryDTCByStatusMask = ByDTCStatusMask;
#[allow(missing_docs)]
pub type ReportMirrorMemoryDTCExtDataRecordByDTCNumber = ByDTCMaskRecord;
#[allow(missing_docs)]
pub type ReportNumberOfMirrorMemoryDTCByStatusMask = ByDTCStatusMask;
#[allow(missing_docs)]
pub type ReportNumberOfEmissionsOBDDTCByStatusMask = ByDTCStatusMask;
#[allow(missing_docs)]
pub type ReportEmissionsOBDDTCByStatusMask = ByDTCStatusMask;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// `Filter` by `ExtDataRecordNumber`
pub struct ReportDTCExtDataRecordByRecordNumber {
    /// `ExtData` record number
    pub record: DTCExtDataRecordNumber,
}

#[allow(missing_docs)]
pub type ReportUserDefMemoryDTCByStatusMask = ByDTCStatusMask;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Filter by snapshot record and DTC number
pub struct ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
    /// DTC mask
    pub mask: DTCMaskRecord,
    /// Record
    pub record: DTCSnapshotRecordNumber,
    /// Memory selection
    pub memory: MemorySelection,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Filter by DTC, `ExtData` record number and memory selection
pub struct ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
    /// DTC mask
    pub mask: DTCMaskRecord,
    /// Record
    pub record: DTCExtDataRecordNumber,
    /// Memory selection
    pub memory: MemorySelection,
}

#[allow(missing_docs)]
pub type ReportWWHOBDDTCByMaskRecord = ByFunctionalGroupIdentifier;
#[allow(missing_docs)]
pub type ReportWWHOBDDTCWithPermanentStatus = ByFunctionalGroupIdentifier;

/*********** Responses **********/
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
#[repr(u8)]
#[allow(missing_docs)]
/// One DTC response
pub enum DTCRspSubfunction {
    #[default]
    Reserved1 = 0x00, // 0x00
    ResponseNumberOfDTCByStatusMask(ResponseNumberOfDTCByStatusMask) = 0x01, // 0x01
    ResponseDTCByStatusMask(ResponseDTCByStatusMask),                        // 0x02
    ResponseDTCSnapshotIdentification(ResponseDTCSnapshotIdentification),    // 0x03
    ResponseDTCSnapshotRecordByDTCNumber(ResponseDTCSnapshotRecordByDTCNumber), // 0x04
    ResponseDTCStoredDataByRecordNumber(ResponseDTCStoredDataByRecordNumber), // 0x05
    ResponseDTCExtDataRecordByDTCNumber(ResponseDTCExtDataRecordByDTCNumber), // 0x06
    ResponseNumberOfDTCBySeverityMaskRecord(ResponseNumberOfDTCBySeverityMaskRecord), // 0x07
    ResponseDTCBySeverityMaskRecord(ResponseDTCBySeverityMaskRecord),        // 0x08
    ResponseSeverityInformationOfDTC(ResponseDTCBySeverityMaskRecord),       // 0x09
    ResponseSupportedDTC(ResponseSupportedDTC),                              // 0x0a
    ResponseFirstTestFailedDTC(ResponseFirstTestFailedDTC),                  // 0x0b
    ResponseFirstConfirmedDTC(ResponseFirstConfirmedDTC),                    // 0x0c
    ResponseMostRecentTestFailedDTC(ResponseMostRecentTestFailedDTC),        // 0x0d
    ResponseMostRecentConfirmedDTC(ResponseMostRecentConfirmedDTC),          // 0x0e
    ResponseMirrorMemoryDTCByStatusMask(ResponseMirrorMemoryDTCByStatusMask), // 0x0f
    ResponseMirrorMemoryDTCExtDataRecordByDTCNumber(ResponseDTCExtDataRecordByDTCNumber), // 0x10
    ResponseNumberOfMirrorMemoryDTCByStatusMask(ResponseNumberOfMirrorMemoryDTCByStatusMask), // 0x11
    ResponseNumberOfEmissionsOBDDTCByStatusMask(ResponseNumberOfEmissionsOBDDTCByStatusMask), // 0x12
    ResponseEmissionsOBDDTCByStatusMask(ResponseEmissionsOBDDTCByStatusMask), // 0x13
    ResponseDTCFaultDetectionCounter(ResponseDTCFaultDetectionCounter),       // 0x14
    ResponseDTCWithPermanentStatus(ResponseDTCWithPermanentStatus),           // 0x15
    ResponseDTCExtDataRecordByRecordNumber(ResponseDTCExtDataRecordByRecordNumber), // 0x16
    ResponseUserDefMemoryDTCByStatusMask(ResponseUserDefMemoryDTCByStatusMask), // 0x17
    ResponseUserDefMemoryDTCSnapshotRecordByDTCNumber(
        ResponseUserDefMemoryDTCSnapshotRecordByDTCNumber,
    ), // 0x18
    ResponseUserDefMemoryDTCExtDataRecordByDTCNumber(
        ResponseUserDefMemoryDTCExtDataRecordByDTCNumber,
    ), // 0x19
    Reserved2,                                                                // 0x1a-0x41
    ResponseWWHOBDDTCByMaskRecord(Vec<u8>),                                   // 0x42
    Reserved3,                                                                // 0x43-0x54,
    ResponseWWHOBDDTCWithPermanentStatus(Vec<u8>),                            // 0x55
    Reserved4,                                                                // 0x56-0x7f
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Result of DTC count
pub struct GotDTCCount {
    /// DTC mask
    pub mask: DTCStatusMask,
    /// DTC format
    pub format: u8,
    ///  Nuber of DTC found
    pub count: u16,
}
#[allow(missing_docs)]
pub type ResponseNumberOfDTCByStatusMask = GotDTCCount;
#[allow(missing_docs)]
pub type ResponseNumberOfDTCBySeverityMaskRecord = GotDTCCount;
#[allow(missing_docs)]
pub type ResponseNumberOfMirrorMemoryDTCByStatusMask = GotDTCCount;
#[allow(missing_docs)]
pub type ResponseNumberOfEmissionsOBDDTCByStatusMask = GotDTCCount;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Result of DTC and status
pub struct DtcAndStatusRecord {
    /// DTC
    pub dtc: Dtc,
    /// Status
    pub status: DTCStatusMask,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Result of DTC and status records
pub struct GotListDtcAndStatusRecord {
    /// Availability mask
    pub availability_mask: DTCStatusMask,
    /// List of Dtc and status records
    pub dtcs: Vec<DtcAndStatusRecord>,
}

#[allow(missing_docs)]
pub type ResponseDTCByStatusMask = GotListDtcAndStatusRecord;
#[allow(missing_docs)]
pub type ResponseSupportedDTC = GotListDtcAndStatusRecord;
#[allow(missing_docs)]
pub type ResponseFirstTestFailedDTC = GotListDtcAndStatusRecord;
#[allow(missing_docs)]
pub type ResponseFirstConfirmedDTC = GotListDtcAndStatusRecord;
#[allow(missing_docs)]
pub type ResponseMostRecentTestFailedDTC = GotListDtcAndStatusRecord;
#[allow(missing_docs)]
pub type ResponseMostRecentConfirmedDTC = GotListDtcAndStatusRecord;
#[allow(missing_docs)]
pub type ResponseMirrorMemoryDTCByStatusMask = GotListDtcAndStatusRecord;
#[allow(missing_docs)]
pub type ResponseEmissionsOBDDTCByStatusMask = GotListDtcAndStatusRecord;
#[allow(missing_docs)]
pub type ResponseDTCWithPermanentStatus = GotListDtcAndStatusRecord;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Result of DTC and Snapshot record numbers
pub struct GotDtcRecordAndSnapshotNumber {
    /// DTC and Snapshot record numbers
    pub records: Vec<(Dtc, DTCSnapshotRecordNumber)>,
}
#[allow(missing_docs)]
pub type ResponseDTCSnapshotIdentification = GotDtcRecordAndSnapshotNumber;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Response of DTC, status and snapshot records
pub struct ResponseDTCSnapshotRecordByDTCNumber {
    /// DTC and its status
    pub dtc_status: DtcAndStatusRecord,
    /// Snapshot records
    pub dtc_snapshot_records: DtcSnapshotRecords,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// DTC snapshot records
pub struct DtcSnapshotRecords {
    /// Snapshot record number
    pub dtc_snapshot_record_number: u8,
    /// Number DIDs in the snapshot
    pub dtc_snapshot_record_number_of_identifiers: u8,
    /// DIDs in the snapshot
    pub dtc_snapshot_record: DTCSnapshotRecord,
}

/// DIDs in a snapshot
pub type DTCSnapshotRecord = Vec<DidSnapshot>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Single DID in a snapshot
pub struct DidSnapshot {
    /// Diagnostic identifier
    pub did: u16,
    /// Value of the Diagnostic Identifier
    pub value: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Response of stored data records
pub struct ResponseDTCStoredDataByRecordNumber {
    /// Records
    pub records: Vec<DtcStoredDataRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Response of a stored data record
pub struct DtcStoredDataRecord {
    /// Record number
    pub record_number: DTCStoredDataRecordNumber,
    /// DTC and DTC status
    pub dtc_status: DtcAndStatusRecord,
    /// Number of Diagnostic Identifiers
    pub number_identifiers: u8,
    /// DIDs and their values
    pub data_records: DTCSnapshotRecord,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Response of DTC, its status and a list of `ExtData` records
pub struct ResponseDTCExtDataRecordByDTCNumber {
    /// DTC and its status
    pub dtc_status: DtcAndStatusRecord,
    /// `ExtData` records
    pub records: Vec<ExtDataRecordAndNumber>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// `ExtData` record
pub struct ExtDataRecordAndNumber {
    /// Record number
    pub record_number: DTCExtDataRecordNumber,
    /// `ExtData` value
    pub extended_data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Response of DTC and its severity records
pub struct ResponseDTCBySeverityMaskRecord {
    /// Availability mask
    pub availability_mask: DTCStatusMask,
    /// DTC and severity record
    pub records: Vec<DTCAndSeverityRecord>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// DTC and severity record
pub struct DTCAndSeverityRecord {
    /// DTC severity
    pub severity: DTCSeverity,
    /// Functional unit
    pub functional_unit: DTCFunctionalUnit,
    /// DTC and its status
    pub dtc_status: DtcAndStatusRecord,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Response of DTC and fault counter
pub struct ResponseDTCFaultDetectionCounter {
    /// DTC and fault counter
    pub faults: Vec<DtcAndFaultCounter>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// DTC and fault counter
pub struct DtcAndFaultCounter {
    /// DTC
    pub dtc: Dtc,
    /// Counter
    pub counter: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Response DTC and its `ExtData` records
pub struct ResponseDTCExtDataRecordByRecordNumber {
    /// DTC and status record
    pub dtc_status: DtcAndStatusRecord,
    /// `ExtData` records
    pub records: Vec<DtcAndStatusAndExtDataRecord>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// DTC status with its `ExtData`
pub struct DtcAndStatusAndExtDataRecord {
    /// DTC and its status
    pub dtc_status: DtcAndStatusRecord,
    /// `ExtData`
    pub extended_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Response of Memory selection by status mask
pub struct ResponseUserDefMemoryDTCByStatusMask {
    /// Memory selection
    pub memory: MemorySelection,
    /// DTC status mask
    pub availability_mask: DTCStatusMask,
    /// List of records
    pub records: Vec<DtcAndStatusRecord>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Response of Memory selection with its DID snapshot records
pub struct ResponseUserDefMemoryDTCSnapshotRecordByDTCNumber {
    /// Memory selection
    pub memory: MemorySelection,
    /// DTC and its status
    pub dtc_status: DtcAndStatusRecord,
    /// List of records containing a DID snapshot
    pub records: Vec<RecordNumberAndIdentifierAndSnapshot>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Record containing a DID snapshot
pub struct RecordNumberAndIdentifierAndSnapshot {
    /// Record number
    pub record_number: DTCStoredDataRecordNumber,
    /// Number of identifiers
    pub number_identifiers: u8,
    /// DID with its value
    pub record: DidSnapshot,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Response of Memory selection with its `ExtData` records
pub struct ResponseUserDefMemoryDTCExtDataRecordByDTCNumber {
    /// Memory selection
    pub memory: MemorySelection,
    /// DTC and its status
    pub dtc_status: DtcAndStatusRecord,
    /// `ExtData` records
    pub records: Vec<ExtDataRecordAndNumber>,
}

#[cfg(test)]
mod tests {
    use super::Dtc;
    use super::*;

    #[test]
    fn test_bincode_ser() {
        let rn = ReportNumberOfDTCByStatusMask { mask: 0x27 };
        let serialized = bincode::serialize(&rn).unwrap();
        assert_eq!(&serialized, &[0x27]);
    }

    #[test]
    fn test_bincode_subfunction() {
        let sub = DTCReqSubfunction::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber(
            ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
                mask: Dtc::new(0xae, 0x38, 0x47),
                record: 0x07,
                memory: 0x01,
            },
        );
        use bincode::config::Options;
        let serialized = bincode::DefaultOptions::new()
            .with_varint_encoding()
            .serialize(&sub)
            .unwrap();
        assert_eq!(&serialized, &[0x18, 0xae, 0x38, 0x47, 0x07, 0x01]);
    }

    #[test]
    fn test_decode_19_04() {
        let uds = [0x19, 0x04, 0xae, 0x38, 0x47, 0xff];
        let deserialized: ReportDTCSnapshotRecordByDTCNumber =
            bincode::deserialize(&uds[2..]).unwrap();
        assert_eq!(deserialized.mask, Dtc::new(0xae, 0x38, 0x47));
        assert_eq!(deserialized.record, 0xff);
    }

    #[test]
    fn test_decode_19_04_too_big() {
        let uds = [0x19, 0x04, 0xae, 0x38, 0x47, 0xff, 0xaa];
        let deserialized: ReportDTCSnapshotRecordByDTCNumber =
            bincode::deserialize(&uds[2..]).unwrap();
        assert_ne!(
            bincode::serialized_size(&deserialized).unwrap() as usize,
            uds[2..].len()
        );
        assert_eq!(deserialized.mask, Dtc::new(0xae, 0x38, 0x47));
        assert_eq!(deserialized.record, 0xff);
    }
}
