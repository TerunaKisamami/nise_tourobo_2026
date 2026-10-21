// generated from rosidl_typesupport_cpp/resource/idl__type_support.cpp.em
// with input from tourobo_2026_interfaces:action/BallIntake.idl
// generated code does not contain a copyright notice

#include "cstddef"
#include "rosidl_runtime_c/message_type_support_struct.h"
#include "tourobo_2026_interfaces/action/detail/ball_intake__struct.hpp"
#include "rosidl_typesupport_cpp/identifier.hpp"
#include "rosidl_typesupport_cpp/message_type_support.hpp"
#include "rosidl_typesupport_c/type_support_map.h"
#include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
#include "rosidl_typesupport_cpp/visibility_control.h"
#include "rosidl_typesupport_interface/macros.h"

namespace tourobo_2026_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _BallIntake_Goal_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _BallIntake_Goal_type_support_ids_t;

static const _BallIntake_Goal_type_support_ids_t _BallIntake_Goal_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _BallIntake_Goal_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _BallIntake_Goal_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _BallIntake_Goal_type_support_symbol_names_t _BallIntake_Goal_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, tourobo_2026_interfaces, action, BallIntake_Goal)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, tourobo_2026_interfaces, action, BallIntake_Goal)),
  }
};

typedef struct _BallIntake_Goal_type_support_data_t
{
  void * data[2];
} _BallIntake_Goal_type_support_data_t;

static _BallIntake_Goal_type_support_data_t _BallIntake_Goal_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _BallIntake_Goal_message_typesupport_map = {
  2,
  "tourobo_2026_interfaces",
  &_BallIntake_Goal_message_typesupport_ids.typesupport_identifier[0],
  &_BallIntake_Goal_message_typesupport_symbol_names.symbol_name[0],
  &_BallIntake_Goal_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t BallIntake_Goal_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_BallIntake_Goal_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_Goal>()
{
  return &::tourobo_2026_interfaces::action::rosidl_typesupport_cpp::BallIntake_Goal_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, tourobo_2026_interfaces, action, BallIntake_Goal)() {
  return get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_Goal>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_intake__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace tourobo_2026_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _BallIntake_Result_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _BallIntake_Result_type_support_ids_t;

static const _BallIntake_Result_type_support_ids_t _BallIntake_Result_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _BallIntake_Result_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _BallIntake_Result_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _BallIntake_Result_type_support_symbol_names_t _BallIntake_Result_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, tourobo_2026_interfaces, action, BallIntake_Result)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, tourobo_2026_interfaces, action, BallIntake_Result)),
  }
};

typedef struct _BallIntake_Result_type_support_data_t
{
  void * data[2];
} _BallIntake_Result_type_support_data_t;

static _BallIntake_Result_type_support_data_t _BallIntake_Result_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _BallIntake_Result_message_typesupport_map = {
  2,
  "tourobo_2026_interfaces",
  &_BallIntake_Result_message_typesupport_ids.typesupport_identifier[0],
  &_BallIntake_Result_message_typesupport_symbol_names.symbol_name[0],
  &_BallIntake_Result_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t BallIntake_Result_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_BallIntake_Result_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_Result>()
{
  return &::tourobo_2026_interfaces::action::rosidl_typesupport_cpp::BallIntake_Result_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, tourobo_2026_interfaces, action, BallIntake_Result)() {
  return get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_Result>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_intake__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace tourobo_2026_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _BallIntake_Feedback_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _BallIntake_Feedback_type_support_ids_t;

static const _BallIntake_Feedback_type_support_ids_t _BallIntake_Feedback_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _BallIntake_Feedback_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _BallIntake_Feedback_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _BallIntake_Feedback_type_support_symbol_names_t _BallIntake_Feedback_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, tourobo_2026_interfaces, action, BallIntake_Feedback)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, tourobo_2026_interfaces, action, BallIntake_Feedback)),
  }
};

typedef struct _BallIntake_Feedback_type_support_data_t
{
  void * data[2];
} _BallIntake_Feedback_type_support_data_t;

