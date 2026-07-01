use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = PathBuf::from("../protobuf_definitions");
    println!("cargo:rerun-if-changed=../protobuf_definitions");

    let mut protos: Vec<PathBuf> = std::fs::read_dir(&proto_dir)
        .map_err(|e| format!("failed to read {}: {e}", proto_dir.display()))?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "proto"))
        .collect();
    protos.sort();

    let file_descriptor_set = protox::compile(&protos, [&proto_dir])?;
    prost_build::Config::new().compile_fds(file_descriptor_set)?;
    Ok(())
}
