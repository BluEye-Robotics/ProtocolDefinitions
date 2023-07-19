#!/bin/bash

echo "Generating Markdown documentation $(pwd)/docs/markdown/protocol.md"
docker run --rm \
  -v $(pwd)/docs/markdown:/out \
  -v $(pwd)/protobuf_definitions:/protos \
  pseudomuto/protoc-gen-doc --doc_opt=markdown,protocol.md

echo "Generating HTML documentation $(pwd)/docs/html/protocol.html"
docker run --rm \
  -v $(pwd)/docs/html:/out \
  -v $(pwd)/protobuf_definitions:/protos \
  pseudomuto/protoc-gen-doc --doc_opt=html,protocol.html
