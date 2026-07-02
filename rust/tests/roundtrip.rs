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

#[test]
fn reflect_descriptor_full_name() {
    use blueye_protocol::prost_reflect::ReflectMessage;

    let msg = Attitude::default();
    assert_eq!(msg.descriptor().full_name(), "blueye.protocol.Attitude");
}

#[test]
fn descriptor_pool_lookup_builds_dynamic_message() {
    use blueye_protocol::prost_reflect::{DynamicMessage, ReflectMessage};
    use blueye_protocol::DESCRIPTORS;

    let descriptor = DESCRIPTORS
        .get_message_by_name("blueye.protocol.Attitude")
        .expect("Attitude must be in the descriptor pool");
    let dynamic = DynamicMessage::new(descriptor);

    assert_eq!(dynamic.descriptor().full_name(), "blueye.protocol.Attitude");
}

#[test]
fn any_payload_roundtrip() {
    use blueye_protocol::prost_types::Any;

    let original = Attitude {
        roll: 1.0,
        pitch: 2.0,
        yaw: 3.0,
    };

    let any = Any::from_msg(&original).expect("encode into Any");
    assert_eq!(any.type_url, "type.googleapis.com/blueye.protocol.Attitude");

    let decoded = any.to_msg::<Attitude>().expect("decode from Any");
    assert_eq!(original, decoded);
}