static _BallIntake_Feedback_type_support_data_t _BallIntake_Feedback_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _BallIntake_Feedback_message_typesupport_map = {
  2,
  "tourobo_2026_interfaces",
  &_BallIntake_Feedback_message_typesupport_ids.typesupport_identifier[0],
  &_BallIntake_Feedback_message_typesupport_symbol_names.symbol_name[0],
  &_BallIntake_Feedback_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t BallIntake_Feedback_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_BallIntake_Feedback_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_Feedback>()
{
  return &::tourobo_2026_interfaces::action::rosidl_typesupport_cpp::BallIntake_Feedback_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, tourobo_2026_interfaces, action, BallIntake_Feedback)() {
  return get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_Feedback>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_intake__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace tourobo_2026_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _BallIntake_SendGoal_Request_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _BallIntake_SendGoal_Request_type_support_ids_t;

static const _BallIntake_SendGoal_Request_type_support_ids_t _BallIntake_SendGoal_Request_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _BallIntake_SendGoal_Request_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _BallIntake_SendGoal_Request_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _BallIntake_SendGoal_Request_type_support_symbol_names_t _BallIntake_SendGoal_Request_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, tourobo_2026_interfaces, action, BallIntake_SendGoal_Request)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, tourobo_2026_interfaces, action, BallIntake_SendGoal_Request)),
  }
};

typedef struct _BallIntake_SendGoal_Request_type_support_data_t
{
  void * data[2];
} _BallIntake_SendGoal_Request_type_support_data_t;

static _BallIntake_SendGoal_Request_type_support_data_t _BallIntake_SendGoal_Request_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _BallIntake_SendGoal_Request_message_typesupport_map = {
  2,
  "tourobo_2026_interfaces",
  &_BallIntake_SendGoal_Request_message_typesupport_ids.typesupport_identifier[0],
  &_BallIntake_SendGoal_Request_message_typesupport_symbol_names.symbol_name[0],
  &_BallIntake_SendGoal_Request_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t BallIntake_SendGoal_Request_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_BallIntake_SendGoal_Request_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_SendGoal_Request>()
{
  return &::tourobo_2026_interfaces::action::rosidl_typesupport_cpp::BallIntake_SendGoal_Request_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, tourobo_2026_interfaces, action, BallIntake_SendGoal_Request)() {
  return get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_SendGoal_Request>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_intake__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace tourobo_2026_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _BallIntake_SendGoal_Response_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _BallIntake_SendGoal_Response_type_support_ids_t;

static const _BallIntake_SendGoal_Response_type_support_ids_t _BallIntake_SendGoal_Response_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _BallIntake_SendGoal_Response_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _BallIntake_SendGoal_Response_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _BallIntake_SendGoal_Response_type_support_symbol_names_t _BallIntake_SendGoal_Response_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, tourobo_2026_interfaces, action, BallIntake_SendGoal_Response)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, tourobo_2026_interfaces, action, BallIntake_SendGoal_Response)),
  }
};

typedef struct _BallIntake_SendGoal_Response_type_support_data_t
{
  void * data[2];
} _BallIntake_SendGoal_Response_type_support_data_t;

static _BallIntake_SendGoal_Response_type_support_data_t _BallIntake_SendGoal_Response_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _BallIntake_SendGoal_Response_message_typesupport_map = {
  2,
  "tourobo_2026_interfaces",
  &_BallIntake_SendGoal_Response_message_typesupport_ids.typesupport_identifier[0],
  &_BallIntake_SendGoal_Response_message_typesupport_symbol_names.symbol_name[0],
  &_BallIntake_SendGoal_Response_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t BallIntake_SendGoal_Response_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_BallIntake_SendGoal_Response_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_SendGoal_Response>()
{
  return &::tourobo_2026_interfaces::action::rosidl_typesupport_cpp::BallIntake_SendGoal_Response_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, tourobo_2026_interfaces, action, BallIntake_SendGoal_Response)() {
  return get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_SendGoal_Response>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
#include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_intake__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
#include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
#include "rosidl_typesupport_cpp/service_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace tourobo_2026_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _BallIntake_SendGoal_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _BallIntake_SendGoal_type_support_ids_t;

static const _BallIntake_SendGoal_type_support_ids_t _BallIntake_SendGoal_service_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _BallIntake_SendGoal_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _BallIntake_SendGoal_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _BallIntake_SendGoal_type_support_symbol_names_t _BallIntake_SendGoal_service_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, tourobo_2026_interfaces, action, BallIntake_SendGoal)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, tourobo_2026_interfaces, action, BallIntake_SendGoal)),
  }
};

