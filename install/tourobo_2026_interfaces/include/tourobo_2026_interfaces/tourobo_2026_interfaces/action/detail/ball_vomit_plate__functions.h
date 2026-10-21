// generated from rosidl_generator_c/resource/idl__functions.h.em
// with input from tourobo_2026_interfaces:action/BallVomitPlate.idl
// generated code does not contain a copyright notice

#ifndef TOUROBO_2026_INTERFACES__ACTION__DETAIL__BALL_VOMIT_PLATE__FUNCTIONS_H_
#define TOUROBO_2026_INTERFACES__ACTION__DETAIL__BALL_VOMIT_PLATE__FUNCTIONS_H_

#ifdef __cplusplus
extern "C"
{
#endif

#include <stdbool.h>
#include <stdlib.h>

#include "rosidl_runtime_c/visibility_control.h"
#include "tourobo_2026_interfaces/msg/rosidl_generator_c__visibility_control.h"

#include "tourobo_2026_interfaces/action/detail/ball_vomit_plate__struct.h"

/// Initialize action/BallVomitPlate message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * tourobo_2026_interfaces__action__BallVomitPlate_Goal
 * )) before or use
 * tourobo_2026_interfaces__action__BallVomitPlate_Goal__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Goal__init(tourobo_2026_interfaces__action__BallVomitPlate_Goal * msg);

/// Finalize action/BallVomitPlate message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_Goal__fini(tourobo_2026_interfaces__action__BallVomitPlate_Goal * msg);

/// Create action/BallVomitPlate message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Goal__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_Goal *
tourobo_2026_interfaces__action__BallVomitPlate_Goal__create();

/// Destroy action/BallVomitPlate message.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Goal__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_Goal__destroy(tourobo_2026_interfaces__action__BallVomitPlate_Goal * msg);

/// Check for action/BallVomitPlate message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Goal__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_Goal * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_Goal * rhs);

/// Copy a action/BallVomitPlate message.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source message pointer.
 * \param[out] output The target message pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer is null
 *   or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Goal__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_Goal * input,
  tourobo_2026_interfaces__action__BallVomitPlate_Goal * output);

/// Initialize array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the number of elements and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Goal__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence * array, size_t size);

/// Finalize array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Goal__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence * array);

/// Create array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the array and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__create(size_t size);

/// Destroy array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence * array);

/// Check for action/BallVomitPlate message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence * rhs);

/// Copy an array of action/BallVomitPlate messages.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source array pointer.
 * \param[out] output The target array pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer
 *   is null or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence * output);

/// Initialize action/BallVomitPlate message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * tourobo_2026_interfaces__action__BallVomitPlate_Result
 * )) before or use
 * tourobo_2026_interfaces__action__BallVomitPlate_Result__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Result__init(tourobo_2026_interfaces__action__BallVomitPlate_Result * msg);

/// Finalize action/BallVomitPlate message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_Result__fini(tourobo_2026_interfaces__action__BallVomitPlate_Result * msg);

/// Create action/BallVomitPlate message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Result__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_Result *
tourobo_2026_interfaces__action__BallVomitPlate_Result__create();

/// Destroy action/BallVomitPlate message.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Result__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_Result__destroy(tourobo_2026_interfaces__action__BallVomitPlate_Result * msg);

/// Check for action/BallVomitPlate message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Result__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_Result * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_Result * rhs);

/// Copy a action/BallVomitPlate message.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source message pointer.
 * \param[out] output The target message pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer is null
 *   or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Result__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_Result * input,
  tourobo_2026_interfaces__action__BallVomitPlate_Result * output);

/// Initialize array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the number of elements and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Result__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence * array, size_t size);

/// Finalize array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Result__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence * array);

/// Create array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the array and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__create(size_t size);

/// Destroy array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence * array);

/// Check for action/BallVomitPlate message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence * rhs);

/// Copy an array of action/BallVomitPlate messages.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source array pointer.
 * \param[out] output The target array pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer
 *   is null or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence * output);

