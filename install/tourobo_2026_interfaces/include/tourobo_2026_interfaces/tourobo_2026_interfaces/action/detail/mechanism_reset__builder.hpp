// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from tourobo_2026_interfaces:action/MechanismReset.idl
// generated code does not contain a copyright notice

#ifndef TOUROBO_2026_INTERFACES__ACTION__DETAIL__MECHANISM_RESET__BUILDER_HPP_
#define TOUROBO_2026_INTERFACES__ACTION__DETAIL__MECHANISM_RESET__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "tourobo_2026_interfaces/action/detail/mechanism_reset__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace tourobo_2026_interfaces
{

namespace action
{

namespace builder
{

class Init_MechanismReset_Goal_execute
{
public:
  Init_MechanismReset_Goal_execute()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::tourobo_2026_interfaces::action::MechanismReset_Goal execute(::tourobo_2026_interfaces::action::MechanismReset_Goal::_execute_type arg)
  {
    msg_.execute = std::move(arg);
    return std::move(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_Goal msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::tourobo_2026_interfaces::action::MechanismReset_Goal>()
{
  return tourobo_2026_interfaces::action::builder::Init_MechanismReset_Goal_execute();
}

}  // namespace tourobo_2026_interfaces


namespace tourobo_2026_interfaces
{

namespace action
{

namespace builder
{

class Init_MechanismReset_Result_next_is_right_arm_open
{
public:
  explicit Init_MechanismReset_Result_next_is_right_arm_open(::tourobo_2026_interfaces::action::MechanismReset_Result & msg)
  : msg_(msg)
  {}
  ::tourobo_2026_interfaces::action::MechanismReset_Result next_is_right_arm_open(::tourobo_2026_interfaces::action::MechanismReset_Result::_next_is_right_arm_open_type arg)
  {
    msg_.next_is_right_arm_open = std::move(arg);
    return std::move(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_Result msg_;
};

class Init_MechanismReset_Result_next_is_left_arm_open
{
public:
  explicit Init_MechanismReset_Result_next_is_left_arm_open(::tourobo_2026_interfaces::action::MechanismReset_Result & msg)
  : msg_(msg)
  {}
  Init_MechanismReset_Result_next_is_right_arm_open next_is_left_arm_open(::tourobo_2026_interfaces::action::MechanismReset_Result::_next_is_left_arm_open_type arg)
  {
    msg_.next_is_left_arm_open = std::move(arg);
    return Init_MechanismReset_Result_next_is_right_arm_open(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_Result msg_;
};

class Init_MechanismReset_Result_next_push_state
{
public:
  explicit Init_MechanismReset_Result_next_push_state(::tourobo_2026_interfaces::action::MechanismReset_Result & msg)
  : msg_(msg)
  {}
  Init_MechanismReset_Result_next_is_left_arm_open next_push_state(::tourobo_2026_interfaces::action::MechanismReset_Result::_next_push_state_type arg)
  {
    msg_.next_push_state = std::move(arg);
    return Init_MechanismReset_Result_next_is_left_arm_open(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_Result msg_;
};

class Init_MechanismReset_Result_next_carry
{
public:
  explicit Init_MechanismReset_Result_next_carry(::tourobo_2026_interfaces::action::MechanismReset_Result & msg)
  : msg_(msg)
  {}
  Init_MechanismReset_Result_next_push_state next_carry(::tourobo_2026_interfaces::action::MechanismReset_Result::_next_carry_type arg)
  {
    msg_.next_carry = std::move(arg);
    return Init_MechanismReset_Result_next_push_state(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_Result msg_;
};

class Init_MechanismReset_Result_next_state
{
public:
  explicit Init_MechanismReset_Result_next_state(::tourobo_2026_interfaces::action::MechanismReset_Result & msg)
  : msg_(msg)
  {}
  Init_MechanismReset_Result_next_carry next_state(::tourobo_2026_interfaces::action::MechanismReset_Result::_next_state_type arg)
  {
    msg_.next_state = std::move(arg);
    return Init_MechanismReset_Result_next_carry(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_Result msg_;
};

class Init_MechanismReset_Result_success
{
public:
  Init_MechanismReset_Result_success()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_MechanismReset_Result_next_state success(::tourobo_2026_interfaces::action::MechanismReset_Result::_success_type arg)
  {
    msg_.success = std::move(arg);
    return Init_MechanismReset_Result_next_state(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_Result msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::tourobo_2026_interfaces::action::MechanismReset_Result>()
{
  return tourobo_2026_interfaces::action::builder::Init_MechanismReset_Result_success();
}

}  // namespace tourobo_2026_interfaces


namespace tourobo_2026_interfaces
{

namespace action
{


}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::tourobo_2026_interfaces::action::MechanismReset_Feedback>()
{
  return ::tourobo_2026_interfaces::action::MechanismReset_Feedback(rosidl_runtime_cpp::MessageInitialization::ZERO);
}

}  // namespace tourobo_2026_interfaces


namespace tourobo_2026_interfaces
{

namespace action
{

namespace builder
{

class Init_MechanismReset_SendGoal_Request_goal
{
public:
  explicit Init_MechanismReset_SendGoal_Request_goal(::tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request & msg)
  : msg_(msg)
  {}
  ::tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request goal(::tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request::_goal_type arg)
  {
    msg_.goal = std::move(arg);
    return std::move(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request msg_;
};

class Init_MechanismReset_SendGoal_Request_goal_id
{
public:
  Init_MechanismReset_SendGoal_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_MechanismReset_SendGoal_Request_goal goal_id(::tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_MechanismReset_SendGoal_Request_goal(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request>()
{
  return tourobo_2026_interfaces::action::builder::Init_MechanismReset_SendGoal_Request_goal_id();
}

}  // namespace tourobo_2026_interfaces


namespace tourobo_2026_interfaces
{

namespace action
{

namespace builder
{

class Init_MechanismReset_SendGoal_Response_stamp
{
public:
  explicit Init_MechanismReset_SendGoal_Response_stamp(::tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response & msg)
  : msg_(msg)
  {}
  ::tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response stamp(::tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response::_stamp_type arg)
  {
    msg_.stamp = std::move(arg);
    return std::move(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response msg_;
};

class Init_MechanismReset_SendGoal_Response_accepted
{
public:
  Init_MechanismReset_SendGoal_Response_accepted()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_MechanismReset_SendGoal_Response_stamp accepted(::tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response::_accepted_type arg)
  {
    msg_.accepted = std::move(arg);
    return Init_MechanismReset_SendGoal_Response_stamp(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response>()
{
  return tourobo_2026_interfaces::action::builder::Init_MechanismReset_SendGoal_Response_accepted();
}

}  // namespace tourobo_2026_interfaces


namespace tourobo_2026_interfaces
{

namespace action
{

namespace builder
{

class Init_MechanismReset_GetResult_Request_goal_id
{
public:
  Init_MechanismReset_GetResult_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::tourobo_2026_interfaces::action::MechanismReset_GetResult_Request goal_id(::tourobo_2026_interfaces::action::MechanismReset_GetResult_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_GetResult_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::tourobo_2026_interfaces::action::MechanismReset_GetResult_Request>()
{
  return tourobo_2026_interfaces::action::builder::Init_MechanismReset_GetResult_Request_goal_id();
}

}  // namespace tourobo_2026_interfaces


namespace tourobo_2026_interfaces
{

namespace action
{

namespace builder
{

class Init_MechanismReset_GetResult_Response_result
{
public:
  explicit Init_MechanismReset_GetResult_Response_result(::tourobo_2026_interfaces::action::MechanismReset_GetResult_Response & msg)
  : msg_(msg)
  {}
  ::tourobo_2026_interfaces::action::MechanismReset_GetResult_Response result(::tourobo_2026_interfaces::action::MechanismReset_GetResult_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return std::move(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_GetResult_Response msg_;
};

class Init_MechanismReset_GetResult_Response_status
{
public:
  Init_MechanismReset_GetResult_Response_status()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_MechanismReset_GetResult_Response_result status(::tourobo_2026_interfaces::action::MechanismReset_GetResult_Response::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_MechanismReset_GetResult_Response_result(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_GetResult_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::tourobo_2026_interfaces::action::MechanismReset_GetResult_Response>()
{
  return tourobo_2026_interfaces::action::builder::Init_MechanismReset_GetResult_Response_status();
}

}  // namespace tourobo_2026_interfaces


namespace tourobo_2026_interfaces
{

namespace action
{

namespace builder
{

class Init_MechanismReset_FeedbackMessage_feedback
{
public:
  explicit Init_MechanismReset_FeedbackMessage_feedback(::tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage & msg)
  : msg_(msg)
  {}
  ::tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage feedback(::tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage::_feedback_type arg)
  {
    msg_.feedback = std::move(arg);
    return std::move(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage msg_;
};

class Init_MechanismReset_FeedbackMessage_goal_id
{
public:
  Init_MechanismReset_FeedbackMessage_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_MechanismReset_FeedbackMessage_feedback goal_id(::tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_MechanismReset_FeedbackMessage_feedback(msg_);
  }

private:
  ::tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage>()
{
  return tourobo_2026_interfaces::action::builder::Init_MechanismReset_FeedbackMessage_goal_id();
}

}  // namespace tourobo_2026_interfaces

#endif  // TOUROBO_2026_INTERFACES__ACTION__DETAIL__MECHANISM_RESET__BUILDER_HPP_
