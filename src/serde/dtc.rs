use super::deserializer::DecodeError;
use super::Payload;
use crate::proto::dtc::*;
use crate::UdsError::{self, *};
use std::io::{Read, Write};

impl Payload for ReadDTCReq {
    fn length(&self) -> usize {
        1
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, UdsError> {
        super::default_read(reader, payload_length)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), UdsError> {
        if payload_length < 1 {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: 1u32,
            });
        }
        let mut v: Vec<u8> = vec![0; payload_length];
        reader.read_exact(&mut v)?;
        let sub: DTCReqSubfunction = super::deserializer::from_bytes(&v).map_err(|e| match e {
            DecodeError::Custom(msg) => UdsError::EncodingError { msg },
        })?;
        self.sub = sub;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        super::serializer::to_writer(writer, &self.sub).map_err(|e| match e {
            crate::serde::serializer::EncodeError::Custom(msg) => UdsError::EncodingError { msg },
            crate::serde::serializer::EncodeError::Io(io) => UdsError::Io(io),
        })
    }
}

impl Payload for ReadDTCRsp {
    fn length(&self) -> usize {
        todo!()
    }

    fn read<T: Read>(reader: &mut T, payload_length: usize) -> Result<Self, UdsError> {
        super::default_read(reader, payload_length)
    }

    fn read_replace<T: Read>(
        &mut self,
        reader: &mut T,
        payload_length: usize,
    ) -> Result<(), UdsError> {
        if payload_length < 1 {
            return Err(PayloadLengthTooShort {
                value: payload_length as u32,
                expected: 1u32,
            });
        }
        let mut v: Vec<u8> = vec![0; payload_length];
        reader.read_exact(&mut v)?;
        let sub: DTCRspSubfunction = super::deserializer::from_bytes(&v).map_err(|e| match e {
            DecodeError::Custom(msg) => UdsError::EncodingError { msg },
        })?;
        self.sub = sub;
        Ok(())
    }

    fn write<T: Write>(&self, writer: &mut T) -> Result<(), UdsError> {
        super::serializer::to_writer(writer, &self.sub).map_err(|e| match e {
            crate::serde::serializer::EncodeError::Custom(msg) => UdsError::EncodingError { msg },
            crate::serde::serializer::EncodeError::Io(io) => UdsError::Io(io),
        })
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::proto::dtc;
    use crate::{uds_read, UdsMessage};

    use super::{DTCRspSubfunction, DtcAndStatusRecord};

    #[test]
    fn available_dtc_deserialize() {
        let req = [
            0x59, 0x0a, 0xff, 0xea, 0x19, 0x88, 0x00, 0xad, 0xa6, 0x11, 0x50,
        ];
        let msg = uds_read(&mut Cursor::new(req), req.len()).unwrap();
        let supported = dtc::ResponseSupportedDTC {
            availability_mask: 0xff,
            dtcs: vec![
                DtcAndStatusRecord {
                    dtc: dtc::Dtc::new(0xea, 0x19, 0x88),
                    status: 0x00,
                },
                DtcAndStatusRecord {
                    dtc: dtc::Dtc::new(0xad, 0xa6, 0x11),
                    status: 0x50,
                },
            ],
        };
        let expected_sub: DTCRspSubfunction = DTCRspSubfunction::ResponseSupportedDTC(supported);
        let rsp = match msg {
            UdsMessage::ReadDTCRsp(rsp) => rsp,
            _ => panic!("Decoding didn't give back a ReadDTCRsp"),
        };
        assert_eq!(&rsp.sub, &expected_sub);
    }
}