typedef struct _BallIntake_SendGoal_type_support_data_t
{
  void * data[2];
} _BallIntake_SendGoal_type_support_data_t;

static _BallIntake_SendGoal_type_support_data_t _BallIntake_SendGoal_service_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _BallIntake_SendGoal_service_typesupport_map = {
  2,
  "tourobo_2026_interfaces",
  &_BallIntake_SendGoal_service_typesupport_ids.typesupport_identifier[0],
  &_BallIntake_SendGoal_service_typesupport_symbol_names.symbol_name[0],
  &_BallIntake_SendGoal_service_typesupport_data.data[0],
};

static const rosidl_service_type_support_t BallIntake_SendGoal_service_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_BallIntake_SendGoal_service_typesupport_map),
  ::rosidl_typesupport_cpp::get_service_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
get_service_type_support_handle<tourobo_2026_interfaces::action::BallIntake_SendGoal>()
{
  return &::tourobo_2026_interfaces::action::rosidl_typesupport_cpp::BallIntake_SendGoal_service_type_support_handle;
}

}  // namespace rosidl_typesupport_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_cpp, tourobo_2026_interfaces, action, BallIntake_SendGoal)() {
  return ::rosidl_typesupport_cpp::get_service_type_support_handle<tourobo_2026_interfaces::action::BallIntake_SendGoal>();
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_intake__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace tourobo_2026_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _BallIntake_GetResult_Request_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _BallIntake_GetResult_Request_type_support_ids_t;

static const _BallIntake_GetResult_Request_type_support_ids_t _BallIntake_GetResult_Request_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _BallIntake_GetResult_Request_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _BallIntake_GetResult_Request_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _BallIntake_GetResult_Request_type_support_symbol_names_t _BallIntake_GetResult_Request_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, tourobo_2026_interfaces, action, BallIntake_GetResult_Request)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, tourobo_2026_interfaces, action, BallIntake_GetResult_Request)),
  }
};

typedef struct _BallIntake_GetResult_Request_type_support_data_t
{
  void * data[2];
} _BallIntake_GetResult_Request_type_support_data_t;

static _BallIntake_GetResult_Request_type_support_data_t _BallIntake_GetResult_Request_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _BallIntake_GetResult_Request_message_typesupport_map = {
  2,
  "tourobo_2026_interfaces",
  &_BallIntake_GetResult_Request_message_typesupport_ids.typesupport_identifier[0],
  &_BallIntake_GetResult_Request_message_typesupport_symbol_names.symbol_name[0],
  &_BallIntake_GetResult_Request_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t BallIntake_GetResult_Request_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_BallIntake_GetResult_Request_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_GetResult_Request>()
{
  return &::tourobo_2026_interfaces::action::rosidl_typesupport_cpp::BallIntake_GetResult_Request_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, tourobo_2026_interfaces, action, BallIntake_GetResult_Request)() {
  return get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_GetResult_Request>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_intake__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace tourobo_2026_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _BallIntake_GetResult_Response_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _BallIntake_GetResult_Response_type_support_ids_t;

static const _BallIntake_GetResult_Response_type_support_ids_t _BallIntake_GetResult_Response_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _BallIntake_GetResult_Response_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _BallIntake_GetResult_Response_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _BallIntake_GetResult_Response_type_support_symbol_names_t _BallIntake_GetResult_Response_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, tourobo_2026_interfaces, action, BallIntake_GetResult_Response)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, tourobo_2026_interfaces, action, BallIntake_GetResult_Response)),
  }
};

typedef struct _BallIntake_GetResult_Response_type_support_data_t
{
  void * data[2];
} _BallIntake_GetResult_Response_type_support_data_t;

