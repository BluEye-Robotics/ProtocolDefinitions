cmake_minimum_required(VERSION 3.14)
project (blueyeprotocol VERSION 3.0.0)

set(CMAKE_CXX_STANDARD 17)
include(GNUInstallDirs)


find_package(Protobuf REQUIRED)

file(GLOB ProtoFiles "protobuf_definitions/*.proto")
PROTOBUF_GENERATE_CPP(ProtoSources ProtoHeaders ${ProtoFiles})
protobuf_generate_python(PROTO_PY ${ProtoFiles})

add_library(blueyeprotocol SHARED ${ProtoSources} ${ProtoHeaders})

target_link_libraries(blueyeprotocol PUBLIC ${PROTOBUF_LIBRARIES})
target_include_directories(blueyeprotocol PUBLIC ${PROTOBUF_INCLUDE_DIRS})

install(TARGETS blueyeprotocol
        EXPORT blueyeprotocol
        LIBRARY DESTINATION lib)
install(FILES ${ProtoHeaders} DESTINATION include/blueyeprotocol)


# ---------------------------------------------------------------------------------------
# Install cmake config
# ---------------------------------------------------------------------------------------

set(INCLUDE_INSTALL_DIR include/blueyeprotocol )
set(LIB_INSTALL_DIR lib/)

include(CMakePackageConfigHelpers)
configure_package_config_file(cmake/blueyeprotocolConfig.cmake.in
  ${CMAKE_CURRENT_BINARY_DIR}/blueyeprotocolConfig.cmake
  INSTALL_DESTINATION ${LIB_INSTALL_DIR}/blueyeprotocol/cmake
  PATH_VARS INCLUDE_INSTALL_DIR LIB_INSTALL_DIR)
write_basic_package_version_file(
  ${CMAKE_CURRENT_BINARY_DIR}/blueyeprotocolConfigVersion.cmake
  VERSION 1.0.0
  COMPATIBILITY SameMajorVersion )
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/blueyeprotocolConfig.cmake
              ${CMAKE_CURRENT_BINARY_DIR}/blueyeprotocolConfigVersion.cmake
        DESTINATION ${LIB_INSTALL_DIR}/cmake/blueyeprotocol )