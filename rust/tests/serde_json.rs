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

// --- unknown enum values ------------------------------------------------------
//
// A guest-port connector EEPROM can hold a device id newer than the firmware
// reading it (a Water Linked DVL A100 flashed before the drone knew that id).
// C++ `MessageToJsonString` renders such an out-of-range enum value as its bare
// number and keeps going; the JSON here must do the same, or one unrecognised
// device costs the reader the entire message.

use blueye_protocol::guest_port_connector_info::ConnectedDevice;
use blueye_protocol::{
    GuestPortConnectorInfo, GuestPortDevice, GuestPortDeviceId, GuestPortDeviceList, GuestPortInfo,
    GuestPortNumber,
};

/// An id well past the end of `GuestPortDeviceID`, standing in for a device
/// this build has never heard of.
const UNKNOWN_DEVICE_ID: i32 = 4242;

fn connector_with(port: GuestPortNumber, devices: Vec<GuestPortDevice>) -> GuestPortConnectorInfo {
    GuestPortConnectorInfo {
        guest_port_number: port as i32,
        capabilities: Vec::new(),
        connected_device: Some(ConnectedDevice::DeviceList(GuestPortDeviceList { devices })),
    }
}

#[test]
fn unknown_enum_value_serializes_as_its_number() {
    let device = GuestPortDevice {
        device_id: UNKNOWN_DEVICE_ID,
        name: "Unknown device".to_string(),
        ..Default::default()
    };

    let value = serde_json::to_value(&device).expect("an unknown enum value must still serialize");

    assert_eq!(value["device_id"], serde_json::json!(UNKNOWN_DEVICE_ID));
    assert_eq!(value["name"], serde_json::json!("Unknown device"));
}

#[test]
fn known_enum_value_still_serializes_as_its_declared_name() {
    let device = GuestPortDevice {
        device_id: GuestPortDeviceId::BlueyeCam as i32,
        ..Default::default()
    };

    let value = serde_json::to_value(&device).expect("serialize");

    assert_eq!(
        value["device_id"],
        serde_json::json!("GUEST_PORT_DEVICE_ID_BLUEYE_CAM")
    );
}

#[test]
fn unknown_enum_value_in_a_repeated_field_serializes_as_its_number() {
    let device = GuestPortDevice {
        compatible_guest_ports: vec![GuestPortNumber::Port1 as i32, 77],
        ..Default::default()
    };

    let value = serde_json::to_value(&device).expect("an unknown enum value must still serialize");

    assert_eq!(
        value["compatible_guest_ports"],
        serde_json::json!(["GUEST_PORT_NUMBER_PORT_1", 77])
    );
}

#[test]
fn unknown_enum_value_in_a_oneof_serializes_as_its_number() {
    let connector = GuestPortConnectorInfo {
        guest_port_number: GuestPortNumber::Port1 as i32,
        capabilities: Vec::new(),
        connected_device: Some(ConnectedDevice::Error(99)),
    };

    let value =
        serde_json::to_value(&connector).expect("an unknown enum value must still serialize");

    assert_eq!(value["error"], serde_json::json!(99));
}

#[test]
fn one_unknown_device_id_does_not_take_down_the_rest_of_the_message() {
    let info = GuestPortInfo {
        gp1: Some(connector_with(
            GuestPortNumber::Port1,
            vec![GuestPortDevice {
                device_id: UNKNOWN_DEVICE_ID,
                name: "Unknown device".to_string(),
                ..Default::default()
            }],
        )),
        gp2: Some(connector_with(
            GuestPortNumber::Port2,
            vec![GuestPortDevice {
                device_id: GuestPortDeviceId::BlueyeCam as i32,
                name: "Blueye Camera".to_string(),
                ..Default::default()
            }],
        )),
        ..Default::default()
    };

    let value = serde_json::to_value(&info).expect("one unknown device must not fail the message");

    let gp1_device = &value["gp1"]["device_list"]["devices"][0];
    assert_eq!(
        gp1_device["device_id"],
        serde_json::json!(UNKNOWN_DEVICE_ID)
    );
    assert_eq!(gp1_device["name"], serde_json::json!("Unknown device"));

    let gp2_device = &value["gp2"]["device_list"]["devices"][0];
    assert_eq!(
        gp2_device["device_id"],
        serde_json::json!("GUEST_PORT_DEVICE_ID_BLUEYE_CAM")
    );
    assert_eq!(gp2_device["name"], serde_json::json!("Blueye Camera"));
}
