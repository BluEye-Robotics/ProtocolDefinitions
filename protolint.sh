#!/bin/bash

docker run --rm \
    -v $(pwd):/workspace \
    --workdir /workspace \
    yoheimuta/protolint:0.44.0 lint protobuf_definitions
