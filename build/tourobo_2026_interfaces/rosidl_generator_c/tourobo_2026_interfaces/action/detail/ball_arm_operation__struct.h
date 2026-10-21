// NOLINT: This file starts with a BOM since it contain non-ASCII characters
// generated from rosidl_generator_c/resource/idl__struct.h.em
// with input from tourobo_2026_interfaces:action/BallArmOperation.idl
// generated code does not contain a copyright notice

#ifndef TOUROBO_2026_INTERFACES__ACTION__DETAIL__BALL_ARM_OPERATION__STRUCT_H_
#define TOUROBO_2026_INTERFACES__ACTION__DETAIL__BALL_ARM_OPERATION__STRUCT_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>


// Constants defined in the message

/// Struct defined in action/BallArmOperation in the package tourobo_2026_interfaces.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_Goal
{
  /// 1:左を操作 2:右を操作
  int32_t target_arm;
  bool is_open;
  int32_t current_state;
  int32_t carry;
  int32_t push_state;
  int32_t shoot_angle_state;
  bool is_left_arm_open;
  bool is_right_arm_open;
} tourobo_2026_interfaces__action__BallArmOperation_Goal;

// Struct for a sequence of tourobo_2026_interfaces__action__BallArmOperation_Goal.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_Goal__Sequence
{
  tourobo_2026_interfaces__action__BallArmOperation_Goal * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} tourobo_2026_interfaces__action__BallArmOperation_Goal__Sequence;


// Constants defined in the message

/// Struct defined in action/BallArmOperation in the package tourobo_2026_interfaces.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_Result
{
  bool success;
  int32_t next_state;
  int32_t next_carry;
  int32_t next_push_state;
  int32_t next_shoot_angle_state;
  bool next_is_left_arm_open;
  bool next_is_right_arm_open;
} tourobo_2026_interfaces__action__BallArmOperation_Result;

// Struct for a sequence of tourobo_2026_interfaces__action__BallArmOperation_Result.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_Result__Sequence
{
  tourobo_2026_interfaces__action__BallArmOperation_Result * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} tourobo_2026_interfaces__action__BallArmOperation_Result__Sequence;


// Constants defined in the message

/// Struct defined in action/BallArmOperation in the package tourobo_2026_interfaces.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_Feedback
{
  uint8_t structure_needs_at_least_one_member;
} tourobo_2026_interfaces__action__BallArmOperation_Feedback;

// Struct for a sequence of tourobo_2026_interfaces__action__BallArmOperation_Feedback.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_Feedback__Sequence
{
  tourobo_2026_interfaces__action__BallArmOperation_Feedback * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} tourobo_2026_interfaces__action__BallArmOperation_Feedback__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'goal'
#include "tourobo_2026_interfaces/action/detail/ball_arm_operation__struct.h"

/// Struct defined in action/BallArmOperation in the package tourobo_2026_interfaces.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
  tourobo_2026_interfaces__action__BallArmOperation_Goal goal;
} tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request;

// Struct for a sequence of tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request__Sequence
{
  tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.h"

/// Struct defined in action/BallArmOperation in the package tourobo_2026_interfaces.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response
{
  bool accepted;
  builtin_interfaces__msg__Time stamp;
} tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response;

// Struct for a sequence of tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response__Sequence
{
  tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"

/// Struct defined in action/BallArmOperation in the package tourobo_2026_interfaces.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request
{
  unique_identifier_msgs__msg__UUID goal_id;
} tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request;

// Struct for a sequence of tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request__Sequence
{
  tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'result'
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_arm_operation__struct.h"

/// Struct defined in action/BallArmOperation in the package tourobo_2026_interfaces.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response
{
  int8_t status;
  tourobo_2026_interfaces__action__BallArmOperation_Result result;
} tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response;

// Struct for a sequence of tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response__Sequence
{
  tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response__Sequence;


// Constants defined in the message

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.h"
// Member 'feedback'
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_arm_operation__struct.h"

/// Struct defined in action/BallArmOperation in the package tourobo_2026_interfaces.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage
{
  unique_identifier_msgs__msg__UUID goal_id;
  tourobo_2026_interfaces__action__BallArmOperation_Feedback feedback;
} tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage;

// Struct for a sequence of tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage.
typedef struct tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage__Sequence
{
  tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage * data;
  /// The number of valid items in data
  size_t size;
  /// The number of allocated items in data
  size_t capacity;
} tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage__Sequence;

#ifdef __cplusplus
}
#endif

#endif  // TOUROBO_2026_INTERFACES__ACTION__DETAIL__BALL_ARM_OPERATION__STRUCT_H_
