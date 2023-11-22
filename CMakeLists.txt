cmake_minimum_required(VERSION 3.14)
project(blueyeprotocol VERSION 3.0.0)

set(CMAKE_CXX_STANDARD 17)
include(GNUInstallDirs)

find_package(Protobuf REQUIRED)
find_package(Python3 COMPONENTS Interpreter REQUIRED)

file(GLOB ProtoFiles "protobuf_definitions/*.proto")
PROTOBUF_GENERATE_CPP(ProtoSources ProtoHeaders ${ProtoFiles})
protobuf_generate_python(PROTO_PY ${ProtoFiles})

add_library(blueyeprotocol SHARED ${ProtoSources} ${ProtoHeaders})

set(EXT_LIBS
  ${PROTOBUF_LIBRARIES}
)

# For protobuf linking. Should rather user find_package(protobuf CONFIG REQUIRED) but that fails in yocto
find_package(absl QUIET)

if(absl_FOUND)
  list(APPEND EXT_LIBS
    absl::log_internal_message
    absl::log_internal_check_op
  )
endif()

target_link_libraries(blueyeprotocol PUBLIC ${EXT_LIBS})
target_include_directories(blueyeprotocol PUBLIC ${PROTOBUF_INCLUDE_DIRS})

install(TARGETS blueyeprotocol
  EXPORT blueyeprotocol
  LIBRARY DESTINATION lib)
install(FILES ${ProtoHeaders} DESTINATION include/blueyeprotocol)

add_custom_target(pyblueyeprotocol
  DEPENDS ${PROTO_PY})

install(FILES ${PROTO_PY} DESTINATION ${Python3_SITELIB} OPTIONAL)
add_custom_target(python_bindings)
add_dependencies(python_bindings pyblueyeprotocol)

# ---------------------------------------------------------------------------------------
# Install cmake config
# ---------------------------------------------------------------------------------------
set(INCLUDE_INSTALL_DIR include)
set(LIB_INSTALL_DIR lib/)

include(CMakePackageConfigHelpers)
configure_package_config_file(cmake/blueyeprotocolConfig.cmake.in
  ${CMAKE_CURRENT_BINARY_DIR}/blueyeprotocolConfig.cmake
  INSTALL_DESTINATION ${LIB_INSTALL_DIR}/blueyeprotocol/cmake
  PATH_VARS INCLUDE_INSTALL_DIR LIB_INSTALL_DIR)
write_basic_package_version_file(
  ${CMAKE_CURRENT_BINARY_DIR}/blueyeprotocolConfigVersion.cmake
  VERSION 1.0.0
  COMPATIBILITY SameMajorVersion)
install(FILES ${CMAKE_CURRENT_BINARY_DIR}/blueyeprotocolConfig.cmake
  ${CMAKE_CURRENT_BINARY_DIR}/blueyeprotocolConfigVersion.cmake
  DESTINATION ${LIB_INSTALL_DIR}/cmake/blueyeprotocol)
