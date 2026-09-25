use std::path::PathBuf;

use prost::Message as _;
use prost_types::{DescriptorProto, FileDescriptorSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = PathBuf::from("../protobuf_definitions");
    // Watch the directory so added/removed .proto files retrigger the build.
    println!("cargo:rerun-if-changed=../protobuf_definitions");

    let entries = std::fs::read_dir(&proto_dir)
        .map_err(|e| format!("failed to read {}: {e}", proto_dir.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("failed to read entry in {}: {e}", proto_dir.display()))?;
    let mut protos: Vec<PathBuf> = entries
        .into_iter()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "proto"))
        .collect();
    protos.sort();

    // Watch each file individually so edits to an existing .proto retrigger codegen
    // (directory mtime alone doesn't change when a file inside it is edited).
    for proto in &protos {
        println!("cargo:rerun-if-changed={}", proto.display());
    }

    let file_descriptor_set = protox::compile(&protos, [&proto_dir])?;

    // Persist the descriptor set so the crate can expose it for runtime
    // reflection (decoding google.protobuf.Any payloads by type name).
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let descriptor_set_bytes = file_descriptor_set.encode_to_vec();
    std::fs::write(out_dir.join("file_descriptor_set.bin"), &descriptor_set_bytes)?;

    let mut config = prost_build::Config::new();
    // Generate `impl prost::Name` so google.protobuf.Any type-URLs resolve.
    config.enable_type_names();
    config.type_name_domain(["."], "type.googleapis.com");

    // Derive prost-reflect's ReflectMessage on every blueye.protocol message so
    // consumers can reflect on them and look them up in DESCRIPTORS. This mirrors
    // what prost-reflect-build injects, but sourced from our protox descriptor
    // set instead of a protoc invocation.
    for full_name in message_full_names(&file_descriptor_set) {
        config.message_attribute(
            &full_name,
            format!(
                "#[derive(::prost_reflect::ReflectMessage)]\n\
                 #[prost_reflect(message_name = \"{full_name}\", \
                 file_descriptor_set_bytes = \"crate::FILE_DESCRIPTOR_SET\")]"
            ),
        );
    }

    config.compile_fds(file_descriptor_set)?;

    // With the `serde` feature, generate protobuf-JSON Serialize/Deserialize
    // impls matching C++ MessageToJsonString conventions (snake_case keys,
    // defaults emitted, unknown fields ignored on read, and -- via
    // render_unknown_enums_as_numbers below -- out-of-range enum values written
    // as their number rather than failing the message) for the persistent
    // storage settings file and the guest-port info JSON. Scoped to those
    // message closures rather than the whole package, which would also cover
    // well-known types that prost-types has no serde impls for.
    if std::env::var_os("CARGO_FEATURE_SERDE").is_some() {
        pbjson_build::Builder::new()
            .register_descriptors(&descriptor_set_bytes)?
            .preserve_proto_field_names()
            .emit_fields()
            .ignore_unknown_fields()
            .build(&[
                ".blueye.protocol.PersistentStorageSettings",
                ".blueye.protocol.GuestPortInfo",
                ".blueye.protocol.GuestPortConnectorInfo",
                ".blueye.protocol.GuestPortDeviceList",
                ".blueye.protocol.GuestPortDevice",
                ".blueye.protocol.GuestPortDeviceID",
                ".blueye.protocol.GuestPortNumber",
                ".blueye.protocol.GuestPortCapability",
                ".blueye.protocol.GuestPortDetachStatus",
                ".blueye.protocol.GuestPortError",
            ])?;
        render_unknown_enums_as_numbers(&out_dir)?;
    }
    Ok(())
}

/// Rewrites the pbjson-generated serializers in `out_dir` so an enum value
/// outside the generated enum renders as its bare number instead of failing.
///
/// pbjson emits `Enum::try_from(v).map_err(|_| ..custom("Invalid variant ..")))?`
/// for every enum field. Because serde serialization is all-or-nothing, one
/// unrecognised value fails the entire message -- a guest-port EEPROM flashed
/// with a device id newer than the firmware reading it would cost the reader
/// every port. C++ `MessageToJsonString`, whose conventions these impls exist to
/// match, writes the number and carries on. Each site is redirected to
/// [`crate::json_enum::enum_or_int`], which does the same.
///
/// Errors if any such site survives the rewrite, so a pbjson upgrade that
/// changes the generated shape fails the build instead of silently restoring
/// the whole-message failure.
fn render_unknown_enums_as_numbers(
    out_dir: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // A path to an enum type as the generated code spells it (`GuestPortNumber`,
    // or `parent_message::NestedEnum` for a nested one).
    const ENUM_PATH: &str = r"[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*";
    // The `.map_err(..)` tail pbjson appends to every `try_from`; the value it
    // formats is a plain expression (`self.device_id`, `*v`), never a call.
    const MAP_ERR: &str = r#"\s*\.map_err\(\|_\| serde::ser::Error::custom\(format!\("Invalid variant \{\}", [^()]*\)\)\)"#;

    // Repeated enum field: a fallible map over the values, collected into a
    // Result. Rewritten first -- it contains a `try_from` site of its own that
    // the singular pattern below would otherwise match in isolation.
    let repeated = regex::Regex::new(&format!(
        r"(?s)(self\.\w+)\.iter\(\)\.cloned\(\)\.map\(\|v\| \{{\s*({ENUM_PATH})::try_from\(v\){MAP_ERR}\s*\}}\)\.collect::<std::result::Result<Vec<_>, _>>\(\)\?"
    ))?;
    // Singular enum field, in a plain field or a oneof arm.
    let singular = regex::Regex::new(&format!(
        r"(?s)({ENUM_PATH})::try_from\(([^()]+)\){MAP_ERR}\?"
    ))?;

    for entry in std::fs::read_dir(out_dir)? {
        let path = entry?.path();
        if !path.to_string_lossy().ends_with(".serde.rs") {
            continue;
        }
        let generated = std::fs::read_to_string(&path)?;
        let rewritten = repeated.replace_all(
            &generated,
            "${1}.iter().cloned().map(crate::json_enum::enum_or_int::<${2}>).collect::<Vec<_>>()",
        );
        let rewritten =
            singular.replace_all(&rewritten, "crate::json_enum::enum_or_int::<${1}>(${2})");

        if rewritten.contains("Invalid variant") {
            return Err(format!(
                "{}: pbjson still rejects unknown enum values after the rewrite -- \
                 its generated shape has changed, update render_unknown_enums_as_numbers",
                path.display()
            )
            .into());
        }
        std::fs::write(&path, rewritten.as_ref())?;
    }
    Ok(())
}

/// Fully-qualified names of every message in the `blueye.protocol` package,
/// including nested messages (e.g. `blueye.protocol.Parent.Nested`). Names use
/// no leading dot, matching what prost-reflect-build passes to prost-build's
/// path matcher.
fn message_full_names(fds: &FileDescriptorSet) -> Vec<String> {
    let mut names = Vec::new();
    for file in &fds.file {
        // Skip well-known-type files (google.protobuf.*): prost maps those to
        // prost-types and generates no Rust for them, so an attribute on them
        // would match nothing.
        if file.package() != "blueye.protocol" {
            continue;
        }
        for message in &file.message_type {
            collect_message_names(file.package(), message, &mut names);
        }
    }
    names
}

fn collect_message_names(prefix: &str, message: &DescriptorProto, names: &mut Vec<String>) {
    let full_name = format!("{prefix}.{}", message.name());
    for nested in &message.nested_type {
        collect_message_names(&full_name, nested, names);
    }
    names.push(full_name);
}
