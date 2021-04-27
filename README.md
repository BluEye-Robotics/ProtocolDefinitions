# ProtocolDefinitions

[![CI build](https://github.com/BluEye-Robotics/ProtocolDefinitions/actions/workflows/ci-build.yaml/badge.svg)](https://github.com/BluEye-Robotics/ProtocolDefinitions/actions/workflows/ci-build.yaml)

This repository contains protocol definitions for interacting with the Blueye Pioneer and Blueye Pro underwater drone.

## Protocol v2
See [blueye.protocol](https://github.com/BluEye-Robotics/blueye.protocol) and
[blueye.sdk](https://github.com/BluEye-Robotics/blueye.sdk) for Python libraries that
are built on these definitions and simplifies their use.

## Protocol v3
Version 3 of the Blueye communication protocol is based on [Protocol Buffers](https://developers.google.com/protocol-buffers).

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
