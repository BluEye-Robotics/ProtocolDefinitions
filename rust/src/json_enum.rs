//! Protobuf-JSON rendering of enum values that fall outside the generated enum.
//!
//! `pbjson`'s generated serializers reject any enum value they cannot name, and
//! serde serialization is all-or-nothing, so a single unrecognised value fails
//! the *whole* message. That bites on the guest port: a connector EEPROM can be
//! flashed with a device id newer than the firmware reading it (a Water Linked
//! DVL A100 read by a build that predates that id), and the drone would lose
//! every port's info rather than the one unknown device.
//!
//! C++ `MessageToJsonString` instead writes the bare number and carries on, and
//! the consumers of this JSON are written against that (p2-django strips
//! integer `device_id`s from the guest-port info it serves). `build.rs` rewrites
//! the generated `try_from(..).map_err(..)?` sites to go through
//! [`enum_or_int`], so the two agree.

use serde::{Serialize, Serializer};

/// A protobuf enum field value on the way out to JSON: either a variant this
/// build knows, serialized as its declared name by the generated `Serialize`
/// impl, or a raw number it does not, serialized as that number.
pub(crate) enum EnumOrInt<T> {
    Known(T),
    Unknown(i32),
}

impl<T: Serialize> Serialize for EnumOrInt<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Known(value) => value.serialize(serializer),
            Self::Unknown(value) => serializer.serialize_i32(*value),
        }
    }
}

/// Wraps a raw protobuf enum value for serialization: [`EnumOrInt::Known`] when
/// `T` recognises it, [`EnumOrInt::Unknown`] otherwise.
pub(crate) fn enum_or_int<T: TryFrom<i32>>(value: i32) -> EnumOrInt<T> {
    match T::try_from(value) {
        Ok(known) => EnumOrInt::Known(known),
        Err(_) => EnumOrInt::Unknown(value),
    }
}
