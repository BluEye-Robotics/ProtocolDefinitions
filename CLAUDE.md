# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Protocol definition repository for Blueye Robotics underwater drones. The core definitions live in `protobuf_definitions/*.proto` (proto3 syntax) and are compiled into language-specific libraries for C++, Python, TypeScript, and C#/.NET.

Published packages:
- **npm**: `@blueyerobotics/protocol-definitions`
- **NuGet**: `Blueye.Protocol.Protobuf` (GitHub Packages)
- **Python**: `blueye.protocol` (PyPI, via separate repo triggered by CI)

## Build Commands

### C++ (CMake)
```
cmake -B build && cmake --build build
cmake --install build --prefix /usr/local
```

### TypeScript
```
pnpm install
# Generate TypeScript from proto files:
mkdir -p ./out/ && protoc protobuf_definitions/*.proto \
  --plugin=./node_modules/.bin/protoc-gen-ts_proto \
  --proto_path=protobuf_definitions \
  --ts_proto_out=./out \
  --ts_proto_opt=outputIndex=true \
  --ts_proto_opt=globalThisPolyfill=true \
  --ts_proto_opt=useExactTypes=false
# Compile TypeScript:
pnpm run build
```

### C#/.NET
```
dotnet pack Blueye.Protocol.Protobuf.csproj -c Release -o out
```

### Linting
```
# Via Docker:
./protolint.sh
```
Protolint runs on every push/PR in CI. Configuration in `.protolint.yaml` (max line length: 120 chars).

### Documentation Generation
```
./protoc-gen-doc.sh
```

## Proto File Structure

All proto definitions are in `protobuf_definitions/`:
- **message_formats.proto** — Main message types, enums, and parameter definitions (largest file)
- **telemetry.proto** — Telemetry message definitions
- **control.proto** — Control command messages
- **req_rep.proto** — Request/response message patterns
- **mission_planning.proto** — Mission planning messages
- **aquatroll.proto** — AquaTroll sensor-specific messages

## CI/CD Pipeline

- **ci-build.yaml** — Protolint validation (all pushes and PRs)
- **ci-typescript.yaml** — Generate TS, compile, publish to npm (master only)
- **ci-dotnet.yaml** — Build and publish NuGet package (all pushes)
- **ci-python.yaml** — Triggers `blueye.protocol` repo update via repository dispatch (master, when proto files change)
- **gen-docs.yaml** — Generate HTML docs, upload to Azure (master only)

## Code Style

- 2-space indentation, trim trailing whitespace (`.editorconfig`)
- Proto files: max 120 character line length (`.protolint.yaml`)
