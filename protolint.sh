#!/bin/bash

if [[ "$(docker images -q yoheimuta/protolint:v0.31.0 2> /dev/null)" == "" ]]; then
  echo "Image yoheimuta/protolint:v0.31.0 not found, run docker pull to install:"
  echo "docker pull yoheimuta/protolint:v0.31.0"
else
  docker run --rm \
    -v $(pwd):/workspace \
    --workdir /workspace \
    yoheimuta/protolint:v0.31.0 lint protobuf_definitions
fi
