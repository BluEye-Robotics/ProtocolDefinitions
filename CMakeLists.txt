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
set(project_config_in "${CMAKE_CURRENT_LIST_DIR}/cmake/blueyeprotocolConfig.cmake.in")
set(project_config_out "${CMAKE_CURRENT_BINARY_DIR}/blueyeprotocolConfig.cmake")
set(config_targets_file "blueyeprotocolConfigTargets.cmake")
set(version_config_file "${CMAKE_CURRENT_BINARY_DIR}/blueyeprotocolConfigVersion.cmake")
set(export_dest_dir "${CMAKE_INSTALL_LIBDIR}/cmake/blueyeprotocol")
install(EXPORT blueyeprotocol DESTINATION ${export_dest_dir} NAMESPACE blueyeprotocol:: FILE ${config_targets_file})

include(CMakePackageConfigHelpers)
configure_file("${project_config_in}" "${project_config_out}" @ONLY)

write_basic_package_version_file("${version_config_file}" COMPATIBILITY SameMajorVersion)
install(FILES "${project_config_out}" "${version_config_file}" DESTINATION "${export_dest_dir}")
