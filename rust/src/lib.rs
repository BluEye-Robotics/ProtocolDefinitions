//! Rust bindings for the Blueye Robotics protocol definitions.
//!
//! Generated at build time from the repository's `protobuf_definitions/*.proto`
//! files via [`prost`]. All messages live in the `blueye.protocol` proto
//! package and are re-exported flat from the crate root.

mod generated {
    include!(concat!(env!("OUT_DIR"), "/blueye.protocol.rs"));
}

pub use generated::*;

/// Re-export of the [`prost::Message`] trait for encoding/decoding.
pub use prost::Message;
/// Re-export of well-known types (`Timestamp`, `Duration`, `Any`).
pub use prost_types;
