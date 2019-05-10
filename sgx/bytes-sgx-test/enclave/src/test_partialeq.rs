use std::prelude::v1::*;
use serde_bytes::{Bytes, ByteBuf};

pub fn _bytes_eq_slice(bytes: &Bytes, slice: &[u8]) -> bool {
    bytes == slice
}

pub fn _bytebuf_eq_vec(bytebuf: ByteBuf, vec: Vec<u8>) -> bool {
    bytebuf == vec
}

pub fn _bytes_eq_bytestring(bytes: &Bytes) -> bool {
    bytes == b"..."
}