/// Initialize action/BallVomitPlate message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * tourobo_2026_interfaces__action__BallVomitPlate_Feedback
 * )) before or use
 * tourobo_2026_interfaces__action__BallVomitPlate_Feedback__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__init(tourobo_2026_interfaces__action__BallVomitPlate_Feedback * msg);

/// Finalize action/BallVomitPlate message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__fini(tourobo_2026_interfaces__action__BallVomitPlate_Feedback * msg);

/// Create action/BallVomitPlate message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Feedback__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_Feedback *
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__create();

/// Destroy action/BallVomitPlate message.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Feedback__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__destroy(tourobo_2026_interfaces__action__BallVomitPlate_Feedback * msg);

/// Check for action/BallVomitPlate message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_Feedback * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_Feedback * rhs);

/// Copy a action/BallVomitPlate message.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source message pointer.
 * \param[out] output The target message pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer is null
 *   or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_Feedback * input,
  tourobo_2026_interfaces__action__BallVomitPlate_Feedback * output);

/// Initialize array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the number of elements and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Feedback__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence * array, size_t size);

/// Finalize array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Feedback__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence * array);

/// Create array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the array and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__create(size_t size);

/// Destroy array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence * array);

/// Check for action/BallVomitPlate message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence * rhs);

/// Copy an array of action/BallVomitPlate messages.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source array pointer.
 * \param[out] output The target array pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer
 *   is null or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence * output);

/// Initialize action/BallVomitPlate message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request
 * )) before or use
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__init(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * msg);

/// Finalize action/BallVomitPlate message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__fini(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * msg);

/// Create action/BallVomitPlate message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request *
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__create();

/// Destroy action/BallVomitPlate message.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__destroy(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * msg);

/// Check for action/BallVomitPlate message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * rhs);

/// Copy a action/BallVomitPlate message.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source message pointer.
 * \param[out] output The target message pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer is null
 *   or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * input,
  tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * output);

/// Initialize array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the number of elements and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence * array, size_t size);

/// Finalize array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence * array);

/// Create array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the array and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__create(size_t size);

/// Destroy array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence * array);

/// Check for action/BallVomitPlate message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence * rhs);

/// Copy an array of action/BallVomitPlate messages.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source array pointer.
 * \param[out] output The target array pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer
 *   is null or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence * output);

/// Initialize action/BallVomitPlate message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response
 * )) before or use
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__init(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * msg);

/// Finalize action/BallVomitPlate message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__fini(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * msg);

/// Create action/BallVomitPlate message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response *
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__create();

/// Destroy action/BallVomitPlate message.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__destroy(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * msg);

/// Check for action/BallVomitPlate message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * rhs);

/// Copy a action/BallVomitPlate message.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source message pointer.
 * \param[out] output The target message pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer is null
 *   or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * input,
  tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * output);

/// Initialize array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the number of elements and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence * array, size_t size);

/// Finalize array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence * array);

/// Create array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the array and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__create(size_t size);

/// Destroy array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence * array);

/// Check for action/BallVomitPlate message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence * rhs);

/// Copy an array of action/BallVomitPlate messages.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source array pointer.
 * \param[out] output The target array pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer
 *   is null or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence * output);

/// Initialize action/BallVomitPlate message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request
 * )) before or use
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__init(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * msg);

/// Finalize action/BallVomitPlate message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__fini(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * msg);

/// Create action/BallVomitPlate message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request *
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__create();

/// Destroy action/BallVomitPlate message.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__destroy(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * msg);

/// Check for action/BallVomitPlate message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * rhs);

/// Copy a action/BallVomitPlate message.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source message pointer.
 * \param[out] output The target message pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer is null
 *   or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * input,
  tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * output);

/// Initialize array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the number of elements and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence * array, size_t size);

/// Finalize array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence * array);

/// Create array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the array and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__create(size_t size);

/// Destroy array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence * array);

/// Check for action/BallVomitPlate message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence * rhs);

/// Copy an array of action/BallVomitPlate messages.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source array pointer.
 * \param[out] output The target array pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer
 *   is null or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence * output);

