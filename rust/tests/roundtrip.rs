use blueye_protocol::{Attitude, Message, SystemTime};

#[test]
fn attitude_roundtrip() {
    let original = Attitude {
        roll: 1.5,
        pitch: -10.25,
        yaw: 179.0,
    };

    let bytes = original.encode_to_vec();
    let decoded = Attitude::decode(bytes.as_slice()).expect("decode failed");

    assert_eq!(original, decoded);
}

#[test]
fn timestamp_roundtrip() {
    let original = SystemTime {
        unix_timestamp: Some(blueye_protocol::prost_types::Timestamp {
            seconds: 1_700_000_000,
            nanos: 123_456_789,
        }),
    };

    let bytes = original.encode_to_vec();
    let decoded = SystemTime::decode(bytes.as_slice()).expect("decode failed");

    assert_eq!(original, decoded);
}

#[test]
fn decode_rejects_garbage() {
    // Tag 0x08 claims field 1 is varint-encoded, but Attitude.roll is a
    // float (fixed32) — wire-type mismatch must error, not panic.
    let garbage: &[u8] = &[0x08];
    assert!(Attitude::decode(garbage).is_err());
}
