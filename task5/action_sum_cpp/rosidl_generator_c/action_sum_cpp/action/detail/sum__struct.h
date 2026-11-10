// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from action_sum_cpp:action/Sum.idl
// generated code does not contain a copyright notice

#ifndef ACTION_SUM_CPP__ACTION__DETAIL__SUM__STRUCT_H_
#define ACTION_SUM_CPP__ACTION__DETAIL__SUM__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in action/Sum in the package action_sum_cpp.
typedef struct action_sum_cpp__action__Sum_Goal
{
  int32_t n;
} action_sum_cpp__action__Sum_Goal;

// Struct for a sequence of action_sum_cpp__action__Sum_Goal.
typedef struct action_sum_cpp__action__Sum_Goal__Sequence
{
  action_sum_cpp__action__Sum_Goal * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_sum_cpp__action__Sum_Goal__Sequence;


// Constants defined in the message

/// Struct defined in action/Sum in the package action_sum_cpp.
typedef struct action_sum_cpp__action__Sum_Result
{
  int32_t total_sum;
} action_sum_cpp__action__Sum_Result;

// Struct for a sequence of action_sum_cpp__action__Sum_Result.
typedef struct action_sum_cpp__action__Sum_Result__Sequence
{
  action_sum_cpp__action__Sum_Result * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_sum_cpp__action__Sum_Result__Sequence;


// Constants defined in the message

/// Struct defined in action/Sum in the package action_sum_cpp.
typedef struct action_sum_cpp__action__Sum_Feedback
{
  int32_t current_num;
  int32_t current_sum;
} action_sum_cpp__action__Sum_Feedback;

// Struct for a sequence of action_sum_cpp__action__Sum_Feedback.
typedef struct action_sum_cpp__action__Sum_Feedback__Sequence
{
  action_sum_cpp__action__Sum_Feedback * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_sum_cpp__action__Sum_Feedback__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'goal'
#include "action_sum_cpp/action/detail/sum__struct.h"

/// Struct defined in action/Sum in the package action_sum_cpp.
typedef struct action_sum_cpp__action__Sum_SendGoal_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
  action_sum_cpp__action__Sum_Goal goal;
} action_sum_cpp__action__Sum_SendGoal_Request;

// Struct for a sequence of action_sum_cpp__action__Sum_SendGoal_Request.
typedef struct action_sum_cpp__action__Sum_SendGoal_Request__Sequence
{
  action_sum_cpp__action__Sum_SendGoal_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_sum_cpp__action__Sum_SendGoal_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.h"

/// Struct defined in action/Sum in the package action_sum_cpp.
typedef struct action_sum_cpp__action__Sum_SendGoal_Response
{
  bool accepted;
  builtin_interfaces__msg__Time stamp;
} action_sum_cpp__action__Sum_SendGoal_Response;

// Struct for a sequence of action_sum_cpp__action__Sum_SendGoal_Response.
typedef struct action_sum_cpp__action__Sum_SendGoal_Response__Sequence
{
  action_sum_cpp__action__Sum_SendGoal_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_sum_cpp__action__Sum_SendGoal_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"

/// Struct defined in action/Sum in the package action_sum_cpp.
typedef struct action_sum_cpp__action__Sum_GetResult_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
} action_sum_cpp__action__Sum_GetResult_Request;

// Struct for a sequence of action_sum_cpp__action__Sum_GetResult_Request.
typedef struct action_sum_cpp__action__Sum_GetResult_Request__Sequence
{
  action_sum_cpp__action__Sum_GetResult_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_sum_cpp__action__Sum_GetResult_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'result'
// already included above
// #include "action_sum_cpp/action/detail/sum__struct.h"

/// Struct defined in action/Sum in the package action_sum_cpp.
typedef struct action_sum_cpp__action__Sum_GetResult_Response
{
  int8_t status;
  action_sum_cpp__action__Sum_Result result;
} action_sum_cpp__action__Sum_GetResult_Response;

// Struct for a sequence of action_sum_cpp__action__Sum_GetResult_Response.
typedef struct action_sum_cpp__action__Sum_GetResult_Response__Sequence
{
  action_sum_cpp__action__Sum_GetResult_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_sum_cpp__action__Sum_GetResult_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'feedback'
// already included above
// #include "action_sum_cpp/action/detail/sum__struct.h"

/// Struct defined in action/Sum in the package action_sum_cpp.
typedef struct action_sum_cpp__action__Sum_FeedbackMessage
{
  unique_identifier_msgs__msg__UUID goal_id;
  action_sum_cpp__action__Sum_Feedback feedback;
} action_sum_cpp__action__Sum_FeedbackMessage;

// Struct for a sequence of action_sum_cpp__action__Sum_FeedbackMessage.
typedef struct action_sum_cpp__action__Sum_FeedbackMessage__Sequence
{
  action_sum_cpp__action__Sum_FeedbackMessage * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} action_sum_cpp__action__Sum_FeedbackMessage__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // ACTION_SUM_CPP__ACTION__DETAIL__SUM__STRUCT_H_
