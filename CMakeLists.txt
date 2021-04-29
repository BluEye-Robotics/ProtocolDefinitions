cmake_minimum_required(VERSION 3.14)
project (blueyeprotocol)

set(CMAKE_CXX_STANDARD 17)

find_package(Protobuf REQUIRED)

file(GLOB ProtoFiles "protobuf_definitions/*.proto")
PROTOBUF_GENERATE_CPP(ProtoSources ProtoHeaders ${ProtoFiles})
protobuf_generate_python(PROTO_PY ${ProtoFiles})

add_library(blueyeprotocol SHARED ${ProtoSources} ${ProtoHeaders})

target_link_libraries(blueyeprotocol PUBLIC ${PROTOBUF_LIBRARIES})
target_include_directories(blueyeprotocol PUBLIC ${PROTOBUF_INCLUDE_DIRS})

install(TARGETS blueyeprotocol LIBRARY DESTINATION lib)
install(FILES ${ProtoHeaders} DESTINATION include/blueyeprotocol)
