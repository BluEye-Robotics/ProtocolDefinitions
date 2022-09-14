# ProtocolDefinitions

[![CI build](https://github.com/BluEye-Robotics/ProtocolDefinitions/actions/workflows/ci-build.yaml/badge.svg)](https://github.com/BluEye-Robotics/ProtocolDefinitions/actions/workflows/ci-build.yaml)

This repository contains protocol definitions for interacting with the Blueye underwater drones.

## Protocol v2
Drones running a Blunux version \< 3.0 will need to use the legacy protocol. [blueye.legacyprotocol](https://github.com/BluEye-Robotics/blueye.legacyprotocol) contains a Python package that is built on this protocol and simplifies its use.

## Protocol v3
Version 3 of the Blueye communication protocol is based on [Protocol Buffers](https://developers.google.com/protocol-buffers).

The protobuf definitions are compiled to language specific libraries, and are available as a [NuGet package](https://github.com/BluEye-Robotics/ProtocolDefinitions/packages/1239508) and a [Python package](https://pypi.org/project/blueye.protocol/).

Automatically generated documentation for the protocol format can be found at https://blueyebuildserver.blob.core.windows.net/protocoldefinitions/docs/protocol.html.

This documentation will be merged with the `blueye.sdk` repository when v3 is finalized.

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
