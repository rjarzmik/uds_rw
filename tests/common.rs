use std::{
    cmp::min,
    io::{self},
};

use uds_rw::{uds_read, uds_write, UdsError, UdsMessage};

struct Buffer {
    cursor: usize,
    memory: Vec<u8>,
}

impl io::Read for Buffer {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        println!(
            "read: {:x?}[{}] into buffer of {} bytes",
            &self.memory,
            self.cursor,
            buf.len()
        );
        if self.cursor < self.memory.len() {
            let left = self.memory.len() - self.cursor;
            let nb = min(left, buf.len());
            let source = &self.memory[self.cursor..self.cursor + nb];
            buf[0..nb].copy_from_slice(source);
            self.cursor += nb;
            Ok(nb)
        } else {
            Ok(0)
        }
    }
}

impl Buffer {
    pub fn new(avec: &[u8]) -> Self {
        Self {
            cursor: 0,
            memory: avec.to_owned(),
        }
    }
}

pub fn test_encode_decode(msg: &UdsMessage, expected: &[u8]) {
    let mut ser = vec![];
    uds_write(&mut ser, msg).unwrap();
    assert_eq!(expected, &ser);
    let req_back = uds_read(&mut Buffer::new(&ser), ser.len()).unwrap();
    assert_eq!(&req_back, msg);
}

pub fn test_decode_serialized_truncated(truncated: &[u8]) {
    let req_back = uds_read(&mut Buffer::new(truncated), truncated.len());
    println!("RJK: {req_back:?}");
    assert!(matches!(req_back, Err(UdsError::Io(..))));
}
