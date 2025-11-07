#[allow(dead_code)]
mod common;

mod request {
    use super::common::test_encode_decode;
    use uds_rw::{message::*, UdsMessage};

    #[test]
    fn dtc_report_number_of_dtc_by_status_mask() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportNumberOfDTCByStatusMask(ReportNumberOfDTCByStatusMask {
                mask: 0xa0,
            }),
        });
        let exp = vec![0x19, 0x01, 0xa0];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_dtc_by_status_mask() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportDTCByStatusMask(ReportDTCByStatusMask { mask: 0xa0 }),
        });
        let exp = vec![0x19, 0x02, 0xa0];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_dtc_snapshot_identification() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportDTCSnapshotIdentification(
                ReportDTCSnapshotIdentification {
                    mask: [0x10, 0x00, 0x02].into(),
                    record: 0x12,
                },
            ),
        });
        let exp = vec![0x19, 0x03, 0x10, 0x00, 0x02, 0x12];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_dtc_snapshot_record_by_dtc_number() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportDTCSnapshotRecordByDTCNumber(
                ReportDTCSnapshotRecordByDTCNumber {
                    mask: [0x10, 0x00, 0x02].into(),
                    record: 0x12,
                },
            ),
        });
        let exp = vec![0x19, 0x04, 0x10, 0x00, 0x02, 0x12];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_dtc_store_data_by_record_number() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportDTCStoredDataByRecordNumber(
                ReportDTCStoredDataByRecordNumber { record: 0x08 },
            ),
        });
        let exp = vec![0x19, 0x05, 0x08];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_dtc_ext_data_record_by_dtc_number() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportDTCExtDataRecordByDTCNumber(
                ReportDTCExtDataRecordByDTCNumber {
                    mask: [0xff, 0xff, 0xfe].into(),
                },
            ),
        });
        let exp = vec![0x19, 0x06, 0xff, 0xff, 0xfe];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_number_of_dtc_by_severity_mask_record() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportNumberOfDTCBySeverityMaskRecord(
                ReportNumberOfDTCBySeverityMaskRecord { mask: 0xff80 },
            ),
        });
        let exp = vec![0x19, 0x07, 0xff, 0x80];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_dtc_by_severity_mask_record() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportDTCBySeverityMaskRecord(ReportDTCBySeverityMaskRecord {
                mask: 0x1234,
            }),
        });
        let exp = vec![0x19, 0x08, 0x12, 0x34];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_serverity_information_of_dtc() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportSeverityInformationOfDTC(
                ReportSeverityInformationOfDTC {
                    mask: [0xff, 0xff, 0xf0].into(),
                },
            ),
        });
        let exp = vec![0x19, 0x09, 0xff, 0xff, 0xf0];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_supported_dtc() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportSupportedDTC,
        });
        let exp = vec![0x19, 0x0a];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_first_test_failed_dtc() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportFirstTestFailedDTC,
        });
        let exp = vec![0x19, 0x0b];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_first_confirmed_dtc() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportFirstConfirmedDTC,
        });
        let exp = vec![0x19, 0x0c];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_most_recent_test_failed_dtc() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportMostRecentTestFailedDTC,
        });
        let exp = vec![0x19, 0x0d];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_most_recent_confirmed_dtc() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportMostRecentConfirmedDTC,
        });
        let exp = vec![0x19, 0x0e];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_mirror_memory_dtc_by_status_mask_dtc() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportMirrorMemoryDTCByStatusMask(
                ReportMirrorMemoryDTCByStatusMask { mask: 0x4 },
            ),
        });
        let exp = vec![0x19, 0x0f, 0x04];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_mirror_memory_dtc_ext_data_record_by_dtc_number() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportMirrorMemoryDTCExtDataRecordByDTCNumber(
                ReportMirrorMemoryDTCExtDataRecordByDTCNumber {
                    mask: [0x00, 0xff, 0xa3].into(),
                },
            ),
        });
        let exp = vec![0x19, 0x10, 0x00, 0xff, 0xa3];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_number_of_mirror_memory_dtc_by_status_mask() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportNumberOfMirrorMemoryDTCByStatusMask(
                ReportNumberOfMirrorMemoryDTCByStatusMask { mask: 0xf0 },
            ),
        });
        let exp = vec![0x19, 0x11, 0xf0];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_number_of_emissions_obddtc_by_status_mask() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportNumberOfEmissionsOBDDTCByStatusMask(
                ReportNumberOfEmissionsOBDDTCByStatusMask { mask: 0xf0 },
            ),
        });
        let exp = vec![0x19, 0x12, 0xf0];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_emissions_obddtc_by_status_mask() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportEmissionsOBDDTCByStatusMask(
                ReportEmissionsOBDDTCByStatusMask { mask: 0xf0 },
            ),
        });
        let exp = vec![0x19, 0x13, 0xf0];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_dtc_fault_detection_counter() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportDTCFaultDetectionCounter,
        });
        let exp = vec![0x19, 0x14];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_dtc_with_permanent_status() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportDTCWithPermanentStatus,
        });
        let exp = vec![0x19, 0x15];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_dtc_ext_data_record_by_record_number() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportDTCExtDataRecordByRecordNumber(
                ReportDTCExtDataRecordByRecordNumber { record: 3 },
            ),
        });
        let exp = vec![0x19, 0x16, 0x03];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_dtc_user_def_memory_dtc_by_status_mask() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportUserDefMemoryDTCByStatusMask(
                ReportUserDefMemoryDTCByStatusMask { mask: 0xf0 },
            ),
        });
        let exp = vec![0x19, 0x17, 0xf0];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_user_def_memory_dtc_snapshot_record_by_dtc_number() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportUserDefMemoryDTCSnapshotRecordByDTCNumber(
                ReportUserDefMemoryDTCSnapshotRecordByDTCNumber {
                    mask: [0xf0, 0x00, 0x00].into(),
                    record: 3,
                    memory: 7,
                },
            ),
        });
        let exp = vec![0x19, 0x18, 0xf0, 0x00, 0x00, 0x03, 0x07];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_user_def_memory_dtc_ext_data_record_by_dtc_number() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportUserDefMemoryDTCExtDataRecordByDTCNumber(
                ReportUserDefMemoryDTCExtDataRecordByDTCNumber {
                    mask: [0xf0, 0x00, 0x00].into(),
                    record: 3,
                    memory: 7,
                },
            ),
        });
        let exp = vec![0x19, 0x19, 0xf0, 0x00, 0x00, 0x03, 0x07];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_wwobddtc_by_mask_record() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportWWHOBDDTCByMaskRecord(ReportWWHOBDDTCByMaskRecord {
                group: 0x17,
            }),
        });
        let exp = vec![0x19, 0x42, 0x17];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_report_wwobddtc_with_permanent_status() {
        let req = UdsMessage::ReadDTCReq(ReadDTCReq {
            sub: DTCReqSubfunction::ReportWWHOBDDTCWithPermanentStatus(
                ReportWWHOBDDTCWithPermanentStatus { group: 0x9 },
            ),
        });
        let exp = vec![0x19, 0x55, 0x9];
        test_encode_decode(&req, &exp);
    }
}

