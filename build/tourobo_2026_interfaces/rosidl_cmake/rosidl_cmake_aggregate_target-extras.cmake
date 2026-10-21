# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target tourobo_2026_interfaces::tourobo_2026_interfaces
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${tourobo_2026_interfaces_TARGETS}.
if(tourobo_2026_interfaces_TARGETS AND NOT TARGET tourobo_2026_interfaces::tourobo_2026_interfaces)
  add_library(tourobo_2026_interfaces::tourobo_2026_interfaces INTERFACE IMPORTED)
  set_target_properties(tourobo_2026_interfaces::tourobo_2026_interfaces PROPERTIES
    INTERFACE_LINK_LIBRARIES "${tourobo_2026_interfaces_TARGETS}")
endif()