/// Initialize action/BallVomitPlate message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response
 * )) before or use
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__init(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * msg);

/// Finalize action/BallVomitPlate message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__fini(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * msg);

/// Create action/BallVomitPlate message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response *
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__create();

/// Destroy action/BallVomitPlate message.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__destroy(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * msg);

/// Check for action/BallVomitPlate message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * rhs);

/// Copy a action/BallVomitPlate message.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source message pointer.
 * \param[out] output The target message pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer is null
 *   or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * input,
  tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * output);

/// Initialize array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the number of elements and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence * array, size_t size);

/// Finalize array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence * array);

/// Create array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the array and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__create(size_t size);

/// Destroy array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence * array);

/// Check for action/BallVomitPlate message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence * rhs);

/// Copy an array of action/BallVomitPlate messages.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source array pointer.
 * \param[out] output The target array pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer
 *   is null or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence * output);

/// Initialize action/BallVomitPlate message.
/**
 * If the init function is called twice for the same message without
 * calling fini inbetween previously allocated memory will be leaked.
 * \param[in,out] msg The previously allocated message pointer.
 * Fields without a default value will not be initialized by this function.
 * You might want to call memset(msg, 0, sizeof(
 * tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage
 * )) before or use
 * tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__create()
 * to allocate and initialize the message.
 * \return true if initialization was successful, otherwise false
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__init(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * msg);

/// Finalize action/BallVomitPlate message.
/**
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__fini(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * msg);

/// Create action/BallVomitPlate message.
/**
 * It allocates the memory for the message, sets the memory to zero, and
 * calls
 * tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__init().
 * \return The pointer to the initialized message if successful,
 * otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage *
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__create();

/// Destroy action/BallVomitPlate message.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__fini()
 * and frees the memory of the message.
 * \param[in,out] msg The allocated message pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__destroy(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * msg);

/// Check for action/BallVomitPlate message equality.
/**
 * \param[in] lhs The message on the left hand size of the equality operator.
 * \param[in] rhs The message on the right hand size of the equality operator.
 * \return true if messages are equal, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * rhs);

/// Copy a action/BallVomitPlate message.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source message pointer.
 * \param[out] output The target message pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer is null
 *   or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * input,
  tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * output);

/// Initialize array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the number of elements and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__init()
 * for each element of the array.
 * \param[in,out] array The allocated array pointer.
 * \param[in] size The size / capacity of the array.
 * \return true if initialization was successful, otherwise false
 * If the array pointer is valid and the size is zero it is guaranteed
 # to return true.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence * array, size_t size);

/// Finalize array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__fini()
 * for each element of the array and frees the memory for the number of
 * elements.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence * array);

/// Create array of action/BallVomitPlate messages.
/**
 * It allocates the memory for the array and calls
 * tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__init().
 * \param[in] size The size / capacity of the array.
 * \return The pointer to the initialized array if successful, otherwise NULL
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__create(size_t size);

/// Destroy array of action/BallVomitPlate messages.
/**
 * It calls
 * tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__fini()
 * on the array,
 * and frees the memory of the array.
 * \param[in,out] array The initialized array pointer.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
void
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence * array);

/// Check for action/BallVomitPlate message array equality.
/**
 * \param[in] lhs The message array on the left hand size of the equality operator.
 * \param[in] rhs The message array on the right hand size of the equality operator.
 * \return true if message arrays are equal in size and content, otherwise false.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence * rhs);

/// Copy an array of action/BallVomitPlate messages.
/**
 * This functions performs a deep copy, as opposed to the shallow copy that
 * plain assignment yields.
 *
 * \param[in] input The source array pointer.
 * \param[out] output The target array pointer, which must
 *   have been initialized before calling this function.
 * \return true if successful, or false if either pointer
 *   is null or memory allocation fails.
 */
ROSIDL_GENERATOR_C_PUBLIC_tourobo_2026_interfaces
bool
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence * output);

#ifdef __cplusplus
}
#endif

#endif  // TOUROBO_2026_INTERFACES__ACTION__DETAIL__BALL_VOMIT_PLATE__FUNCTIONS_H_
