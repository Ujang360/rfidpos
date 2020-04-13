use crc16::{State, GENIBUS};
use rfidpos_proto::ProtoUnion;

fn main() {
    let mut proto_instance = ProtoUnion::default();
    println!("{:02X?}", proto_instance.borrow_raw_frame());
    proto_instance.fill_with_empty_frame(true, 0, 0xB0);
    println!("{:02X?}", proto_instance.borrow_raw_frame());
    let sample_data = vec![0xC1, 0x00, 0xF0];
    let crc16_genibus = State::<GENIBUS>::calculate(&sample_data);
    println!("{:#04X}", crc16_genibus);
}
