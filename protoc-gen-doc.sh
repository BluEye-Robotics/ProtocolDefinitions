if [[ "$(docker images -q pseudomuto/protoc-gen-doc:latest 2> /dev/null)" == "" ]]; then
  echo "Image pseudomuto/protoc-gen-doc:latest not found, run docker pull to install:"
  echo "docker pull pseudomuto/protoc-gen-doc"
else 
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
fi