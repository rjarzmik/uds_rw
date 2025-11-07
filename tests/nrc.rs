#[allow(dead_code)]
mod common;

use common::test_encode_decode;
use uds_rw::UdsMessage;

#[test]
fn nrc_ok() {
    use uds_rw::message::{Nrc, NrcCode};
    let req = UdsMessage::Nrc(Nrc {
        sid: 0x34,
        nrc: NrcCode::GeneralReject.into(),
    });
    let exp = vec![0x7f, 0x34, 0x10];
    test_encode_decode(&req, &exp);
}