static _BallIntake_GetResult_Response_type_support_data_t _BallIntake_GetResult_Response_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _BallIntake_GetResult_Response_message_typesupport_map = {
  2,
  "tourobo_2026_interfaces",
  &_BallIntake_GetResult_Response_message_typesupport_ids.typesupport_identifier[0],
  &_BallIntake_GetResult_Response_message_typesupport_symbol_names.symbol_name[0],
  &_BallIntake_GetResult_Response_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t BallIntake_GetResult_Response_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_BallIntake_GetResult_Response_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_GetResult_Response>()
{
  return &::tourobo_2026_interfaces::action::rosidl_typesupport_cpp::BallIntake_GetResult_Response_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, tourobo_2026_interfaces, action, BallIntake_GetResult_Response)() {
  return get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_GetResult_Response>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/service_type_support_struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_intake__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/service_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/service_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace tourobo_2026_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _BallIntake_GetResult_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _BallIntake_GetResult_type_support_ids_t;

static const _BallIntake_GetResult_type_support_ids_t _BallIntake_GetResult_service_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _BallIntake_GetResult_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _BallIntake_GetResult_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _BallIntake_GetResult_type_support_symbol_names_t _BallIntake_GetResult_service_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, tourobo_2026_interfaces, action, BallIntake_GetResult)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, tourobo_2026_interfaces, action, BallIntake_GetResult)),
  }
};

typedef struct _BallIntake_GetResult_type_support_data_t
{
  void * data[2];
} _BallIntake_GetResult_type_support_data_t;

static _BallIntake_GetResult_type_support_data_t _BallIntake_GetResult_service_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _BallIntake_GetResult_service_typesupport_map = {
  2,
  "tourobo_2026_interfaces",
  &_BallIntake_GetResult_service_typesupport_ids.typesupport_identifier[0],
  &_BallIntake_GetResult_service_typesupport_symbol_names.symbol_name[0],
  &_BallIntake_GetResult_service_typesupport_data.data[0],
};

static const rosidl_service_type_support_t BallIntake_GetResult_service_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_BallIntake_GetResult_service_typesupport_map),
  ::rosidl_typesupport_cpp::get_service_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
get_service_type_support_handle<tourobo_2026_interfaces::action::BallIntake_GetResult>()
{
  return &::tourobo_2026_interfaces::action::rosidl_typesupport_cpp::BallIntake_GetResult_service_type_support_handle;
}

}  // namespace rosidl_typesupport_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_service_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__SERVICE_SYMBOL_NAME(rosidl_typesupport_cpp, tourobo_2026_interfaces, action, BallIntake_GetResult)() {
  return ::rosidl_typesupport_cpp::get_service_type_support_handle<tourobo_2026_interfaces::action::BallIntake_GetResult>();
}

#ifdef __cplusplus
}
#endif

// already included above
// #include "cstddef"
// already included above
// #include "rosidl_runtime_c/message_type_support_struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_intake__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/identifier.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_c/type_support_map.h"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support_dispatch.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
// already included above
// #include "rosidl_typesupport_interface/macros.h"

namespace tourobo_2026_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

typedef struct _BallIntake_FeedbackMessage_type_support_ids_t
{
  const char * typesupport_identifier[2];
} _BallIntake_FeedbackMessage_type_support_ids_t;

static const _BallIntake_FeedbackMessage_type_support_ids_t _BallIntake_FeedbackMessage_message_typesupport_ids = {
  {
    "rosidl_typesupport_fastrtps_cpp",  // ::rosidl_typesupport_fastrtps_cpp::typesupport_identifier,
    "rosidl_typesupport_introspection_cpp",  // ::rosidl_typesupport_introspection_cpp::typesupport_identifier,
  }
};

typedef struct _BallIntake_FeedbackMessage_type_support_symbol_names_t
{
  const char * symbol_name[2];
} _BallIntake_FeedbackMessage_type_support_symbol_names_t;

#define STRINGIFY_(s) #s
#define STRINGIFY(s) STRINGIFY_(s)

static const _BallIntake_FeedbackMessage_type_support_symbol_names_t _BallIntake_FeedbackMessage_message_typesupport_symbol_names = {
  {
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_fastrtps_cpp, tourobo_2026_interfaces, action, BallIntake_FeedbackMessage)),
    STRINGIFY(ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_introspection_cpp, tourobo_2026_interfaces, action, BallIntake_FeedbackMessage)),
  }
};

typedef struct _BallIntake_FeedbackMessage_type_support_data_t
{
  void * data[2];
} _BallIntake_FeedbackMessage_type_support_data_t;

static _BallIntake_FeedbackMessage_type_support_data_t _BallIntake_FeedbackMessage_message_typesupport_data = {
  {
    0,  // will store the shared library later
    0,  // will store the shared library later
  }
};

