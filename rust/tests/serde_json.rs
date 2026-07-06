//! Tests for the opt-in `serde` feature: pbjson-generated protobuf-JSON
//! serialization for `PersistentStorageSettings`, matching the conventions of
//! C++ `MessageToJsonString` as used for `.persistent_storage_settings.json`
//! (snake_case proto field names, all fields emitted, unknown fields ignored).
#![cfg(feature = "serde")]

use blueye_protocol::PersistentStorageSettings;

fn mixed_settings() -> PersistentStorageSettings {
    PersistentStorageSettings {
        videos: true,
        binlog: true,
        webserver_log: true,
        compass_calibration: true,
        ..Default::default()
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
    use blueye_protocol::prost_reflect::ReflectMessage;
    use std::collections::BTreeSet;

    let settings = mixed_settings();
    let value = serde_json::to_value(&settings).expect("serialize");
    let object = value.as_object().expect("must serialize to a JSON object");

    // The emitted keys must match the schema exactly, so every field appears
    // even when it holds its default value — the point of generating the
    // serializer is that new proto fields can't silently go missing from the
    // settings file. Deriving the expected set from the descriptor keeps this
    // test in sync with the schema automatically.
    let schema_fields: BTreeSet<String> = settings
        .descriptor()
        .fields()
        .map(|field| field.name().to_string())
        .collect();
    let emitted_keys: BTreeSet<String> = object.keys().cloned().collect();
    assert_eq!(emitted_keys, schema_fields);

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
