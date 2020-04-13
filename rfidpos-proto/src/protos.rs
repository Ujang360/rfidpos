use crc16::{State, GENIBUS};
use static_assertions::assert_eq_size;
use std::sync::Arc;

pub const PROTO_HEADER: u16 = 0xBEEF;
pub const PROTO_LENGTH: usize = 8;
pub const CRC16_OFFSET: usize = 6;

const FLAG_ACCESS: i32 = -2_147_483_648_i32;
const FLAG_WRITE: i32 = 0x4000_0000;
const MASK_RANK: i32 = 0b11;
const MASK_REGISTER: i32 = 0b1111_1111_1111;
const MASK_DATA: i32 = 0xFFFF;
const BIT_SHIFT_RANK: usize = 28;
const BIT_SHIFT_REGISTER: usize = 16;

#[repr(C, packed(1))]
struct ProtoFrame {
    header: u16,
    payload: i32,
    crc16_genibus: u16,
}

#[repr(C, packed(1))]
pub union ProtoUnion {
    pub(crate) raw_frame: [u8; PROTO_LENGTH],
    data_frame: ProtoFrame,
}

impl ProtoFrame {
    fn new() -> ProtoFrame {
        ProtoFrame {
            header: PROTO_HEADER,
            payload: 0,
            crc16_genibus: 0xE38D,
        }
    }
}

impl ProtoUnion {
    fn update_crc16(&mut self) {
        unsafe {
            self.data_frame.crc16_genibus = self.compute_crc16();
        }
    }

    fn compute_crc16(&self) -> u16 {
        unsafe { State::<GENIBUS>::calculate(&(&self.raw_frame)[0..(CRC16_OFFSET - 1)]) }
    }

    pub fn update_from_raw_bytes(&mut self, raw_bytes: &[u8]) -> Result<(), String> {
        unsafe {
            self.raw_frame[..].clone_from_slice(&raw_bytes[..]);

            if self.compute_crc16() != self.data_frame.crc16_genibus {
                return Err("Invalid CRC-16, ensure remote calculate CRC-16 using GENIBUS standard!".into());
            }
        }

        Ok(())
    }

    pub fn borrow_raw_frame(&self) -> Arc<&[u8; PROTO_LENGTH]> {
        unsafe { Arc::new(&self.raw_frame) }
    }

    pub fn fill_with_empty_frame(&mut self, is_write_mode: bool, rank: i32, register: i32) {
        self.fill(is_write_mode, rank, register, 0);
    }

    pub fn fill(&mut self, is_write_mode: bool, rank: i32, register: i32, data: i32) {
        unsafe {
            self.data_frame.payload = data & MASK_DATA;
            self.data_frame.payload |= (rank & MASK_RANK) << BIT_SHIFT_RANK;
            self.data_frame.payload |= (register & MASK_REGISTER) << BIT_SHIFT_REGISTER;

            if data != 0 {
                self.data_frame.payload |= FLAG_ACCESS;
            }

            if is_write_mode {
                self.data_frame.payload |= FLAG_WRITE;
            }
        }
        self.update_crc16();
    }

    pub fn parse_info(&self) -> (bool, bool, i32, i32, i32) {
        unsafe {
            let is_access = self.data_frame.payload & FLAG_ACCESS != 0;
            let is_write = self.data_frame.payload & FLAG_WRITE != 0;
            let rank = (self.data_frame.payload >> BIT_SHIFT_RANK) & MASK_RANK;
            let register = (self.data_frame.payload >> BIT_SHIFT_REGISTER) & MASK_RANK;
            let data = self.data_frame.payload & MASK_DATA;

            (is_access, is_write, rank, register, data)
        }
    }
}

impl Default for ProtoUnion {
    fn default() -> ProtoUnion {
        ProtoUnion {
            data_frame: ProtoFrame::new(),
        }
    }
}

assert_eq_size!(ProtoFrame, [u8; PROTO_LENGTH]);
assert_eq_size!(ProtoUnion, [u8; PROTO_LENGTH]);
