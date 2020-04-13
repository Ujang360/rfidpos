use rfidpos_proto::ProtoUnion;

fn main() {
    let mut proto_instance = ProtoUnion::default();
    println!("{:02X?}", proto_instance.borrow_raw_frame());
    proto_instance.fill_with_empty_frame(true, 0, 0xEC);
    println!("{:02X?}", proto_instance.borrow_raw_frame());
}
