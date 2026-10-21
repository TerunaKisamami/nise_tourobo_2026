// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from tourobo_2026_interfaces:action/BallPutPlate.idl
// generated code does not contain a copyright notice
#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
#include <Python.h>
#include <stdbool.h>
#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-function"
#endif
#include "numpy/ndarrayobject.h"
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif
#include "rosidl_runtime_c/visibility_control.h"
#include "tourobo_2026_interfaces/action/detail/ball_put_plate__struct.h"
#include "tourobo_2026_interfaces/action/detail/ball_put_plate__functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool tourobo_2026_interfaces__action__ball_put_plate__goal__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[65];
    {
      char * class_name = NULL;
      char * module_name = NULL;
      {
        PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
        if (class_attr) {
          PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
          if (name_attr) {
            class_name = (char *)PyUnicode_1BYTE_DATA(name_attr);
            Py_DECREF(name_attr);
          }
          PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
          if (module_attr) {
            module_name = (char *)PyUnicode_1BYTE_DATA(module_attr);
            Py_DECREF(module_attr);
          }
          Py_DECREF(class_attr);
        }
      }
      if (!class_name || !module_name) {
        return false;
      }
      snprintf(full_classname_dest, sizeof(full_classname_dest), "%s.%s", module_name, class_name);
    }
    assert(strncmp("tourobo_2026_interfaces.action._ball_put_plate.BallPutPlate_Goal", full_classname_dest, 64) == 0);
  }
  tourobo_2026_interfaces__action__BallPutPlate_Goal * ros_message = _ros_message;
  {  // execute
    PyObject * field = PyObject_GetAttrString(_pymsg, "execute");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->execute = (Py_True == field);
    Py_DECREF(field);
  }
  {  // current_state
    PyObject * field = PyObject_GetAttrString(_pymsg, "current_state");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->current_state = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // carry
    PyObject * field = PyObject_GetAttrString(_pymsg, "carry");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->carry = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // push_state
    PyObject * field = PyObject_GetAttrString(_pymsg, "push_state");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->push_state = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // shoot_angle_state
    PyObject * field = PyObject_GetAttrString(_pymsg, "shoot_angle_state");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->shoot_angle_state = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // is_left_arm_open
    PyObject * field = PyObject_GetAttrString(_pymsg, "is_left_arm_open");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->is_left_arm_open = (Py_True == field);
    Py_DECREF(field);
  }
  {  // is_right_arm_open
    PyObject * field = PyObject_GetAttrString(_pymsg, "is_right_arm_open");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->is_right_arm_open = (Py_True == field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * tourobo_2026_interfaces__action__ball_put_plate__goal__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of BallPutPlate_Goal */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("tourobo_2026_interfaces.action._ball_put_plate");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "BallPutPlate_Goal");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  tourobo_2026_interfaces__action__BallPutPlate_Goal * ros_message = (tourobo_2026_interfaces__action__BallPutPlate_Goal *)raw_ros_message;
  {  // execute
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->execute ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "execute", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // current_state
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->current_state);
    {
      int rc = PyObject_SetAttrString(_pymessage, "current_state", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // carry
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->carry);
    {
      int rc = PyObject_SetAttrString(_pymessage, "carry", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // push_state
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->push_state);
    {
      int rc = PyObject_SetAttrString(_pymessage, "push_state", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // shoot_angle_state
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->shoot_angle_state);
    {
      int rc = PyObject_SetAttrString(_pymessage, "shoot_angle_state", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // is_left_arm_open
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->is_left_arm_open ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "is_left_arm_open", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // is_right_arm_open
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->is_right_arm_open ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "is_right_arm_open", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}

#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
// already included above
// #include <Python.h>
// already included above
// #include <stdbool.h>
// already included above
// #include "numpy/ndarrayobject.h"
// already included above
// #include "rosidl_runtime_c/visibility_control.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_plate__struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_plate__functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool tourobo_2026_interfaces__action__ball_put_plate__result__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[67];
    {
      char * class_name = NULL;
      char * module_name = NULL;
      {
        PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
        if (class_attr) {
          PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
          if (name_attr) {
            class_name = (char *)PyUnicode_1BYTE_DATA(name_attr);
            Py_DECREF(name_attr);
          }
          PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
          if (module_attr) {
            module_name = (char *)PyUnicode_1BYTE_DATA(module_attr);
            Py_DECREF(module_attr);
          }
          Py_DECREF(class_attr);
        }
      }
      if (!class_name || !module_name) {
        return false;
      }
      snprintf(full_classname_dest, sizeof(full_classname_dest), "%s.%s", module_name, class_name);
    }
    assert(strncmp("tourobo_2026_interfaces.action._ball_put_plate.BallPutPlate_Result", full_classname_dest, 66) == 0);
  }
  tourobo_2026_interfaces__action__BallPutPlate_Result * ros_message = _ros_message;
  {  // success
    PyObject * field = PyObject_GetAttrString(_pymsg, "success");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->success = (Py_True == field);
    Py_DECREF(field);
  }
  {  // next_state
    PyObject * field = PyObject_GetAttrString(_pymsg, "next_state");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->next_state = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // next_carry
    PyObject * field = PyObject_GetAttrString(_pymsg, "next_carry");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->next_carry = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // next_push_state
    PyObject * field = PyObject_GetAttrString(_pymsg, "next_push_state");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->next_push_state = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // next_shoot_angle_state
    PyObject * field = PyObject_GetAttrString(_pymsg, "next_shoot_angle_state");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->next_shoot_angle_state = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // next_is_left_arm_open
    PyObject * field = PyObject_GetAttrString(_pymsg, "next_is_left_arm_open");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->next_is_left_arm_open = (Py_True == field);
    Py_DECREF(field);
  }
  {  // next_is_right_arm_open
    PyObject * field = PyObject_GetAttrString(_pymsg, "next_is_right_arm_open");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->next_is_right_arm_open = (Py_True == field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * tourobo_2026_interfaces__action__ball_put_plate__result__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of BallPutPlate_Result */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("tourobo_2026_interfaces.action._ball_put_plate");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "BallPutPlate_Result");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  tourobo_2026_interfaces__action__BallPutPlate_Result * ros_message = (tourobo_2026_interfaces__action__BallPutPlate_Result *)raw_ros_message;
  {  // success
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->success ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "success", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // next_state
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->next_state);
    {
      int rc = PyObject_SetAttrString(_pymessage, "next_state", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // next_carry
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->next_carry);
    {
      int rc = PyObject_SetAttrString(_pymessage, "next_carry", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // next_push_state
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->next_push_state);
    {
      int rc = PyObject_SetAttrString(_pymessage, "next_push_state", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // next_shoot_angle_state
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->next_shoot_angle_state);
    {
      int rc = PyObject_SetAttrString(_pymessage, "next_shoot_angle_state", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // next_is_left_arm_open
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->next_is_left_arm_open ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "next_is_left_arm_open", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // next_is_right_arm_open
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->next_is_right_arm_open ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "next_is_right_arm_open", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}

#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
// already included above
// #include <Python.h>
// already included above
// #include <stdbool.h>
// already included above
// #include "numpy/ndarrayobject.h"
// already included above
// #include "rosidl_runtime_c/visibility_control.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_plate__struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_plate__functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool tourobo_2026_interfaces__action__ball_put_plate__feedback__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[69];
    {
      char * class_name = NULL;
      char * module_name = NULL;
      {
        PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
        if (class_attr) {
          PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
          if (name_attr) {
            class_name = (char *)PyUnicode_1BYTE_DATA(name_attr);
            Py_DECREF(name_attr);
          }
          PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
          if (module_attr) {
            module_name = (char *)PyUnicode_1BYTE_DATA(module_attr);
            Py_DECREF(module_attr);
          }
          Py_DECREF(class_attr);
        }
      }
      if (!class_name || !module_name) {
        return false;
      }
      snprintf(full_classname_dest, sizeof(full_classname_dest), "%s.%s", module_name, class_name);
    }
    assert(strncmp("tourobo_2026_interfaces.action._ball_put_plate.BallPutPlate_Feedback", full_classname_dest, 68) == 0);
  }
  tourobo_2026_interfaces__action__BallPutPlate_Feedback * ros_message = _ros_message;
  ros_message->structure_needs_at_least_one_member = 0;

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * tourobo_2026_interfaces__action__ball_put_plate__feedback__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of BallPutPlate_Feedback */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("tourobo_2026_interfaces.action._ball_put_plate");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "BallPutPlate_Feedback");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  (void)raw_ros_message;

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}

#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
// already included above
// #include <Python.h>
// already included above
// #include <stdbool.h>
// already included above
// #include "numpy/ndarrayobject.h"
// already included above
// #include "rosidl_runtime_c/visibility_control.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_plate__struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_plate__functions.h"

ROSIDL_GENERATOR_C_IMPORT
bool unique_identifier_msgs__msg__uuid__convert_from_py(PyObject * _pymsg, void * _ros_message);
ROSIDL_GENERATOR_C_IMPORT
PyObject * unique_identifier_msgs__msg__uuid__convert_to_py(void * raw_ros_message);
bool tourobo_2026_interfaces__action__ball_put_plate__goal__convert_from_py(PyObject * _pymsg, void * _ros_message);
PyObject * tourobo_2026_interfaces__action__ball_put_plate__goal__convert_to_py(void * raw_ros_message);

ROSIDL_GENERATOR_C_EXPORT
bool tourobo_2026_interfaces__action__ball_put_plate__send_goal__request__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[77];
    {
      char * class_name = NULL;
      char * module_name = NULL;
      {
        PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
        if (class_attr) {
          PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
          if (name_attr) {
            class_name = (char *)PyUnicode_1BYTE_DATA(name_attr);
            Py_DECREF(name_attr);
          }
          PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
          if (module_attr) {
            module_name = (char *)PyUnicode_1BYTE_DATA(module_attr);
            Py_DECREF(module_attr);
          }
          Py_DECREF(class_attr);
        }
      }
      if (!class_name || !module_name) {
        return false;
      }
      snprintf(full_classname_dest, sizeof(full_classname_dest), "%s.%s", module_name, class_name);
    }
    assert(strncmp("tourobo_2026_interfaces.action._ball_put_plate.BallPutPlate_SendGoal_Request", full_classname_dest, 76) == 0);
  }
  tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request * ros_message = _ros_message;
  {  // goal_id
    PyObject * field = PyObject_GetAttrString(_pymsg, "goal_id");
    if (!field) {
      return false;
    }
    if (!unique_identifier_msgs__msg__uuid__convert_from_py(field, &ros_message->goal_id)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }
  {  // goal
    PyObject * field = PyObject_GetAttrString(_pymsg, "goal");
    if (!field) {
      return false;
    }
    if (!tourobo_2026_interfaces__action__ball_put_plate__goal__convert_from_py(field, &ros_message->goal)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * tourobo_2026_interfaces__action__ball_put_plate__send_goal__request__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of BallPutPlate_SendGoal_Request */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("tourobo_2026_interfaces.action._ball_put_plate");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "BallPutPlate_SendGoal_Request");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request * ros_message = (tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request *)raw_ros_message;
  {  // goal_id
    PyObject * field = NULL;
    field = unique_identifier_msgs__msg__uuid__convert_to_py(&ros_message->goal_id);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "goal_id", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // goal
    PyObject * field = NULL;
    field = tourobo_2026_interfaces__action__ball_put_plate__goal__convert_to_py(&ros_message->goal);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "goal", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}

#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
// already included above
// #include <Python.h>
// already included above
// #include <stdbool.h>
// already included above
// #include "numpy/ndarrayobject.h"
// already included above
// #include "rosidl_runtime_c/visibility_control.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_plate__struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_plate__functions.h"

ROSIDL_GENERATOR_C_IMPORT
bool builtin_interfaces__msg__time__convert_from_py(PyObject * _pymsg, void * _ros_message);
ROSIDL_GENERATOR_C_IMPORT
PyObject * builtin_interfaces__msg__time__convert_to_py(void * raw_ros_message);

ROSIDL_GENERATOR_C_EXPORT
bool tourobo_2026_interfaces__action__ball_put_plate__send_goal__response__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[78];
    {
      char * class_name = NULL;
      char * module_name = NULL;
      {
        PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
        if (class_attr) {
          PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
          if (name_attr) {
            class_name = (char *)PyUnicode_1BYTE_DATA(name_attr);
            Py_DECREF(name_attr);
          }
          PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
          if (module_attr) {
            module_name = (char *)PyUnicode_1BYTE_DATA(module_attr);
            Py_DECREF(module_attr);
          }
          Py_DECREF(class_attr);
        }
      }
      if (!class_name || !module_name) {
        return false;
      }
      snprintf(full_classname_dest, sizeof(full_classname_dest), "%s.%s", module_name, class_name);
    }
    assert(strncmp("tourobo_2026_interfaces.action._ball_put_plate.BallPutPlate_SendGoal_Response", full_classname_dest, 77) == 0);
  }
  tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response * ros_message = _ros_message;
  {  // accepted
    PyObject * field = PyObject_GetAttrString(_pymsg, "accepted");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->accepted = (Py_True == field);
    Py_DECREF(field);
  }
  {  // stamp
    PyObject * field = PyObject_GetAttrString(_pymsg, "stamp");
    if (!field) {
      return false;
    }
    if (!builtin_interfaces__msg__time__convert_from_py(field, &ros_message->stamp)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * tourobo_2026_interfaces__action__ball_put_plate__send_goal__response__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of BallPutPlate_SendGoal_Response */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("tourobo_2026_interfaces.action._ball_put_plate");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "BallPutPlate_SendGoal_Response");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response * ros_message = (tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response *)raw_ros_message;
  {  // accepted
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->accepted ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "accepted", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // stamp
    PyObject * field = NULL;
    field = builtin_interfaces__msg__time__convert_to_py(&ros_message->stamp);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "stamp", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}

#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
// already included above
// #include <Python.h>
// already included above
// #include <stdbool.h>
// already included above
// #include "numpy/ndarrayobject.h"
// already included above
// #include "rosidl_runtime_c/visibility_control.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_plate__struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_plate__functions.h"

ROSIDL_GENERATOR_C_IMPORT
bool unique_identifier_msgs__msg__uuid__convert_from_py(PyObject * _pymsg, void * _ros_message);
ROSIDL_GENERATOR_C_IMPORT
PyObject * unique_identifier_msgs__msg__uuid__convert_to_py(void * raw_ros_message);

ROSIDL_GENERATOR_C_EXPORT
bool tourobo_2026_interfaces__action__ball_put_plate__get_result__request__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[78];
    {
      char * class_name = NULL;
      char * module_name = NULL;
      {
        PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
        if (class_attr) {
          PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
          if (name_attr) {
            class_name = (char *)PyUnicode_1BYTE_DATA(name_attr);
            Py_DECREF(name_attr);
          }
          PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
          if (module_attr) {
            module_name = (char *)PyUnicode_1BYTE_DATA(module_attr);
            Py_DECREF(module_attr);
          }
          Py_DECREF(class_attr);
        }
      }
      if (!class_name || !module_name) {
        return false;
      }
      snprintf(full_classname_dest, sizeof(full_classname_dest), "%s.%s", module_name, class_name);
    }
    assert(strncmp("tourobo_2026_interfaces.action._ball_put_plate.BallPutPlate_GetResult_Request", full_classname_dest, 77) == 0);
  }
  tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request * ros_message = _ros_message;
  {  // goal_id
    PyObject * field = PyObject_GetAttrString(_pymsg, "goal_id");
    if (!field) {
      return false;
    }
    if (!unique_identifier_msgs__msg__uuid__convert_from_py(field, &ros_message->goal_id)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * tourobo_2026_interfaces__action__ball_put_plate__get_result__request__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of BallPutPlate_GetResult_Request */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("tourobo_2026_interfaces.action._ball_put_plate");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "BallPutPlate_GetResult_Request");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request * ros_message = (tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request *)raw_ros_message;
  {  // goal_id
    PyObject * field = NULL;
    field = unique_identifier_msgs__msg__uuid__convert_to_py(&ros_message->goal_id);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "goal_id", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}

#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
// already included above
// #include <Python.h>
// already included above
// #include <stdbool.h>
// already included above
// #include "numpy/ndarrayobject.h"
// already included above
// #include "rosidl_runtime_c/visibility_control.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_plate__struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_plate__functions.h"

bool tourobo_2026_interfaces__action__ball_put_plate__result__convert_from_py(PyObject * _pymsg, void * _ros_message);
PyObject * tourobo_2026_interfaces__action__ball_put_plate__result__convert_to_py(void * raw_ros_message);

ROSIDL_GENERATOR_C_EXPORT
bool tourobo_2026_interfaces__action__ball_put_plate__get_result__response__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[79];
    {
      char * class_name = NULL;
      char * module_name = NULL;
      {
        PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
        if (class_attr) {
          PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
          if (name_attr) {
            class_name = (char *)PyUnicode_1BYTE_DATA(name_attr);
            Py_DECREF(name_attr);
          }
          PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
          if (module_attr) {
            module_name = (char *)PyUnicode_1BYTE_DATA(module_attr);
            Py_DECREF(module_attr);
          }
          Py_DECREF(class_attr);
        }
      }
      if (!class_name || !module_name) {
        return false;
      }
      snprintf(full_classname_dest, sizeof(full_classname_dest), "%s.%s", module_name, class_name);
    }
    assert(strncmp("tourobo_2026_interfaces.action._ball_put_plate.BallPutPlate_GetResult_Response", full_classname_dest, 78) == 0);
  }
  tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response * ros_message = _ros_message;
  {  // status
    PyObject * field = PyObject_GetAttrString(_pymsg, "status");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->status = (int8_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // result
    PyObject * field = PyObject_GetAttrString(_pymsg, "result");
    if (!field) {
      return false;
    }
    if (!tourobo_2026_interfaces__action__ball_put_plate__result__convert_from_py(field, &ros_message->result)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * tourobo_2026_interfaces__action__ball_put_plate__get_result__response__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of BallPutPlate_GetResult_Response */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("tourobo_2026_interfaces.action._ball_put_plate");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "BallPutPlate_GetResult_Response");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response * ros_message = (tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response *)raw_ros_message;
  {  // status
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->status);
    {
      int rc = PyObject_SetAttrString(_pymessage, "status", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // result
    PyObject * field = NULL;
    field = tourobo_2026_interfaces__action__ball_put_plate__result__convert_to_py(&ros_message->result);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "result", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}

#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
// already included above
// #include <Python.h>
// already included above
// #include <stdbool.h>
// already included above
// #include "numpy/ndarrayobject.h"
// already included above
// #include "rosidl_runtime_c/visibility_control.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_plate__struct.h"
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_put_plate__functions.h"

ROSIDL_GENERATOR_C_IMPORT
bool unique_identifier_msgs__msg__uuid__convert_from_py(PyObject * _pymsg, void * _ros_message);
ROSIDL_GENERATOR_C_IMPORT
PyObject * unique_identifier_msgs__msg__uuid__convert_to_py(void * raw_ros_message);
bool tourobo_2026_interfaces__action__ball_put_plate__feedback__convert_from_py(PyObject * _pymsg, void * _ros_message);
PyObject * tourobo_2026_interfaces__action__ball_put_plate__feedback__convert_to_py(void * raw_ros_message);

ROSIDL_GENERATOR_C_EXPORT
bool tourobo_2026_interfaces__action__ball_put_plate__feedback_message__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[76];
    {
      char * class_name = NULL;
      char * module_name = NULL;
      {
        PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
        if (class_attr) {
          PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
          if (name_attr) {
            class_name = (char *)PyUnicode_1BYTE_DATA(name_attr);
            Py_DECREF(name_attr);
          }
          PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
          if (module_attr) {
            module_name = (char *)PyUnicode_1BYTE_DATA(module_attr);
            Py_DECREF(module_attr);
          }
          Py_DECREF(class_attr);
        }
      }
      if (!class_name || !module_name) {
        return false;
      }
      snprintf(full_classname_dest, sizeof(full_classname_dest), "%s.%s", module_name, class_name);
    }
    assert(strncmp("tourobo_2026_interfaces.action._ball_put_plate.BallPutPlate_FeedbackMessage", full_classname_dest, 75) == 0);
  }
  tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage * ros_message = _ros_message;
  {  // goal_id
    PyObject * field = PyObject_GetAttrString(_pymsg, "goal_id");
    if (!field) {
      return false;
    }
    if (!unique_identifier_msgs__msg__uuid__convert_from_py(field, &ros_message->goal_id)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }
  {  // feedback
    PyObject * field = PyObject_GetAttrString(_pymsg, "feedback");
    if (!field) {
      return false;
    }
    if (!tourobo_2026_interfaces__action__ball_put_plate__feedback__convert_from_py(field, &ros_message->feedback)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * tourobo_2026_interfaces__action__ball_put_plate__feedback_message__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of BallPutPlate_FeedbackMessage */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("tourobo_2026_interfaces.action._ball_put_plate");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "BallPutPlate_FeedbackMessage");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage * ros_message = (tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage *)raw_ros_message;
  {  // goal_id
    PyObject * field = NULL;
    field = unique_identifier_msgs__msg__uuid__convert_to_py(&ros_message->goal_id);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "goal_id", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // feedback
    PyObject * field = NULL;
    field = tourobo_2026_interfaces__action__ball_put_plate__feedback__convert_to_py(&ros_message->feedback);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "feedback", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}