static const type_support_map_t _BallIntake_FeedbackMessage_message_typesupport_map = {
  2,
  "tourobo_2026_interfaces",
  &_BallIntake_FeedbackMessage_message_typesupport_ids.typesupport_identifier[0],
  &_BallIntake_FeedbackMessage_message_typesupport_symbol_names.symbol_name[0],
  &_BallIntake_FeedbackMessage_message_typesupport_data.data[0],
};

static const rosidl_message_type_support_t BallIntake_FeedbackMessage_message_type_support_handle = {
  ::rosidl_typesupport_cpp::typesupport_identifier,
  reinterpret_cast<const type_support_map_t *>(&_BallIntake_FeedbackMessage_message_typesupport_map),
  ::rosidl_typesupport_cpp::get_message_typesupport_handle_function,
};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_FeedbackMessage>()
{
  return &::tourobo_2026_interfaces::action::rosidl_typesupport_cpp::BallIntake_FeedbackMessage_message_type_support_handle;
}

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_message_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__MESSAGE_SYMBOL_NAME(rosidl_typesupport_cpp, tourobo_2026_interfaces, action, BallIntake_FeedbackMessage)() {
  return get_message_type_support_handle<tourobo_2026_interfaces::action::BallIntake_FeedbackMessage>();
}

#ifdef __cplusplus
}
#endif
}  // namespace rosidl_typesupport_cpp

#include "action_msgs/msg/goal_status_array.hpp"
#include "action_msgs/srv/cancel_goal.hpp"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_intake__struct.hpp"
// already included above
// #include "rosidl_typesupport_cpp/visibility_control.h"
#include "rosidl_runtime_c/action_type_support_struct.h"
#include "rosidl_typesupport_cpp/action_type_support.hpp"
// already included above
// #include "rosidl_typesupport_cpp/message_type_support.hpp"
// already included above
// #include "rosidl_typesupport_cpp/service_type_support.hpp"

namespace tourobo_2026_interfaces
{

namespace action
{

namespace rosidl_typesupport_cpp
{

static rosidl_action_type_support_t BallIntake_action_type_support_handle = {
  NULL, NULL, NULL, NULL, NULL};

}  // namespace rosidl_typesupport_cpp

}  // namespace action

}  // namespace tourobo_2026_interfaces

namespace rosidl_typesupport_cpp
{

template<>
ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_action_type_support_t *
get_action_type_support_handle<tourobo_2026_interfaces::action::BallIntake>()
{
  using ::tourobo_2026_interfaces::action::rosidl_typesupport_cpp::BallIntake_action_type_support_handle;
  // Thread-safe by always writing the same values to the static struct
  BallIntake_action_type_support_handle.goal_service_type_support = get_service_type_support_handle<::tourobo_2026_interfaces::action::BallIntake::Impl::SendGoalService>();
  BallIntake_action_type_support_handle.result_service_type_support = get_service_type_support_handle<::tourobo_2026_interfaces::action::BallIntake::Impl::GetResultService>();
  BallIntake_action_type_support_handle.cancel_service_type_support = get_service_type_support_handle<::tourobo_2026_interfaces::action::BallIntake::Impl::CancelGoalService>();
  BallIntake_action_type_support_handle.feedback_message_type_support = get_message_type_support_handle<::tourobo_2026_interfaces::action::BallIntake::Impl::FeedbackMessage>();
  BallIntake_action_type_support_handle.status_message_type_support = get_message_type_support_handle<::tourobo_2026_interfaces::action::BallIntake::Impl::GoalStatusMessage>();
  return &BallIntake_action_type_support_handle;
}

}  // namespace rosidl_typesupport_cpp

#ifdef __cplusplus
extern "C"
{
#endif

ROSIDL_TYPESUPPORT_CPP_PUBLIC
const rosidl_action_type_support_t *
ROSIDL_TYPESUPPORT_INTERFACE__ACTION_SYMBOL_NAME(rosidl_typesupport_cpp, tourobo_2026_interfaces, action, BallIntake)() {
  return ::rosidl_typesupport_cpp::get_action_type_support_handle<tourobo_2026_interfaces::action::BallIntake>();
}

#ifdef __cplusplus
}
#endif
