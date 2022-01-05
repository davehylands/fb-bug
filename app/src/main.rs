use flatbuffers::{FlatBufferBuilder, root};

mod fb_generated;

use fb_generated::*;
use std::io::Result;

fn make_flatbuffer() -> Vec<u8> {
    let mut fbb = FlatBufferBuilder::new();

    let sub_root_proto1_args = SubRootProto1Args {
        sub_field1: 234,
    };
    let sub_root1_offset = SubRootProto1::create(&mut fbb, &sub_root_proto1_args);

    let sub_root_proto2_args = SubRootProto2Args {
        timestamp: 123,
        sub_field1: 456,
    };
    let sub_root2_offset = SubRootProto2::create(&mut fbb, &sub_root_proto2_args);

    let data : &[i8] = &[1,2,3,4,5];
    let root_args = RootProtoArgs {
        field1: Some(fbb.create_string("A string2")),
        field2: Some(fbb.create_vector(data)),
        sub_root1: Some(sub_root1_offset),
        sub_root2: Some(sub_root2_offset),
        field3: 789,
    };
    let root_offset = RootProto::create(&mut fbb, &root_args);
    fbb.finish(root_offset, None);

    fbb.finished_data().to_vec()
}

fn main() -> Result<()> {

    let fb_data = make_flatbuffer();

    std::fs::write("fb.bin", &fb_data)?;

    let root = root::<RootProto>(&fb_data).unwrap();

    if let Some(sub_root) = root.sub_root2() {
        let vt = sub_root._tab.vtable();
        println!("sub_root = {} buf.len = {} vt.num_fields = {} vt.num_bytes = {} timestamp = 0x{:x}",
        sub_root._tab.loc, sub_root._tab.buf.len(), vt.num_fields(), vt.num_bytes(), sub_root.timestamp());

        let ts_offset = sub_root._tab.loc + (vt.get(4) as usize);
        if ts_offset % 8 != 0 {
            println!("*** timestamp offset not 8-byte aligned ***");
        }

        for i in 0..vt.num_fields() {
            let slot_loc = (i as u16) * 2 + 4;
            println!("sub_root[{}] = {:2} {}", i, vt.get(slot_loc), sub_root._tab.loc + (vt.get(slot_loc) as usize));
        }
    }


    Ok(())
}
