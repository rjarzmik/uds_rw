mod common;

use common::{test_decode_serialized_truncated, test_encode_decode};
use uds_rw::UdsMessage;

#[test]
fn request_download_req_ok() {
    use uds_rw::message::RequestDownloadReq;
    let req = UdsMessage::RequestDownloadReq(RequestDownloadReq {
        compression_method: 0x01,
        encryption_method: 0x02,
        memory_size_bytes: 2,
        memory_address_bytes: 4,
        memory_address: 0x1028,
        memory_size: 0x10,
    });
    let exp = vec![0x34, 0x12, 0x24, 0x00, 0x00, 0x10, 0x28, 0x00, 0x10];
    test_encode_decode(&req, &exp);
}

#[test]
fn request_download_req_serialized_truncated() {
    let truncated = vec![0x34, 0x12, 0x24, 0x00, 0x00, 0x10, 0x28, 0x00];
    test_decode_serialized_truncated(&truncated);
}

#[test]
fn request_download_rsp_ok() {
    use uds_rw::message::RequestDownloadRsp;
    let req = UdsMessage::RequestDownloadRsp(RequestDownloadRsp {
        max_block_size_bytes: 2,
        max_block_size: 0x0210,
    });
    let exp = vec![0x74, 0x20, 0x02, 0x10];
    test_encode_decode(&req, &exp);
}

#[test]
fn request_download_rsp_serialized_truncated() {
    let truncated = vec![0x34, 0x12, 0x24, 0x00, 0x00, 0x10, 0x28, 0x00];
    test_decode_serialized_truncated(&truncated);
}

#[test]
fn request_file_transfer_req_ok() {
    use uds_rw::message::{ModeOfOperation, RequestFileTransferReq};
    let req = UdsMessage::RequestFileTransferReq(RequestFileTransferReq {
        mode_of_operation: ModeOfOperation::AddFile,
        path_name: "toto.txt".to_string(),
        compression_method: 0x1,
        encryption_method: 0x2,
        file_size_bytes: 2,
        file_size_uncompressed: 0x1234,
        file_size_compressed: 0x5678,
    });
    let mut exp = vec![0x38, 0x01, 0x00, "toto.txt".len() as u8];
    exp.extend_from_slice(b"toto.txt");
    exp.extend_from_slice(&[0x12, 0x2, 0x12, 0x34, 0x56, 0x78]);
    test_encode_decode(&req, &exp);
}

#[test]
fn request_file_transfer_req_truncated() {
    let truncated = vec![0x34, 0x12, 0x24, 0x00, 0x00, 0x10, 0x28, 0x00];
    test_decode_serialized_truncated(&truncated);
}

#[test]
fn request_file_transfer_rsp_ok() {
    use uds_rw::message::{ModeOfOperation, RequestFileTransferRsp};
    let req = UdsMessage::RequestFileTransferRsp(RequestFileTransferRsp {
        mode_of_operation: ModeOfOperation::ReadFile,
        max_block_size_bytes: 2,
        max_block_size: 0x1022,
        compression_method: 0x1,
        encryption_method: 0x2,
        file_dir_data_size_bytes: 2,
        file_dir_size_uncompressed: 0x2345,
        file_size_compressed: 0x6789,
    });
    let exp = vec![
        0x78, 0x04, 0x02, 0x10, 0x22, 0x12, 0x00, 0x02, 0x23, 0x45, 0x67, 0x89,
    ];
    test_encode_decode(&req, &exp);
}

#[test]
fn request_file_transfer_rsp_truncated() {
    let truncated = vec![
        0x78, 0x04, 0x02, 0x10, 0x22, 0x12, 0x00, 0x02, 0x23, 0x45, 0x67,
    ];

    test_decode_serialized_truncated(&truncated);
}

#[test]
fn transfer_data_req_ok() {
    use uds_rw::message::TransferDataReq;
    let req = UdsMessage::TransferDataReq(TransferDataReq {
        block_sequence_counter: 3,
        data: vec![0xaa, 0xbb, 0xcc],
    });
    let exp = vec![0x36, 0x03, 0xaa, 0xbb, 0xcc];
    test_encode_decode(&req, &exp);
}

#[test]
fn transfer_data_rsp_ok() {
    use uds_rw::message::TransferDataRsp;
    let req = UdsMessage::TransferDataRsp(TransferDataRsp {
        block_sequence_counter: 4,
    });
    let exp = vec![0x76, 0x04];
    test_encode_decode(&req, &exp);
}

#[test]
fn transfer_exit_req_ok() {
    use uds_rw::message::TransferExitReq;
    let req = UdsMessage::TransferExitReq(TransferExitReq { user_data: vec![] });
    let exp = vec![0x37];
    test_encode_decode(&req, &exp);
}

#[test]
fn transfer_exit_rsp_ok() {
    use uds_rw::message::TransferExitRsp;
    let req = UdsMessage::TransferExitRsp(TransferExitRsp { user_data: vec![] });
    let exp = vec![0x77];
    test_encode_decode(&req, &exp);
}
