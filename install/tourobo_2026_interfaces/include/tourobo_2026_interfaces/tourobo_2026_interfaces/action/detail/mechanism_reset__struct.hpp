// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from tourobo_2026_interfaces:action/MechanismReset.idl
// generated code does not contain a copyright notice

#ifndef TOUROBO_2026_INTERFACES__ACTION__DETAIL__MECHANISM_RESET__STRUCT_HPP_
#define TOUROBO_2026_INTERFACES__ACTION__DETAIL__MECHANISM_RESET__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_Goal __attribute__((deprecated))
#else
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_Goal __declspec(deprecated)
#endif

namespace tourobo_2026_interfaces
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct MechanismReset_Goal_
{
  using Type = MechanismReset_Goal_<ContainerAllocator>;

  explicit MechanismReset_Goal_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->execute = false;
    }
  }

  explicit MechanismReset_Goal_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->execute = false;
    }
  }

  // field types and members
  using _execute_type =
    bool;
  _execute_type execute;

  // setters for named parameter idiom
  Type & set__execute(
    const bool & _arg)
  {
    this->execute = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    tourobo_2026_interfaces::action::MechanismReset_Goal_<ContainerAllocator> *;
  using ConstRawPtr =
    const tourobo_2026_interfaces::action::MechanismReset_Goal_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_Goal_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_Goal_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_Goal_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_Goal_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_Goal_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_Goal_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_Goal_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_Goal_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_Goal
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_Goal_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_Goal
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_Goal_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const MechanismReset_Goal_ & other) const
  {
    if (this->execute != other.execute) {
      return false;
    }
    return true;
  }
  bool operator!=(const MechanismReset_Goal_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct MechanismReset_Goal_

// alias to use template instance with default allocator
using MechanismReset_Goal =
  tourobo_2026_interfaces::action::MechanismReset_Goal_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace tourobo_2026_interfaces


#ifndef _WIN32
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_Result __attribute__((deprecated))
#else
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_Result __declspec(deprecated)
#endif

namespace tourobo_2026_interfaces
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct MechanismReset_Result_
{
  using Type = MechanismReset_Result_<ContainerAllocator>;

  explicit MechanismReset_Result_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->next_state = 0l;
      this->next_carry = 0l;
      this->next_push_state = 0l;
      this->next_is_left_arm_open = false;
      this->next_is_right_arm_open = false;
    }
  }

  explicit MechanismReset_Result_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->success = false;
      this->next_state = 0l;
      this->next_carry = 0l;
      this->next_push_state = 0l;
      this->next_is_left_arm_open = false;
      this->next_is_right_arm_open = false;
    }
  }

  // field types and members
  using _success_type =
    bool;
  _success_type success;
  using _next_state_type =
    int32_t;
  _next_state_type next_state;
  using _next_carry_type =
    int32_t;
  _next_carry_type next_carry;
  using _next_push_state_type =
    int32_t;
  _next_push_state_type next_push_state;
  using _next_is_left_arm_open_type =
    bool;
  _next_is_left_arm_open_type next_is_left_arm_open;
  using _next_is_right_arm_open_type =
    bool;
  _next_is_right_arm_open_type next_is_right_arm_open;

  // setters for named parameter idiom
  Type & set__success(
    const bool & _arg)
  {
    this->success = _arg;
    return *this;
  }
  Type & set__next_state(
    const int32_t & _arg)
  {
    this->next_state = _arg;
    return *this;
  }
  Type & set__next_carry(
    const int32_t & _arg)
  {
    this->next_carry = _arg;
    return *this;
  }
  Type & set__next_push_state(
    const int32_t & _arg)
  {
    this->next_push_state = _arg;
    return *this;
  }
  Type & set__next_is_left_arm_open(
    const bool & _arg)
  {
    this->next_is_left_arm_open = _arg;
    return *this;
  }
  Type & set__next_is_right_arm_open(
    const bool & _arg)
  {
    this->next_is_right_arm_open = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    tourobo_2026_interfaces::action::MechanismReset_Result_<ContainerAllocator> *;
  using ConstRawPtr =
    const tourobo_2026_interfaces::action::MechanismReset_Result_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_Result_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_Result_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_Result_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_Result_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_Result_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_Result_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_Result_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_Result_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_Result
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_Result_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_Result
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_Result_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const MechanismReset_Result_ & other) const
  {
    if (this->success != other.success) {
      return false;
    }
    if (this->next_state != other.next_state) {
      return false;
    }
    if (this->next_carry != other.next_carry) {
      return false;
    }
    if (this->next_push_state != other.next_push_state) {
      return false;
    }
    if (this->next_is_left_arm_open != other.next_is_left_arm_open) {
      return false;
    }
    if (this->next_is_right_arm_open != other.next_is_right_arm_open) {
      return false;
    }
    return true;
  }
  bool operator!=(const MechanismReset_Result_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct MechanismReset_Result_

// alias to use template instance with default allocator
using MechanismReset_Result =
  tourobo_2026_interfaces::action::MechanismReset_Result_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace tourobo_2026_interfaces


#ifndef _WIN32
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_Feedback __attribute__((deprecated))
#else
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_Feedback __declspec(deprecated)
#endif

namespace tourobo_2026_interfaces
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct MechanismReset_Feedback_
{
  using Type = MechanismReset_Feedback_<ContainerAllocator>;

  explicit MechanismReset_Feedback_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->structure_needs_at_least_one_member = 0;
    }
  }

  explicit MechanismReset_Feedback_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->structure_needs_at_least_one_member = 0;
    }
  }

  // field types and members
  using _structure_needs_at_least_one_member_type =
    uint8_t;
  _structure_needs_at_least_one_member_type structure_needs_at_least_one_member;


  // constant declarations

  // pointer types
  using RawPtr =
    tourobo_2026_interfaces::action::MechanismReset_Feedback_<ContainerAllocator> *;
  using ConstRawPtr =
    const tourobo_2026_interfaces::action::MechanismReset_Feedback_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_Feedback_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_Feedback_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_Feedback_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_Feedback_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_Feedback_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_Feedback_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_Feedback_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_Feedback_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_Feedback
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_Feedback_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_Feedback
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_Feedback_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const MechanismReset_Feedback_ & other) const
  {
    if (this->structure_needs_at_least_one_member != other.structure_needs_at_least_one_member) {
      return false;
    }
    return true;
  }
  bool operator!=(const MechanismReset_Feedback_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct MechanismReset_Feedback_

// alias to use template instance with default allocator
using MechanismReset_Feedback =
  tourobo_2026_interfaces::action::MechanismReset_Feedback_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace tourobo_2026_interfaces


// Include directives for member types
// Member 'goal_id'
#include "unique_identifier_msgs/msg/detail/uuid__struct.hpp"
// Member 'goal'
#include "tourobo_2026_interfaces/action/detail/mechanism_reset__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request __attribute__((deprecated))
#else
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request __declspec(deprecated)
#endif

namespace tourobo_2026_interfaces
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct MechanismReset_SendGoal_Request_
{
  using Type = MechanismReset_SendGoal_Request_<ContainerAllocator>;

  explicit MechanismReset_SendGoal_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : goal_id(_init),
    goal(_init)
  {
    (void)_init;
  }

  explicit MechanismReset_SendGoal_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : goal_id(_alloc, _init),
    goal(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _goal_id_type =
    unique_identifier_msgs::msg::UUID_<ContainerAllocator>;
  _goal_id_type goal_id;
  using _goal_type =
    tourobo_2026_interfaces::action::MechanismReset_Goal_<ContainerAllocator>;
  _goal_type goal;

  // setters for named parameter idiom
  Type & set__goal_id(
    const unique_identifier_msgs::msg::UUID_<ContainerAllocator> & _arg)
  {
    this->goal_id = _arg;
    return *this;
  }
  Type & set__goal(
    const tourobo_2026_interfaces::action::MechanismReset_Goal_<ContainerAllocator> & _arg)
  {
    this->goal = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const MechanismReset_SendGoal_Request_ & other) const
  {
    if (this->goal_id != other.goal_id) {
      return false;
    }
    if (this->goal != other.goal) {
      return false;
    }
    return true;
  }
  bool operator!=(const MechanismReset_SendGoal_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct MechanismReset_SendGoal_Request_

// alias to use template instance with default allocator
using MechanismReset_SendGoal_Request =
  tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace tourobo_2026_interfaces


// Include directives for member types
// Member 'stamp'
#include "builtin_interfaces/msg/detail/time__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response __attribute__((deprecated))
#else
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response __declspec(deprecated)
#endif

namespace tourobo_2026_interfaces
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct MechanismReset_SendGoal_Response_
{
  using Type = MechanismReset_SendGoal_Response_<ContainerAllocator>;

  explicit MechanismReset_SendGoal_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : stamp(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->accepted = false;
    }
  }

  explicit MechanismReset_SendGoal_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : stamp(_alloc, _init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->accepted = false;
    }
  }

  // field types and members
  using _accepted_type =
    bool;
  _accepted_type accepted;
  using _stamp_type =
    builtin_interfaces::msg::Time_<ContainerAllocator>;
  _stamp_type stamp;

  // setters for named parameter idiom
  Type & set__accepted(
    const bool & _arg)
  {
    this->accepted = _arg;
    return *this;
  }
  Type & set__stamp(
    const builtin_interfaces::msg::Time_<ContainerAllocator> & _arg)
  {
    this->stamp = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const MechanismReset_SendGoal_Response_ & other) const
  {
    if (this->accepted != other.accepted) {
      return false;
    }
    if (this->stamp != other.stamp) {
      return false;
    }
    return true;
  }
  bool operator!=(const MechanismReset_SendGoal_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct MechanismReset_SendGoal_Response_

// alias to use template instance with default allocator
using MechanismReset_SendGoal_Response =
  tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace tourobo_2026_interfaces
{

namespace action
{

struct MechanismReset_SendGoal
{
  using Request = tourobo_2026_interfaces::action::MechanismReset_SendGoal_Request;
  using Response = tourobo_2026_interfaces::action::MechanismReset_SendGoal_Response;
};

}  // namespace action

}  // namespace tourobo_2026_interfaces


// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_GetResult_Request __attribute__((deprecated))
#else
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_GetResult_Request __declspec(deprecated)
#endif

namespace tourobo_2026_interfaces
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct MechanismReset_GetResult_Request_
{
  using Type = MechanismReset_GetResult_Request_<ContainerAllocator>;

  explicit MechanismReset_GetResult_Request_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : goal_id(_init)
  {
    (void)_init;
  }

  explicit MechanismReset_GetResult_Request_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : goal_id(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _goal_id_type =
    unique_identifier_msgs::msg::UUID_<ContainerAllocator>;
  _goal_id_type goal_id;

  // setters for named parameter idiom
  Type & set__goal_id(
    const unique_identifier_msgs::msg::UUID_<ContainerAllocator> & _arg)
  {
    this->goal_id = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    tourobo_2026_interfaces::action::MechanismReset_GetResult_Request_<ContainerAllocator> *;
  using ConstRawPtr =
    const tourobo_2026_interfaces::action::MechanismReset_GetResult_Request_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Request_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Request_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_GetResult_Request_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Request_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_GetResult_Request_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Request_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Request_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Request_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_GetResult_Request
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Request_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_GetResult_Request
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Request_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const MechanismReset_GetResult_Request_ & other) const
  {
    if (this->goal_id != other.goal_id) {
      return false;
    }
    return true;
  }
  bool operator!=(const MechanismReset_GetResult_Request_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct MechanismReset_GetResult_Request_

// alias to use template instance with default allocator
using MechanismReset_GetResult_Request =
  tourobo_2026_interfaces::action::MechanismReset_GetResult_Request_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace tourobo_2026_interfaces


// Include directives for member types
// Member 'result'
// already included above
// #include "tourobo_2026_interfaces/action/detail/mechanism_reset__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_GetResult_Response __attribute__((deprecated))
#else
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_GetResult_Response __declspec(deprecated)
#endif

namespace tourobo_2026_interfaces
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct MechanismReset_GetResult_Response_
{
  using Type = MechanismReset_GetResult_Response_<ContainerAllocator>;

  explicit MechanismReset_GetResult_Response_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : result(_init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->status = 0;
    }
  }

  explicit MechanismReset_GetResult_Response_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : result(_alloc, _init)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->status = 0;
    }
  }

  // field types and members
  using _status_type =
    int8_t;
  _status_type status;
  using _result_type =
    tourobo_2026_interfaces::action::MechanismReset_Result_<ContainerAllocator>;
  _result_type result;

  // setters for named parameter idiom
  Type & set__status(
    const int8_t & _arg)
  {
    this->status = _arg;
    return *this;
  }
  Type & set__result(
    const tourobo_2026_interfaces::action::MechanismReset_Result_<ContainerAllocator> & _arg)
  {
    this->result = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    tourobo_2026_interfaces::action::MechanismReset_GetResult_Response_<ContainerAllocator> *;
  using ConstRawPtr =
    const tourobo_2026_interfaces::action::MechanismReset_GetResult_Response_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Response_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Response_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_GetResult_Response_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Response_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_GetResult_Response_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Response_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Response_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Response_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_GetResult_Response
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Response_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_GetResult_Response
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_GetResult_Response_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const MechanismReset_GetResult_Response_ & other) const
  {
    if (this->status != other.status) {
      return false;
    }
    if (this->result != other.result) {
      return false;
    }
    return true;
  }
  bool operator!=(const MechanismReset_GetResult_Response_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct MechanismReset_GetResult_Response_

// alias to use template instance with default allocator
using MechanismReset_GetResult_Response =
  tourobo_2026_interfaces::action::MechanismReset_GetResult_Response_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace tourobo_2026_interfaces
{

namespace action
{

struct MechanismReset_GetResult
{
  using Request = tourobo_2026_interfaces::action::MechanismReset_GetResult_Request;
  using Response = tourobo_2026_interfaces::action::MechanismReset_GetResult_Response;
};

}  // namespace action

}  // namespace tourobo_2026_interfaces


// Include directives for member types
// Member 'goal_id'
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__struct.hpp"
// Member 'feedback'
// already included above
// #include "tourobo_2026_interfaces/action/detail/mechanism_reset__struct.hpp"

#ifndef _WIN32
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage __attribute__((deprecated))
#else
# define DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage __declspec(deprecated)
#endif

namespace tourobo_2026_interfaces
{

namespace action
{

// message struct
template<class ContainerAllocator>
struct MechanismReset_FeedbackMessage_
{
  using Type = MechanismReset_FeedbackMessage_<ContainerAllocator>;

  explicit MechanismReset_FeedbackMessage_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : goal_id(_init),
    feedback(_init)
  {
    (void)_init;
  }

  explicit MechanismReset_FeedbackMessage_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  : goal_id(_alloc, _init),
    feedback(_alloc, _init)
  {
    (void)_init;
  }

  // field types and members
  using _goal_id_type =
    unique_identifier_msgs::msg::UUID_<ContainerAllocator>;
  _goal_id_type goal_id;
  using _feedback_type =
    tourobo_2026_interfaces::action::MechanismReset_Feedback_<ContainerAllocator>;
  _feedback_type feedback;

  // setters for named parameter idiom
  Type & set__goal_id(
    const unique_identifier_msgs::msg::UUID_<ContainerAllocator> & _arg)
  {
    this->goal_id = _arg;
    return *this;
  }
  Type & set__feedback(
    const tourobo_2026_interfaces::action::MechanismReset_Feedback_<ContainerAllocator> & _arg)
  {
    this->feedback = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage_<ContainerAllocator> *;
  using ConstRawPtr =
    const tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage
    std::shared_ptr<tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const MechanismReset_FeedbackMessage_ & other) const
  {
    if (this->goal_id != other.goal_id) {
      return false;
    }
    if (this->feedback != other.feedback) {
      return false;
    }
    return true;
  }
  bool operator!=(const MechanismReset_FeedbackMessage_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct MechanismReset_FeedbackMessage_

// alias to use template instance with default allocator
using MechanismReset_FeedbackMessage =
  tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage_<std::allocator<void>>;

// constant definitions

}  // namespace action

}  // namespace tourobo_2026_interfaces

#include "action_msgs/srv/cancel_goal.hpp"
#include "action_msgs/msg/goal_info.hpp"
#include "action_msgs/msg/goal_status_array.hpp"

namespace tourobo_2026_interfaces
{

namespace action
{

struct MechanismReset
{
  /// The goal message defined in the action definition.
  using Goal = tourobo_2026_interfaces::action::MechanismReset_Goal;
  /// The result message defined in the action definition.
  using Result = tourobo_2026_interfaces::action::MechanismReset_Result;
  /// The feedback message defined in the action definition.
  using Feedback = tourobo_2026_interfaces::action::MechanismReset_Feedback;

  struct Impl
  {
    /// The send_goal service using a wrapped version of the goal message as a request.
    using SendGoalService = tourobo_2026_interfaces::action::MechanismReset_SendGoal;
    /// The get_result service using a wrapped version of the result message as a response.
    using GetResultService = tourobo_2026_interfaces::action::MechanismReset_GetResult;
    /// The feedback message with generic fields which wraps the feedback message.
    using FeedbackMessage = tourobo_2026_interfaces::action::MechanismReset_FeedbackMessage;

    /// The generic service to cancel a goal.
    using CancelGoalService = action_msgs::srv::CancelGoal;
    /// The generic message for the status of a goal.
    using GoalStatusMessage = action_msgs::msg::GoalStatusArray;
  };
};

typedef struct MechanismReset MechanismReset;

}  // namespace action

}  // namespace tourobo_2026_interfaces

#endif  // TOUROBO_2026_INTERFACES__ACTION__DETAIL__MECHANISM_RESET__STRUCT_HPP_
