use crc16::{State, GENIBUS};
use static_assertions::assert_eq_size;
use std::sync::Arc;

pub const PROTO_HEADER: u16 = 0xBEEF;
pub const PROTO_LENGTH: usize = 8;
pub const CRC16_OFFSET: usize = 6;

const FLAG_ACCESS: u16 = 0x8000;
const FLAG_WRITE: u16 = 0x4000;
const MASK_RANK: u16 = 0b11;
const MASK_REGISTER: u16 = 0b1111_1111_1111;
const LSHIFT_RANK: usize = 12;

#[repr(C, packed(8))]
struct ProtoFrame {
    header: u16,
    segment0: u16,
    segment1: u16,
    crc16_genibus: u16,
}

#[repr(C, packed(8))]
pub union ProtoUnion {
    raw_frame: [u8; PROTO_LENGTH],
    data_frame: ProtoFrame,
}

impl ProtoFrame {
    fn new() -> ProtoFrame {
        ProtoFrame {
            header: PROTO_HEADER,
            segment0: 0,
            segment1: 0,
            crc16_genibus: 0xE38D,
        }
    }
}

impl ProtoUnion {
    pub fn update_crc16(&mut self) {
        unsafe {
            self.data_frame.crc16_genibus =
                State::<GENIBUS>::calculate(&(&self.raw_frame)[0..(CRC16_OFFSET - 1)]);
        }
    }

    pub fn borrow_raw_frame(&self) -> Arc<&[u8; PROTO_LENGTH]> {
        unsafe { Arc::new(&self.raw_frame) }
    }

    pub fn fill_with_empty_frame(&mut self, is_write_mode: bool, rank: u16, register: u16) {
        self.fill(is_write_mode, rank, register, 0);
    }

    pub fn fill(&mut self, is_write_mode: bool, rank: u16, register: u16, data: u16) {
        unsafe {
            self.data_frame.segment0 = 0;
            self.data_frame.segment0 |= (rank & MASK_RANK) << LSHIFT_RANK;
            self.data_frame.segment0 |= register & MASK_REGISTER;
            self.data_frame.segment1 = data;

            if data != 0 {
                self.data_frame.segment0 |= FLAG_ACCESS;
            }

            if is_write_mode {
                self.data_frame.segment0 |= FLAG_WRITE;
            }
        }
        self.update_crc16();
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
