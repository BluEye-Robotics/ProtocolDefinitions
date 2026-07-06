//! Tests for the opt-in `serde` feature: pbjson-generated protobuf-JSON
//! serialization for `PersistentStorageSettings`, matching the conventions of
//! C++ `MessageToJsonString` as used for `.persistent_storage_settings.json`
//! (snake_case proto field names, all fields emitted, unknown fields ignored).
#![cfg(feature = "serde")]

use blueye_protocol::PersistentStorageSettings;

fn mixed_settings() -> PersistentStorageSettings {
    PersistentStorageSettings {
        videos: true,
        images: false,
        binlog: true,
        multibeam: false,
        webserver_log: true,
        control_system_log: false,
        gyro_calibration: false,
        compass_calibration: true,
        acc_calibration: false,
    }
}

#[test]
fn serialize_uses_snake_case_proto_field_names() {
    let value = serde_json::to_value(mixed_settings()).expect("serialize");
    let object = value.as_object().expect("must serialize to a JSON object");

    assert!(object.contains_key("webserver_log"));
    assert!(!object.contains_key("webserverLog"));
    assert!(object.contains_key("control_system_log"));
    assert!(!object.contains_key("controlSystemLog"));
}

#[test]
fn serialize_emits_all_fields_including_false() {
    let value = serde_json::to_value(mixed_settings()).expect("serialize");
    let object = value.as_object().expect("must serialize to a JSON object");

    let expected_keys = [
        "videos",
        "images",
        "binlog",
        "multibeam",
        "webserver_log",
        "control_system_log",
        "gyro_calibration",
        "compass_calibration",
        "acc_calibration",
    ];
    for key in expected_keys {
        assert!(object.contains_key(key), "missing key {key:?} in {object:?}");
    }
    assert_eq!(object.len(), expected_keys.len());

    assert_eq!(object["videos"], serde_json::Value::Bool(true));
    assert_eq!(object["images"], serde_json::Value::Bool(false));
}

#[test]
fn deserialize_ignores_unknown_fields() {
    let parsed: PersistentStorageSettings =
        serde_json::from_str(r#"{"videos": true, "some_future_field": 42}"#)
            .expect("unknown fields must be ignored");

    assert!(parsed.videos);
}

#[test]
fn deserialize_accepts_both_snake_case_and_camel_case() {
    let snake: PersistentStorageSettings = serde_json::from_str(r#"{"webserver_log": true}"#)
        .expect("snake_case field name must parse");
    assert!(snake.webserver_log);

    let camel: PersistentStorageSettings = serde_json::from_str(r#"{"webserverLog": true}"#)
        .expect("camelCase field name must parse");
    assert!(camel.webserver_log);
}

#[test]
fn json_roundtrip_preserves_values() {
    let original = mixed_settings();
    let json = serde_json::to_string(&original).expect("serialize");
    let decoded: PersistentStorageSettings = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(original, decoded);
}
