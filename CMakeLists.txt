cmake_minimum_required(VERSION 3.27)
project(blueyeprotocol VERSION 3.0.0)

set(CMAKE_CXX_STANDARD 20)
include(GNUInstallDirs)
find_package(protobuf CONFIG REQUIRED)

file(GLOB ProtoFiles "protobuf_definitions/*.proto")

add_library(blueyeprotocol SHARED "${ProtoFiles}")

target_link_libraries(blueyeprotocol PUBLIC protobuf::libprotobuf)

target_include_directories(blueyeprotocol PUBLIC PROTOBUF_INCLUDE_DIRS)

protobuf_generate(
  LANGUAGE cpp
  TARGET blueyeprotocol
  IMPORT_DIRS "${CMAKE_CURRENT_LIST_DIR}/protobuf_definitions"
  PROTOC_OUT_DIR "${PROTO_BINARY_DIR}"
  OUT_VAR "ProtoSources"
)

protobuf_generate(
  LANGUAGE Python
  TARGET blueyeprotocol
  IMPORT_DIRS "${CMAKE_CURRENT_LIST_DIR}/protobuf_definitions"
  PROTOC_OUT_DIR "${PROTO_BINARY_DIR}")

install(TARGETS blueyeprotocol
  EXPORT blueyeprotocol
  LIBRARY DESTINATION lib)

# Filter only header files
list(FILTER ProtoSources INCLUDE REGEX "\\.h$")

install(FILES ${ProtoSources} DESTINATION include/blueyeprotocol)

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

# debug
function(dump_cmake_variables)
  get_cmake_property(_variableNames VARIABLES)
  list(SORT _variableNames)

  foreach(_variableName ${_variableNames})
    if(ARGV0)
      unset(MATCHED)

      # case sensitive match
      # string(REGEX MATCH ${ARGV0} MATCHED ${_variableName})
      #
      # case insenstitive match
      string(TOLOWER "${ARGV0}" ARGV0_lower)
      string(TOLOWER "${_variableName}" _variableName_lower)
      string(REGEX MATCH ${ARGV0_lower} MATCHED ${_variableName_lower})

      if(NOT MATCHED)
        continue()
      endif()
    endif()

    message(STATUS "${_variableName}=${${_variableName}}")
  endforeach()
endfunction()

dump_cmake_variables("ProtoSources")