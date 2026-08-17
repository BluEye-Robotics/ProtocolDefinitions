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
    // defaults emitted, unknown fields ignored on read) for the persistent
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