mod response {
    use super::common::test_encode_decode;
    use uds_rw::{message::*, UdsMessage};

    #[test]
    fn dtc_response_number_of_dtc_by_status_mask() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseNumberOfDTCByStatusMask(
                ResponseNumberOfDTCByStatusMask {
                    mask: 0xff,
                    format: 0xaa,
                    count: 2,
                },
            ),
        });
        let exp = vec![0x59, 0x01, 0xff, 0xaa, 0x00, 0x02];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_dtc_by_status_mask() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseDTCByStatusMask(ResponseDTCByStatusMask {
                availability_mask: 0xf0,
                dtcs: vec![
                    DtcAndStatusRecord {
                        dtc: 0x001011.into(),
                        status: 0x03,
                    },
                    DtcAndStatusRecord {
                        dtc: 0x001012.into(),
                        status: 0x04,
                    },
                ],
            }),
        });
        let exp = vec![
            0x59, 0x02, 0xf0, 0x00, 0x10, 0x11, 0x03, 0x00, 0x10, 0x12, 0x04,
        ];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_dtc_snapshot_identification() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseDTCSnapshotIdentification(
                ResponseDTCSnapshotIdentification {
                    records: vec![(0x001011.into(), 0x03), (0x001012.into(), 0x04)],
                },
            ),
        });
        let exp = vec![0x59, 0x03, 0x00, 0x10, 0x11, 0x03, 0x00, 0x10, 0x12, 0x04];
        test_encode_decode(&req, &exp);
    }

    #[ignore] // Impossible to deserialize `vec` of unkonwn beforehand size
    #[test]
    fn dtc_response_dtc_snapshot_record_by_dtc_number() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseDTCSnapshotRecordByDTCNumber(
                ResponseDTCSnapshotRecordByDTCNumber {
                    dtc_status: DtcAndStatusRecord {
                        dtc: 0x001012.into(),
                        status: 0x07,
                    },
                    dtc_snapshot_records: DtcSnapshotRecords {
                        dtc_snapshot_record_number: 2,
                        dtc_snapshot_record_number_of_identifiers: 1,
                        dtc_snapshot_record: vec![
                            DidSnapshot {
                                did: 0xf180,
                                value: vec![0x01, 0x02],
                            },
                            DidSnapshot {
                                did: 0xf181,
                                value: vec![0x03, 0x03],
                            },
                        ],
                    },
                },
            ),
        });
        let exp = vec![
            0x59, 0x04, 0x00, 0x10, 0x12, 0x07, 2, 1, 0xf1, 0x80, 0x01, 0x02, 0xf1, 0x81, 0x03,
            0x03,
        ];
        test_encode_decode(&req, &exp);
    }

    #[ignore] // Impossible to deserialize `vec` of unkonwn beforehand size
    #[test]
    fn dtc_response_dtc_store_data_by_record_number() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseDTCStoredDataByRecordNumber(
                ResponseDTCStoredDataByRecordNumber {
                    records: vec![
                        DtcStoredDataRecord {
                            record_number: 4,
                            dtc_status: DtcAndStatusRecord {
                                dtc: 0xfff000.into(),
                                status: 0xf0,
                            },
                            number_identifiers: 2,
                            data_records: vec![
                                DidSnapshot {
                                    did: 0xf180,
                                    value: vec![0x11, 0x32],
                                },
                                DidSnapshot {
                                    did: 0xf181,
                                    value: vec![0x12, 0x34],
                                },
                            ],
                        },
                        DtcStoredDataRecord {
                            record_number: 5,
                            dtc_status: DtcAndStatusRecord {
                                dtc: 0xfff000.into(),
                                status: 0xf0,
                            },
                            number_identifiers: 1,
                            data_records: vec![DidSnapshot {
                                did: 0xf180,
                                value: vec![0x11, 0x32],
                            }],
                        },
                    ],
                },
            ),
        });
        let exp = vec![
            0x59, 0x05, // Header
            4, 0xff, 0xf0, 0x00, 0xf0, 2, 0xf1, 0x80, 0x11, 0x32, 0xf1, 0x81, 0x12,
            0x34, // First Record
            5, 0xff, 0xf0, 0x00, 0xf0, 1, 0xf1, 0x80, 0x11, 0x32,
        ];
        test_encode_decode(&req, &exp);
    }

    #[ignore] // Impossible to deserialize `vec` of unkonwn beforehand size
    #[test]
    fn dtc_response_dtc_ext_data_record_by_dtc_number() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseDTCExtDataRecordByDTCNumber(
                ResponseDTCExtDataRecordByDTCNumber {
                    dtc_status: DtcAndStatusRecord {
                        dtc: 0x00f180.into(),
                        status: 0xa0,
                    },
                    records: vec![
                        ExtDataRecordAndNumber {
                            record_number: 1,
                            extended_data: vec![0xab, 0xcd],
                        },
                        ExtDataRecordAndNumber {
                            record_number: 3,
                            extended_data: vec![0xba, 0xdc],
                        },
                    ],
                },
            ),
        });
        let exp = vec![
            0x59, 0x06, 0x00, 0xf1, 0x80, 0xa0, // Header
            1, 0xab, 0xcd, 3, 0xba, 0xdc,
        ];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_number_of_dtc_by_severity_mask_record() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseNumberOfDTCBySeverityMaskRecord(
                ResponseNumberOfDTCBySeverityMaskRecord {
                    mask: 0xa0,
                    format: 0xaa,
                    count: 3,
                },
            ),
        });
        let exp = vec![0x59, 0x07, 0xa0, 0xaa, 0x00, 0x03];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_dtc_by_severity_mask_record() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseDTCBySeverityMaskRecord(
                ResponseDTCBySeverityMaskRecord {
                    availability_mask: 0xf0,
                    records: vec![DTCAndSeverityRecord {
                        severity: 0x33,
                        functional_unit: 1,
                        dtc_status: DtcAndStatusRecord {
                            dtc: 0x00f180.into(),
                            status: 0xa0,
                        },
                    }],
                },
            ),
        });
        let exp = vec![0x59, 0x08, 0xf0, 0x33, 1, 0x00, 0xf1, 0x80, 0xa0];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_serverity_information_of_dtc() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseSeverityInformationOfDTC(
                ResponseDTCBySeverityMaskRecord {
                    availability_mask: 0xf0,
                    records: vec![DTCAndSeverityRecord {
                        severity: 0x33,
                        functional_unit: 1,
                        dtc_status: DtcAndStatusRecord {
                            dtc: 0x00f180.into(),
                            status: 0xa0,
                        },
                    }],
                },
            ),
        });
        let exp = vec![0x59, 0x09, 0xf0, 0x33, 1, 0x00, 0xf1, 0x80, 0xa0];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_supported_dtc() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseSupportedDTC(ResponseSupportedDTC {
                availability_mask: 0xa0,
                dtcs: vec![
                    DtcAndStatusRecord {
                        dtc: 0x00f180.into(),
                        status: 0xa0,
                    },
                    DtcAndStatusRecord {
                        dtc: 0x00f181.into(),
                        status: 0x80,
                    },
                ],
            }),
        });
        let exp = vec![
            0x59, 0x0a, 0xa0, 0x00, 0xf1, 0x80, 0xa0, 0x00, 0xf1, 0x81, 0x80,
        ];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_first_test_failed_dtc() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseFirstTestFailedDTC(ResponseFirstTestFailedDTC {
                availability_mask: 0x0a,
                dtcs: vec![
                    DtcAndStatusRecord {
                        dtc: 0x00f180.into(),
                        status: 0xa0,
                    },
                    DtcAndStatusRecord {
                        dtc: 0x00f181.into(),
                        status: 0x80,
                    },
                ],
            }),
        });
        let exp = vec![
            0x59, 0x0b, 0x0a, 0x00, 0xf1, 0x80, 0xa0, 0x00, 0xf1, 0x81, 0x80,
        ];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_first_confirmed_dtc() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseFirstConfirmedDTC(ResponseFirstConfirmedDTC {
                availability_mask: 0x0a,
                dtcs: vec![
                    DtcAndStatusRecord {
                        dtc: 0x00f180.into(),
                        status: 0xa0,
                    },
                    DtcAndStatusRecord {
                        dtc: 0x00f181.into(),
                        status: 0x80,
                    },
                ],
            }),
        });
        let exp = vec![
            0x59, 0x0c, 0x0a, 0x00, 0xf1, 0x80, 0xa0, 0x00, 0xf1, 0x81, 0x80,
        ];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_most_recent_test_failed_dtc() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseMostRecentTestFailedDTC(
                ResponseMostRecentTestFailedDTC {
                    availability_mask: 0x0a,
                    dtcs: vec![
                        DtcAndStatusRecord {
                            dtc: 0x00f180.into(),
                            status: 0xa0,
                        },
                        DtcAndStatusRecord {
                            dtc: 0x00f181.into(),
                            status: 0x80,
                        },
                    ],
                },
            ),
        });
        let exp = vec![
            0x59, 0x0d, 0x0a, 0x00, 0xf1, 0x80, 0xa0, 0x00, 0xf1, 0x81, 0x80,
        ];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_most_recent_confirmed_dtc() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseMostRecentConfirmedDTC(
                ResponseMostRecentConfirmedDTC {
                    availability_mask: 0x0a,
                    dtcs: vec![
                        DtcAndStatusRecord {
                            dtc: 0x00f180.into(),
                            status: 0xa0,
                        },
                        DtcAndStatusRecord {
                            dtc: 0x00f181.into(),
                            status: 0x80,
                        },
                    ],
                },
            ),
        });
        let exp = vec![
            0x59, 0x0e, 0x0a, 0x00, 0xf1, 0x80, 0xa0, 0x00, 0xf1, 0x81, 0x80,
        ];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_mirror_memory_dtc_by_status_mask_dtc() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseMirrorMemoryDTCByStatusMask(
                ResponseMirrorMemoryDTCByStatusMask {
                    availability_mask: 0x07,
                    dtcs: vec![
                        DtcAndStatusRecord {
                            dtc: 0x00f180.into(),
                            status: 0x03,
                        },
                        DtcAndStatusRecord {
                            dtc: 0x00f181.into(),
                            status: 0x04,
                        },
                    ],
                },
            ),
        });
        let exp = vec![
            0x59, 0x0f, 0x07, 0x00, 0xf1, 0x80, 0x03, 0x00, 0xf1, 0x81, 0x04,
        ];
        test_encode_decode(&req, &exp);
    }

    #[ignore] // Impossible to deserialize `vec` of unkonwn beforehand size
    #[test]
    fn dtc_response_mirror_memory_dtc_ext_data_record_by_dtc_number() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseMirrorMemoryDTCExtDataRecordByDTCNumber(
                ResponseDTCExtDataRecordByDTCNumber {
                    dtc_status: DtcAndStatusRecord {
                        dtc: 0x00f180.into(),
                        status: 0xa0,
                    },
                    records: vec![
                        ExtDataRecordAndNumber {
                            record_number: 7,
                            extended_data: vec![0xa0, 0xb0],
                        },
                        ExtDataRecordAndNumber {
                            record_number: 8,
                            extended_data: vec![0xa1, 0xb1],
                        },
                    ],
                },
            ),
        });
        let exp = vec![
            0x59, 0x10, 0x00, 0xf1, 0x80, 0xa0, 7, 0xa0, 0xb0, 8, 0xa1, 0xb1,
        ];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_number_of_mirror_memory_dtc_by_status_mask() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseNumberOfMirrorMemoryDTCByStatusMask(
                ResponseNumberOfMirrorMemoryDTCByStatusMask {
                    mask: 0xa8,
                    format: 0xb0,
                    count: 2,
                },
            ),
        });
        let exp = vec![0x59, 0x11, 0xa8, 0xb0, 0x00, 0x02];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_number_of_emissions_obddtc_by_status_mask() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseNumberOfEmissionsOBDDTCByStatusMask(
                ResponseNumberOfEmissionsOBDDTCByStatusMask {
                    mask: 0xa8,
                    format: 0xb0,
                    count: 2,
                },
            ),
        });
        let exp = vec![0x59, 0x12, 0xa8, 0xb0, 0x00, 0x02];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_emissions_obddtc_by_status_mask() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseEmissionsOBDDTCByStatusMask(
                ResponseEmissionsOBDDTCByStatusMask {
                    availability_mask: 0x07,
                    dtcs: vec![
                        DtcAndStatusRecord {
                            dtc: 0x00f180.into(),
                            status: 0x03,
                        },
                        DtcAndStatusRecord {
                            dtc: 0x00f181.into(),
                            status: 0x04,
                        },
                    ],
                },
            ),
        });
        let exp = vec![
            0x59, 0x13, 0x07, 0x00, 0xf1, 0x80, 0x03, 0x00, 0xf1, 0x81, 0x04,
        ];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_dtc_fault_detection_counter() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseDTCFaultDetectionCounter(
                ResponseDTCFaultDetectionCounter {
                    faults: vec![
                        DtcAndFaultCounter {
                            dtc: 0x00f180.into(),
                            counter: 1,
                        },
                        DtcAndFaultCounter {
                            dtc: 0x00f181.into(),
                            counter: 1,
                        },
                    ],
                },
            ),
        });
        let exp = vec![0x59, 0x14, 0x00, 0xf1, 0x80, 1, 0x00, 0xf1, 0x81, 1];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_dtc_with_permanent_status() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseDTCWithPermanentStatus(
                ResponseDTCWithPermanentStatus {
                    availability_mask: 0x07,
                    dtcs: vec![
                        DtcAndStatusRecord {
                            dtc: 0x00f180.into(),
                            status: 0x03,
                        },
                        DtcAndStatusRecord {
                            dtc: 0x00f181.into(),
                            status: 0x04,
                        },
                    ],
                },
            ),
        });
        let exp = vec![
            0x59, 0x15, 0x07, 0x00, 0xf1, 0x80, 0x03, 0x00, 0xf1, 0x81, 0x04,
        ];
        test_encode_decode(&req, &exp);
    }

    #[ignore] // Impossible to deserialize `vec` of unkonwn beforehand size
    #[test]
    fn dtc_response_dtc_ext_data_record_by_record_number() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseDTCExtDataRecordByRecordNumber(
                ResponseDTCExtDataRecordByRecordNumber {
                    dtc_status: DtcAndStatusRecord {
                        dtc: 0x00f180.into(),
                        status: 0xa0,
                    },
                    records: vec![
                        DtcAndStatusAndExtDataRecord {
                            dtc_status: DtcAndStatusRecord {
                                dtc: 0x00f180.into(),
                                status: 0xc0,
                            },
                            extended_data: vec![0xa0, 0xb0],
                        },
                        DtcAndStatusAndExtDataRecord {
                            dtc_status: DtcAndStatusRecord {
                                dtc: 0x00f181.into(),
                                status: 0xc1,
                            },
                            extended_data: vec![0xa1, 0xb1],
                        },
                    ],
                },
            ),
        });
        let exp = vec![
            0x59, 0x16, 0x00, 0xf1, 0x80, 0xa0, 0x00, 0xf1, 0x80, 0xc0, 0xa0, 0xb0, 0x00, 0xf1,
            0x81, 0xc1, 0xa1, 0xb1,
        ];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_dtc_user_def_memory_dtc_by_status_mask() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseUserDefMemoryDTCByStatusMask(
                ResponseUserDefMemoryDTCByStatusMask {
                    memory: 1,
                    availability_mask: 0x03,
                    records: vec![
                        DtcAndStatusRecord {
                            dtc: 0x00f180.into(),
                            status: 0x03,
                        },
                        DtcAndStatusRecord {
                            dtc: 0x00f181.into(),
                            status: 0x04,
                        },
                    ],
                },
            ),
        });
        let exp = vec![
            0x59, 0x17, 1, 0x03, 0x00, 0xf1, 0x80, 0x03, 0x00, 0xf1, 0x81, 0x04,
        ];
        test_encode_decode(&req, &exp);
    }

    #[ignore] // Impossible to deserialize `vec` of unkonwn beforehand size
    #[test]
    fn dtc_response_user_def_memory_dtc_snapshot_record_by_dtc_number() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseUserDefMemoryDTCSnapshotRecordByDTCNumber(
                ResponseUserDefMemoryDTCSnapshotRecordByDTCNumber {
                    memory: 1,
                    dtc_status: DtcAndStatusRecord {
                        dtc: 0x00f180.into(),
                        status: 0x03,
                    },
                    records: vec![
                        RecordNumberAndIdentifierAndSnapshot {
                            record_number: 1,
                            number_identifiers: 2,
                            record: DidSnapshot {
                                did: 0x1004,
                                value: vec![0xaa, 0xcc],
                            },
                        },
                        RecordNumberAndIdentifierAndSnapshot {
                            record_number: 3,
                            number_identifiers: 1,
                            record: DidSnapshot {
                                did: 0x1004,
                                value: vec![0xaa, 0xcc],
                            },
                        },
                    ],
                },
            ),
        });
        let exp = vec![0x59, 0x18, 1, 0xf0, 0x00, 0x00, 0x03, 0x07];
        test_encode_decode(&req, &exp);
    }

    #[ignore] // Impossible to deserialize `vec` of unkonwn beforehand size
    #[test]
    fn dtc_response_user_def_memory_dtc_ex_data_record_by_dtc_number() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseUserDefMemoryDTCExtDataRecordByDTCNumber(
                ResponseUserDefMemoryDTCExtDataRecordByDTCNumber {
                    memory: 1,
                    dtc_status: DtcAndStatusRecord {
                        dtc: 0x00f180.into(),
                        status: 0x03,
                    },
                    records: vec![
                        ExtDataRecordAndNumber {
                            record_number: 7,
                            extended_data: vec![0xa0, 0xb0],
                        },
                        ExtDataRecordAndNumber {
                            record_number: 8,
                            extended_data: vec![0xa1, 0xb1],
                        },
                    ],
                },
            ),
        });
        let exp = vec![
            0x59, 0x18, 1, 0xf0, 0x00, 0x00, 0x03, 7, 0xa0, 0xb0, 8, 0xa1, 0xb1,
        ];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_wwobddtc_by_mask_record() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseWWHOBDDTCByMaskRecord(vec![0x41, 0x42, 0x43]),
        });
        let exp = vec![0x59, 0x42, 0x41, 0x42, 0x43];
        test_encode_decode(&req, &exp);
    }

    #[test]
    fn dtc_response_wwobddtc_with_permanent_status() {
        let req = UdsMessage::ReadDTCRsp(ReadDTCRsp {
            sub: DTCRspSubfunction::ResponseWWHOBDDTCWithPermanentStatus(vec![0x41, 0x42, 0x43]),
        });
        let exp = vec![0x59, 0x55, 0x41, 0x42, 0x43];
        test_encode_decode(&req, &exp);
    }
}
