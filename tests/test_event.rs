use binrw::BinRead;
use std::{error::Error, io::Cursor};

use f1_2021_rust::{Body, Packet};

static DATA_EXAMPLE: &[u8] = &[
  229, 7,  1,   5,   1,   3,  187, 134, 38, 178, 108, 178,
  251, 17, 189, 180, 189, 68, 166, 123, 0,  0,   19,  255,
  66,  85, 84,  78,  4,   0,  0,   0,   24, 0,   0,   0,];

#[test]
fn test_event() -> Result<(), Box<dyn Error>> {
    let pkt = Packet::read(&mut Cursor::new(DATA_EXAMPLE))?;

    // eprintln!("{:#?}", pkt);
    // assert!(false);

    assert!(matches!(pkt.body, Body::Event(_)));

    Ok(())
}
