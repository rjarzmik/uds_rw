#[allow(dead_code)]
mod common;

use common::test_encode_decode;
use uds_rw::UdsMessage;

#[test]
fn read_did_req_ok() {
    use uds_rw::message::ReadDIDReq;
    let req = UdsMessage::ReadDIDReq(ReadDIDReq { did: 0xf180 });
    let exp = vec![0x22, 0xf1, 0x80];
    test_encode_decode(&req, &exp);
}

#[test]
fn read_did_rsp_ok() {
    use uds_rw::message::ReadDIDRsp;
    let req = UdsMessage::ReadDIDRsp(ReadDIDRsp {
        did: 0xf180,
        user_data: vec![0x10, 0x14],
    });
    let exp = vec![0x62, 0xf1, 0x80, 0x10, 0x14];
    test_encode_decode(&req, &exp);
}

#[test]
fn write_did_req_ok() {
    use uds_rw::message::WriteDIDReq;
    let req = UdsMessage::WriteDIDReq(WriteDIDReq {
        did: 0xf180,
        user_data: vec![0x10, 0xb0],
    });
    let exp = vec![0x2e, 0xf1, 0x80, 0x10, 0xb0];
    test_encode_decode(&req, &exp);
}

#[test]
fn write_did_rsp_ok() {
    use uds_rw::message::WriteDIDRsp;
    let req = UdsMessage::WriteDIDRsp(WriteDIDRsp { did: 0xf180 });
    let exp = vec![0x6e, 0xf1, 0x80];
    test_encode_decode(&req, &exp);
}
