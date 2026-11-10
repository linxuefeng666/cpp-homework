# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target action_sum_cpp::action_sum_cpp
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${action_sum_cpp_TARGETS}.
if(action_sum_cpp_TARGETS AND NOT TARGET action_sum_cpp::action_sum_cpp)
  add_library(action_sum_cpp::action_sum_cpp INTERFACE IMPORTED)
  set_target_properties(action_sum_cpp::action_sum_cpp PROPERTIES
    INTERFACE_LINK_LIBRARIES "${action_sum_cpp_TARGETS}")
endif()
