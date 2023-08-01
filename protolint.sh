#!/bin/bash

docker run --rm \
    -v $(pwd):/workspace \
    --workdir /workspace \
    yoheimuta/protolint lint protobuf_definitions