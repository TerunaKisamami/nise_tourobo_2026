// generated from rosidl_generator_cpp/resource/idl__traits.hpp.em
// with input from tourobo_2026_interfaces:action/BallPutGate.idl
// generated code does not contain a copyright notice

#ifndef TOUROBO_2026_INTERFACES__ACTION__DETAIL__BALL_PUT_GATE__TRAITS_HPP_
#define TOUROBO_2026_INTERFACES__ACTION__DETAIL__BALL_PUT_GATE__TRAITS_HPP_

#include <stdint.h>

#include <sstream>
#include <string>
#include <type_traits>

#include "tourobo_2026_interfaces/action/detail/ball_put_gate__struct.hpp"
#include "rosidl_runtime_cpp/traits.hpp"

namespace tourobo_2026_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const BallPutGate_Goal & msg,
  std::ostream & out)
{
  out << "{";
  // member: execute
  {
    out << "execute: ";
    rosidl_generator_traits::value_to_yaml(msg.execute, out);
    out << ", ";
  }

  // member: current_state
  {
    out << "current_state: ";
    rosidl_generator_traits::value_to_yaml(msg.current_state, out);
    out << ", ";
  }

  // member: carry
  {
    out << "carry: ";
    rosidl_generator_traits::value_to_yaml(msg.carry, out);
    out << ", ";
  }

  // member: push_state
  {
    out << "push_state: ";
    rosidl_generator_traits::value_to_yaml(msg.push_state, out);
    out << ", ";
  }

  // member: shoot_angle_state
  {
    out << "shoot_angle_state: ";
    rosidl_generator_traits::value_to_yaml(msg.shoot_angle_state, out);
    out << ", ";
  }

  // member: is_left_arm_open
  {
    out << "is_left_arm_open: ";
    rosidl_generator_traits::value_to_yaml(msg.is_left_arm_open, out);
    out << ", ";
  }

  // member: is_right_arm_open
  {
    out << "is_right_arm_open: ";
    rosidl_generator_traits::value_to_yaml(msg.is_right_arm_open, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const BallPutGate_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: execute
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "execute: ";
    rosidl_generator_traits::value_to_yaml(msg.execute, out);
    out << "\n";
  }

  // member: current_state
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "current_state: ";
    rosidl_generator_traits::value_to_yaml(msg.current_state, out);
    out << "\n";
  }

  // member: carry
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "carry: ";
    rosidl_generator_traits::value_to_yaml(msg.carry, out);
    out << "\n";
  }

  // member: push_state
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "push_state: ";
    rosidl_generator_traits::value_to_yaml(msg.push_state, out);
    out << "\n";
  }

  // member: shoot_angle_state
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "shoot_angle_state: ";
    rosidl_generator_traits::value_to_yaml(msg.shoot_angle_state, out);
    out << "\n";
  }

  // member: is_left_arm_open
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "is_left_arm_open: ";
    rosidl_generator_traits::value_to_yaml(msg.is_left_arm_open, out);
    out << "\n";
  }

  // member: is_right_arm_open
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "is_right_arm_open: ";
    rosidl_generator_traits::value_to_yaml(msg.is_right_arm_open, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const BallPutGate_Goal & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use tourobo_2026_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const tourobo_2026_interfaces::action::BallPutGate_Goal & msg,
  std::ostream & out, size_t indentation = 0)
{
  tourobo_2026_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use tourobo_2026_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const tourobo_2026_interfaces::action::BallPutGate_Goal & msg)
{
  return tourobo_2026_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<tourobo_2026_interfaces::action::BallPutGate_Goal>()
{
  return "tourobo_2026_interfaces::action::BallPutGate_Goal";
}

template<>
inline const char * name<tourobo_2026_interfaces::action::BallPutGate_Goal>()
{
  return "tourobo_2026_interfaces/action/BallPutGate_Goal";
}

template<>
struct has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_Goal>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_Goal>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<tourobo_2026_interfaces::action::BallPutGate_Goal>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace tourobo_2026_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const BallPutGate_Result & msg,
  std::ostream & out)
{
  out << "{";
  // member: success
  {
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
    out << ", ";
  }

  // member: next_state
  {
    out << "next_state: ";
    rosidl_generator_traits::value_to_yaml(msg.next_state, out);
    out << ", ";
  }

  // member: next_carry
  {
    out << "next_carry: ";
    rosidl_generator_traits::value_to_yaml(msg.next_carry, out);
    out << ", ";
  }

  // member: next_push_state
  {
    out << "next_push_state: ";
    rosidl_generator_traits::value_to_yaml(msg.next_push_state, out);
    out << ", ";
  }

  // member: next_shoot_angle_state
  {
    out << "next_shoot_angle_state: ";
    rosidl_generator_traits::value_to_yaml(msg.next_shoot_angle_state, out);
    out << ", ";
  }

  // member: next_is_left_arm_open
  {
    out << "next_is_left_arm_open: ";
    rosidl_generator_traits::value_to_yaml(msg.next_is_left_arm_open, out);
    out << ", ";
  }

  // member: next_is_right_arm_open
  {
    out << "next_is_right_arm_open: ";
    rosidl_generator_traits::value_to_yaml(msg.next_is_right_arm_open, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const BallPutGate_Result & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: success
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "success: ";
    rosidl_generator_traits::value_to_yaml(msg.success, out);
    out << "\n";
  }

  // member: next_state
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "next_state: ";
    rosidl_generator_traits::value_to_yaml(msg.next_state, out);
    out << "\n";
  }

  // member: next_carry
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "next_carry: ";
    rosidl_generator_traits::value_to_yaml(msg.next_carry, out);
    out << "\n";
  }

  // member: next_push_state
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "next_push_state: ";
    rosidl_generator_traits::value_to_yaml(msg.next_push_state, out);
    out << "\n";
  }

  // member: next_shoot_angle_state
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "next_shoot_angle_state: ";
    rosidl_generator_traits::value_to_yaml(msg.next_shoot_angle_state, out);
    out << "\n";
  }

  // member: next_is_left_arm_open
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "next_is_left_arm_open: ";
    rosidl_generator_traits::value_to_yaml(msg.next_is_left_arm_open, out);
    out << "\n";
  }

  // member: next_is_right_arm_open
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "next_is_right_arm_open: ";
    rosidl_generator_traits::value_to_yaml(msg.next_is_right_arm_open, out);
    out << "\n";
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const BallPutGate_Result & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use tourobo_2026_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const tourobo_2026_interfaces::action::BallPutGate_Result & msg,
  std::ostream & out, size_t indentation = 0)
{
  tourobo_2026_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use tourobo_2026_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const tourobo_2026_interfaces::action::BallPutGate_Result & msg)
{
  return tourobo_2026_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<tourobo_2026_interfaces::action::BallPutGate_Result>()
{
  return "tourobo_2026_interfaces::action::BallPutGate_Result";
}

template<>
inline const char * name<tourobo_2026_interfaces::action::BallPutGate_Result>()
{
  return "tourobo_2026_interfaces/action/BallPutGate_Result";
}

template<>
struct has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_Result>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_Result>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<tourobo_2026_interfaces::action::BallPutGate_Result>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace tourobo_2026_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const BallPutGate_Feedback & msg,
  std::ostream & out)
{
  (void)msg;
  out << "null";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const BallPutGate_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  (void)msg;
  (void)indentation;
  out << "null\n";
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const BallPutGate_Feedback & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use tourobo_2026_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const tourobo_2026_interfaces::action::BallPutGate_Feedback & msg,
  std::ostream & out, size_t indentation = 0)
{
  tourobo_2026_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use tourobo_2026_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const tourobo_2026_interfaces::action::BallPutGate_Feedback & msg)
{
  return tourobo_2026_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<tourobo_2026_interfaces::action::BallPutGate_Feedback>()
{
  return "tourobo_2026_interfaces::action::BallPutGate_Feedback";
}

template<>
inline const char * name<tourobo_2026_interfaces::action::BallPutGate_Feedback>()
{
  return "tourobo_2026_interfaces/action/BallPutGate_Feedback";
}

template<>
struct has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_Feedback>
  : std::integral_constant<bool, true> {};

template<>
struct has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_Feedback>
  : std::integral_constant<bool, true> {};

template<>
struct is_message<tourobo_2026_interfaces::action::BallPutGate_Feedback>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'goal'
#include "tourobo_2026_interfaces/action/detail/ball_put_gate__traits.hpp"

namespace tourobo_2026_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const BallPutGate_SendGoal_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
    out << ", ";
  }

  // member: goal
  {
    out << "goal: ";
    to_flow_style_yaml(msg.goal, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const BallPutGate_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }

  // member: goal
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal:\n";
    to_block_style_yaml(msg.goal, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const BallPutGate_SendGoal_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use tourobo_2026_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const tourobo_2026_interfaces::action::BallPutGate_SendGoal_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  tourobo_2026_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use tourobo_2026_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const tourobo_2026_interfaces::action::BallPutGate_SendGoal_Request & msg)
{
  return tourobo_2026_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Request>()
{
  return "tourobo_2026_interfaces::action::BallPutGate_SendGoal_Request";
}

template<>
inline const char * name<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Request>()
{
  return "tourobo_2026_interfaces/action/BallPutGate_SendGoal_Request";
}

template<>
struct has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Request>
  : std::integral_constant<bool, has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_Goal>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Request>
  : std::integral_constant<bool, has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_Goal>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__traits.hpp"

namespace tourobo_2026_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const BallPutGate_SendGoal_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: accepted
  {
    out << "accepted: ";
    rosidl_generator_traits::value_to_yaml(msg.accepted, out);
    out << ", ";
  }

  // member: stamp
  {
    out << "stamp: ";
    to_flow_style_yaml(msg.stamp, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const BallPutGate_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: accepted
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "accepted: ";
    rosidl_generator_traits::value_to_yaml(msg.accepted, out);
    out << "\n";
  }

  // member: stamp
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "stamp:\n";
    to_block_style_yaml(msg.stamp, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const BallPutGate_SendGoal_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use tourobo_2026_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const tourobo_2026_interfaces::action::BallPutGate_SendGoal_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  tourobo_2026_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use tourobo_2026_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const tourobo_2026_interfaces::action::BallPutGate_SendGoal_Response & msg)
{
  return tourobo_2026_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Response>()
{
  return "tourobo_2026_interfaces::action::BallPutGate_SendGoal_Response";
}

template<>
inline const char * name<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Response>()
{
  return "tourobo_2026_interfaces/action/BallPutGate_SendGoal_Response";
}

template<>
struct has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Response>
  : std::integral_constant<bool, has_fixed_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Response>
  : std::integral_constant<bool, has_bounded_size<builtin_interfaces::msg::Time>::value> {};

template<>
struct is_message<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<tourobo_2026_interfaces::action::BallPutGate_SendGoal>()
{
  return "tourobo_2026_interfaces::action::BallPutGate_SendGoal";
}

template<>
inline const char * name<tourobo_2026_interfaces::action::BallPutGate_SendGoal>()
{
  return "tourobo_2026_interfaces/action/BallPutGate_SendGoal";
}

template<>
struct has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_SendGoal>
  : std::integral_constant<
    bool,
    has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Request>::value &&
    has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Response>::value
  >
{
};

template<>
struct has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_SendGoal>
  : std::integral_constant<
    bool,
    has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Request>::value &&
    has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Response>::value
  >
{
};

template<>
struct is_service<tourobo_2026_interfaces::action::BallPutGate_SendGoal>
  : std::true_type
{
};

template<>
struct is_service_request<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Request>
  : std::true_type
{
};

template<>
struct is_service_response<tourobo_2026_interfaces::action::BallPutGate_SendGoal_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"

namespace tourobo_2026_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const BallPutGate_GetResult_Request & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const BallPutGate_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const BallPutGate_GetResult_Request & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use tourobo_2026_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const tourobo_2026_interfaces::action::BallPutGate_GetResult_Request & msg,
  std::ostream & out, size_t indentation = 0)
{
  tourobo_2026_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use tourobo_2026_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const tourobo_2026_interfaces::action::BallPutGate_GetResult_Request & msg)
{
  return tourobo_2026_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<tourobo_2026_interfaces::action::BallPutGate_GetResult_Request>()
{
  return "tourobo_2026_interfaces::action::BallPutGate_GetResult_Request";
}

template<>
inline const char * name<tourobo_2026_interfaces::action::BallPutGate_GetResult_Request>()
{
  return "tourobo_2026_interfaces/action/BallPutGate_GetResult_Request";
}

template<>
struct has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_GetResult_Request>
  : std::integral_constant<bool, has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_GetResult_Request>
  : std::integral_constant<bool, has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<tourobo_2026_interfaces::action::BallPutGate_GetResult_Request>
  : std::true_type {};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'result'
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_gate__traits.hpp"

namespace tourobo_2026_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const BallPutGate_GetResult_Response & msg,
  std::ostream & out)
{
  out << "{";
  // member: status
  {
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << ", ";
  }

  // member: result
  {
    out << "result: ";
    to_flow_style_yaml(msg.result, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const BallPutGate_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: status
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "status: ";
    rosidl_generator_traits::value_to_yaml(msg.status, out);
    out << "\n";
  }

  // member: result
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "result:\n";
    to_block_style_yaml(msg.result, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const BallPutGate_GetResult_Response & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use tourobo_2026_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const tourobo_2026_interfaces::action::BallPutGate_GetResult_Response & msg,
  std::ostream & out, size_t indentation = 0)
{
  tourobo_2026_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use tourobo_2026_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const tourobo_2026_interfaces::action::BallPutGate_GetResult_Response & msg)
{
  return tourobo_2026_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<tourobo_2026_interfaces::action::BallPutGate_GetResult_Response>()
{
  return "tourobo_2026_interfaces::action::BallPutGate_GetResult_Response";
}

template<>
inline const char * name<tourobo_2026_interfaces::action::BallPutGate_GetResult_Response>()
{
  return "tourobo_2026_interfaces/action/BallPutGate_GetResult_Response";
}

template<>
struct has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_GetResult_Response>
  : std::integral_constant<bool, has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_Result>::value> {};

template<>
struct has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_GetResult_Response>
  : std::integral_constant<bool, has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_Result>::value> {};

template<>
struct is_message<tourobo_2026_interfaces::action::BallPutGate_GetResult_Response>
  : std::true_type {};

}  // namespace rosidl_generator_traits

namespace rosidl_generator_traits
{

template<>
inline const char * data_type<tourobo_2026_interfaces::action::BallPutGate_GetResult>()
{
  return "tourobo_2026_interfaces::action::BallPutGate_GetResult";
}

template<>
inline const char * name<tourobo_2026_interfaces::action::BallPutGate_GetResult>()
{
  return "tourobo_2026_interfaces/action/BallPutGate_GetResult";
}

template<>
struct has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_GetResult>
  : std::integral_constant<
    bool,
    has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_GetResult_Request>::value &&
    has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_GetResult_Response>::value
  >
{
};

template<>
struct has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_GetResult>
  : std::integral_constant<
    bool,
    has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_GetResult_Request>::value &&
    has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_GetResult_Response>::value
  >
{
};

template<>
struct is_service<tourobo_2026_interfaces::action::BallPutGate_GetResult>
  : std::true_type
{
};

template<>
struct is_service_request<tourobo_2026_interfaces::action::BallPutGate_GetResult_Request>
  : std::true_type
{
};

template<>
struct is_service_response<tourobo_2026_interfaces::action::BallPutGate_GetResult_Response>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits

// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__traits.hpp"
// Member 'feedback'
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_gate__traits.hpp"

namespace tourobo_2026_interfaces
{

namespace action
{

inline void to_flow_style_yaml(
  const BallPutGate_FeedbackMessage & msg,
  std::ostream & out)
{
  out << "{";
  // member: goal_id
  {
    out << "goal_id: ";
    to_flow_style_yaml(msg.goal_id, out);
    out << ", ";
  }

  // member: feedback
  {
    out << "feedback: ";
    to_flow_style_yaml(msg.feedback, out);
  }
  out << "}";
}  // NOLINT(readability/fn_size)

inline void to_block_style_yaml(
  const BallPutGate_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  // member: goal_id
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "goal_id:\n";
    to_block_style_yaml(msg.goal_id, out, indentation + 2);
  }

  // member: feedback
  {
    if (indentation > 0) {
      out << std::string(indentation, ' ');
    }
    out << "feedback:\n";
    to_block_style_yaml(msg.feedback, out, indentation + 2);
  }
}  // NOLINT(readability/fn_size)

inline std::string to_yaml(const BallPutGate_FeedbackMessage & msg, bool use_flow_style = false)
{
  std::ostringstream out;
  if (use_flow_style) {
    to_flow_style_yaml(msg, out);
  } else {
    to_block_style_yaml(msg, out);
  }
  return out.str();
}

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_generator_traits
{

[[deprecated("use tourobo_2026_interfaces::action::to_block_style_yaml() instead")]]
inline void to_yaml(
  const tourobo_2026_interfaces::action::BallPutGate_FeedbackMessage & msg,
  std::ostream & out, size_t indentation = 0)
{
  tourobo_2026_interfaces::action::to_block_style_yaml(msg, out, indentation);
}

[[deprecated("use tourobo_2026_interfaces::action::to_yaml() instead")]]
inline std::string to_yaml(const tourobo_2026_interfaces::action::BallPutGate_FeedbackMessage & msg)
{
  return tourobo_2026_interfaces::action::to_yaml(msg);
}

template<>
inline const char * data_type<tourobo_2026_interfaces::action::BallPutGate_FeedbackMessage>()
{
  return "tourobo_2026_interfaces::action::BallPutGate_FeedbackMessage";
}

template<>
inline const char * name<tourobo_2026_interfaces::action::BallPutGate_FeedbackMessage>()
{
  return "tourobo_2026_interfaces/action/BallPutGate_FeedbackMessage";
}

template<>
struct has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_FeedbackMessage>
  : std::integral_constant<bool, has_fixed_size<tourobo_2026_interfaces::action::BallPutGate_Feedback>::value && has_fixed_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_FeedbackMessage>
  : std::integral_constant<bool, has_bounded_size<tourobo_2026_interfaces::action::BallPutGate_Feedback>::value && has_bounded_size<unique_identifier_msgs::msg::UUID>::value> {};

template<>
struct is_message<tourobo_2026_interfaces::action::BallPutGate_FeedbackMessage>
  : std::true_type {};

}  // namespace rosidl_generator_traits


namespace rosidl_generator_traits
{

template<>
struct is_action<tourobo_2026_interfaces::action::BallPutGate>
  : std::true_type
{
};

template<>
struct is_action_goal<tourobo_2026_interfaces::action::BallPutGate_Goal>
  : std::true_type
{
};

template<>
struct is_action_result<tourobo_2026_interfaces::action::BallPutGate_Result>
  : std::true_type
{
};

template<>
struct is_action_feedback<tourobo_2026_interfaces::action::BallPutGate_Feedback>
  : std::true_type
{
};

}  // namespace rosidl_generator_traits


#endif  // TOUROBO_2026_INTERFACES__ACTION__DETAIL__BALL_PUT_GATE__TRAITS_HPP_
