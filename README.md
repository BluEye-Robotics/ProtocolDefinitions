# ProtocolDefinitions

[![CI build](https://github.com/BluEye-Robotics/ProtocolDefinitions/actions/workflows/ci-build.yaml/badge.svg)](https://github.com/BluEye-Robotics/ProtocolDefinitions/actions/workflows/ci-build.yaml)

This repository contains protocol definitions for interacting with the Blueye underwater drones.

## Protocol v2

Drones running a Blunux version \< 3.0 will need to use the legacy protocol. [blueye.legacyprotocol](https://github.com/BluEye-Robotics/blueye.legacyprotocol) contains a Python package that is built on this protocol and simplifies its use.

## Protocol v3

Version 3 of the Blueye communication protocol is based on [Protocol Buffers](https://developers.google.com/protocol-buffers).

The protobuf definitions are compiled to language specific libraries, and are available as a [NuGet package](https://github.com/BluEye-Robotics/ProtocolDefinitions/packages/1239508), [Python package](https://pypi.org/project/blueye.protocol/) and a [npm package](https://www.npmjs.com/package/@blueyerobotics/protocol-definitions). Swift sources are maintained in [blueye.protocol.swift](https://github.com/BluEye-Robotics/blueye.protocol.swift). This repository also builds a [Kotlin/JVM Maven package](kotlin/README.md) and a Rust crate (see below).

Automatically generated documentation for the protocol format can be found [here](https://blueyebuildserver.blob.core.windows.net/protocoldefinitions/docs/protocol.html).

### Swift and Kotlin

The Swift package exposes `BlueyeProtocol` through Swift Package Manager from
the separate [blueye.protocol.swift repository](https://github.com/BluEye-Robotics/blueye.protocol.swift).
Like Python, it pins this repository as a submodule and commits generated
sources in its own repository. Swift tags use the same version as NuGet and Maven.

The Kotlin/JVM library publishes to GitHub Packages as
`com.blueyerobotics:protocol-definitions`, with the full Protobuf runtime and
generated Java classes plus Kotlin DSLs. See [Kotlin setup and publishing](kotlin/README.md).

All three packages use `protobuf_definitions/` as their source of truth. No
generated Java/Kotlin sources are committed here. Package versions are separate
from the wire protocol version described above.

`version.txt` holds the Blueye App major/minor version, currently `6.5`. The
**Protocol packages** workflow (`ci-dotnet.yaml`) computes one version for every
package using its existing run counter:

- Master builds: stable `6.5.<run-number>` releases.
- Other branches and pull requests: `6.5.<run-number>-branch-<sanitized-branch>`.
- Local builds: `6.5.0-local` for Maven and `6.5.0` for .NET unless overridden.

Merging to master automatically creates the next stable package version. Update
`version.txt` only when the Blueye App major/minor version changes. NuGet and Maven
publish on upstream branch pushes; Swift receives master releases after both
publish successfully. Its receiver generates and validates the pinned schema,
then commits and tags it with the same version. Python keeps its separate
repository and release versioning. Do not rename or recreate the coordinating
workflow without preserving a monotonically increasing build counter.

Once the Swift receiver is installed, set the Actions variable
`SWIFT_PROTOCOL_REPOSITORY` to `BluEye-Robotics/blueye.protocol.swift`. The existing
`CI_PAT` secret must have permission to dispatch events to that repository.
`ci-swift.yaml` receives the shared version and sends it with the exact protocol
SHA. Dispatch is skipped until the variable is configured.

### Rust

The `rust/` directory contains the `blueye-protocol` crate. Code is generated at build time from `protobuf_definitions/*.proto` using [prost](https://crates.io/crates/prost) and [protox](https://crates.io/crates/protox) — no `protoc` installation required.

```sh
cargo build --manifest-path rust/Cargo.toml
cargo test --manifest-path rust/Cargo.toml
```

Use it from another project as a git dependency:

```toml
[dependencies]
blueye-protocol = { git = "https://github.com/BluEye-Robotics/ProtocolDefinitions" }
```

### Installation

#### OS X

```
make
make install PREFIX=/usr/local
```

### Uninstall

#### OS X

```
make uninstall PREFIX=/usr/local
```
