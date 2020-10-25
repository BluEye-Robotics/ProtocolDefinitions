if [[ "$(docker images -q yoheimuta/protolint:latest 2> /dev/null)" == "" ]]; then
  echo "Image yoheimuta/protolint:latest not found, run docker pull to install:"
  echo "docker pull yoheimuta/protolint"
else
  docker run --rm \
    -v $(pwd):/workspace \
    --workdir /workspace \
    yoheimuta/protolint:latest lint protobuf_definitions
fi