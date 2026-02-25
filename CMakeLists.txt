cmake_minimum_required(VERSION 3.14)
project(blueyeprotocol VERSION 3.0.0)

set(CMAKE_CXX_STANDARD 17)
include(GNUInstallDirs)

find_package(Protobuf REQUIRED)
find_package(Python3 COMPONENTS Interpreter REQUIRED)

message(STATUS "Using protobuf ${Protobuf_VERSION}")

# We need to link abseil if protobuf version is 6.x or higher
if(Protobuf_VERSION VERSION_GREATER_EQUAL "6.0.0")
  message(STATUS "Protobuf version is ${Protobuf_VERSION}, linking abseil")
  find_package(absl REQUIRED)
  set(PROTOBUF_LIBRARIES ${PROTOBUF_LIBRARIES} absl::log absl::strings absl::base absl::status absl::log_internal_check_op absl::log_internal_message)
endif()

file(GLOB ProtoFiles "protobuf_definitions/*.proto")

foreach(ProtoFile ${ProtoFiles})
  message(STATUS ${ProtoFile})
endforeach()

# If cmake complains "target pattern contains no '%'", install protoc (protobuf-compiler)
PROTOBUF_GENERATE_CPP(ProtoSources ProtoHeaders ${ProtoFiles})
protobuf_generate_python(PROTO_PY ${ProtoFiles})
add_library(blueyeprotocol SHARED ${ProtoSources} ${ProtoHeaders})

set(EXT_LIBS
  ${PROTOBUF_LIBRARIES}
)

target_link_libraries(blueyeprotocol PUBLIC ${EXT_LIBS})
target_include_directories(blueyeprotocol PUBLIC ${PROTOBUF_INCLUDE_DIRS})

install(TARGETS blueyeprotocol
  EXPORT blueyeprotocol
  LIBRARY DESTINATION lib)
install(FILES ${ProtoHeaders} DESTINATION include/blueyeprotocol)
install(FILES ${ProtoFiles} DESTINATION include/blueyeprotocol/protobuf)

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
