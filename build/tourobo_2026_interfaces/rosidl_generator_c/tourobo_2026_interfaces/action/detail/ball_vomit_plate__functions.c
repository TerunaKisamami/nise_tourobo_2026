// generated from rosidl_generator_c/resource/idl__functions.c.em
// with input from tourobo_2026_interfaces:action/BallVomitPlate.idl
// generated code does not contain a copyright notice
#include "tourobo_2026_interfaces/action/detail/ball_vomit_plate__functions.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "rcutils/allocator.h"


bool
tourobo_2026_interfaces__action__BallVomitPlate_Goal__init(tourobo_2026_interfaces__action__BallVomitPlate_Goal * msg)
{
  if (!msg) {
    return false;
  }
  // execute_mode
  // current_state
  // carry
  // push_state
  // shoot_angle_state
  // is_left_arm_open
  // is_right_arm_open
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_Goal__fini(tourobo_2026_interfaces__action__BallVomitPlate_Goal * msg)
{
  if (!msg) {
    return;
  }
  // execute_mode
  // current_state
  // carry
  // push_state
  // shoot_angle_state
  // is_left_arm_open
  // is_right_arm_open
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_Goal__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_Goal * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_Goal * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // execute_mode
  if (lhs->execute_mode != rhs->execute_mode) {
    return false;
  }
  // current_state
  if (lhs->current_state != rhs->current_state) {
    return false;
  }
  // carry
  if (lhs->carry != rhs->carry) {
    return false;
  }
  // push_state
  if (lhs->push_state != rhs->push_state) {
    return false;
  }
  // shoot_angle_state
  if (lhs->shoot_angle_state != rhs->shoot_angle_state) {
    return false;
  }
  // is_left_arm_open
  if (lhs->is_left_arm_open != rhs->is_left_arm_open) {
    return false;
  }
  // is_right_arm_open
  if (lhs->is_right_arm_open != rhs->is_right_arm_open) {
    return false;
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_Goal__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_Goal * input,
  tourobo_2026_interfaces__action__BallVomitPlate_Goal * output)
{
  if (!input || !output) {
    return false;
  }
  // execute_mode
  output->execute_mode = input->execute_mode;
  // current_state
  output->current_state = input->current_state;
  // carry
  output->carry = input->carry;
  // push_state
  output->push_state = input->push_state;
  // shoot_angle_state
  output->shoot_angle_state = input->shoot_angle_state;
  // is_left_arm_open
  output->is_left_arm_open = input->is_left_arm_open;
  // is_right_arm_open
  output->is_right_arm_open = input->is_right_arm_open;
  return true;
}

tourobo_2026_interfaces__action__BallVomitPlate_Goal *
tourobo_2026_interfaces__action__BallVomitPlate_Goal__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_Goal * msg = (tourobo_2026_interfaces__action__BallVomitPlate_Goal *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_Goal), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_Goal));
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_Goal__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_Goal__destroy(tourobo_2026_interfaces__action__BallVomitPlate_Goal * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    tourobo_2026_interfaces__action__BallVomitPlate_Goal__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_Goal * data = NULL;

  if (size) {
    data = (tourobo_2026_interfaces__action__BallVomitPlate_Goal *)allocator.zero_allocate(size, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_Goal), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = tourobo_2026_interfaces__action__BallVomitPlate_Goal__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        tourobo_2026_interfaces__action__BallVomitPlate_Goal__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      tourobo_2026_interfaces__action__BallVomitPlate_Goal__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence * array = (tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_Goal__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(tourobo_2026_interfaces__action__BallVomitPlate_Goal);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    tourobo_2026_interfaces__action__BallVomitPlate_Goal * data =
      (tourobo_2026_interfaces__action__BallVomitPlate_Goal *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!tourobo_2026_interfaces__action__BallVomitPlate_Goal__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          tourobo_2026_interfaces__action__BallVomitPlate_Goal__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_Goal__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


bool
tourobo_2026_interfaces__action__BallVomitPlate_Result__init(tourobo_2026_interfaces__action__BallVomitPlate_Result * msg)
{
  if (!msg) {
    return false;
  }
  // success
  // next_state
  // next_carry
  // next_push_state
  // next_shoot_angle_state
  // next_is_left_arm_open
  // next_is_right_arm_open
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_Result__fini(tourobo_2026_interfaces__action__BallVomitPlate_Result * msg)
{
  if (!msg) {
    return;
  }
  // success
  // next_state
  // next_carry
  // next_push_state
  // next_shoot_angle_state
  // next_is_left_arm_open
  // next_is_right_arm_open
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_Result__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_Result * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_Result * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // success
  if (lhs->success != rhs->success) {
    return false;
  }
  // next_state
  if (lhs->next_state != rhs->next_state) {
    return false;
  }
  // next_carry
  if (lhs->next_carry != rhs->next_carry) {
    return false;
  }
  // next_push_state
  if (lhs->next_push_state != rhs->next_push_state) {
    return false;
  }
  // next_shoot_angle_state
  if (lhs->next_shoot_angle_state != rhs->next_shoot_angle_state) {
    return false;
  }
  // next_is_left_arm_open
  if (lhs->next_is_left_arm_open != rhs->next_is_left_arm_open) {
    return false;
  }
  // next_is_right_arm_open
  if (lhs->next_is_right_arm_open != rhs->next_is_right_arm_open) {
    return false;
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_Result__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_Result * input,
  tourobo_2026_interfaces__action__BallVomitPlate_Result * output)
{
  if (!input || !output) {
    return false;
  }
  // success
  output->success = input->success;
  // next_state
  output->next_state = input->next_state;
  // next_carry
  output->next_carry = input->next_carry;
  // next_push_state
  output->next_push_state = input->next_push_state;
  // next_shoot_angle_state
  output->next_shoot_angle_state = input->next_shoot_angle_state;
  // next_is_left_arm_open
  output->next_is_left_arm_open = input->next_is_left_arm_open;
  // next_is_right_arm_open
  output->next_is_right_arm_open = input->next_is_right_arm_open;
  return true;
}

tourobo_2026_interfaces__action__BallVomitPlate_Result *
tourobo_2026_interfaces__action__BallVomitPlate_Result__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_Result * msg = (tourobo_2026_interfaces__action__BallVomitPlate_Result *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_Result), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_Result));
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_Result__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_Result__destroy(tourobo_2026_interfaces__action__BallVomitPlate_Result * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    tourobo_2026_interfaces__action__BallVomitPlate_Result__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_Result * data = NULL;

  if (size) {
    data = (tourobo_2026_interfaces__action__BallVomitPlate_Result *)allocator.zero_allocate(size, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_Result), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = tourobo_2026_interfaces__action__BallVomitPlate_Result__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        tourobo_2026_interfaces__action__BallVomitPlate_Result__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      tourobo_2026_interfaces__action__BallVomitPlate_Result__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence * array = (tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_Result__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(tourobo_2026_interfaces__action__BallVomitPlate_Result);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    tourobo_2026_interfaces__action__BallVomitPlate_Result * data =
      (tourobo_2026_interfaces__action__BallVomitPlate_Result *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!tourobo_2026_interfaces__action__BallVomitPlate_Result__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          tourobo_2026_interfaces__action__BallVomitPlate_Result__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_Result__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


bool
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__init(tourobo_2026_interfaces__action__BallVomitPlate_Feedback * msg)
{
  if (!msg) {
    return false;
  }
  // structure_needs_at_least_one_member
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__fini(tourobo_2026_interfaces__action__BallVomitPlate_Feedback * msg)
{
  if (!msg) {
    return;
  }
  // structure_needs_at_least_one_member
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_Feedback * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_Feedback * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // structure_needs_at_least_one_member
  if (lhs->structure_needs_at_least_one_member != rhs->structure_needs_at_least_one_member) {
    return false;
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_Feedback * input,
  tourobo_2026_interfaces__action__BallVomitPlate_Feedback * output)
{
  if (!input || !output) {
    return false;
  }
  // structure_needs_at_least_one_member
  output->structure_needs_at_least_one_member = input->structure_needs_at_least_one_member;
  return true;
}

tourobo_2026_interfaces__action__BallVomitPlate_Feedback *
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_Feedback * msg = (tourobo_2026_interfaces__action__BallVomitPlate_Feedback *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_Feedback), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_Feedback));
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_Feedback__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__destroy(tourobo_2026_interfaces__action__BallVomitPlate_Feedback * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    tourobo_2026_interfaces__action__BallVomitPlate_Feedback__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_Feedback * data = NULL;

  if (size) {
    data = (tourobo_2026_interfaces__action__BallVomitPlate_Feedback *)allocator.zero_allocate(size, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_Feedback), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = tourobo_2026_interfaces__action__BallVomitPlate_Feedback__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        tourobo_2026_interfaces__action__BallVomitPlate_Feedback__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      tourobo_2026_interfaces__action__BallVomitPlate_Feedback__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence * array = (tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_Feedback__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(tourobo_2026_interfaces__action__BallVomitPlate_Feedback);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    tourobo_2026_interfaces__action__BallVomitPlate_Feedback * data =
      (tourobo_2026_interfaces__action__BallVomitPlate_Feedback *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!tourobo_2026_interfaces__action__BallVomitPlate_Feedback__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          tourobo_2026_interfaces__action__BallVomitPlate_Feedback__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_Feedback__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


// Include directives for member types
// Member `goal_id`
#include "unique_identifier_msgs/msg/detail/uuid__functions.h"
// Member `goal`
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_vomit_plate__functions.h"

bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__init(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * msg)
{
  if (!msg) {
    return false;
  }
  // goal_id
  if (!unique_identifier_msgs__msg__UUID__init(&msg->goal_id)) {
    tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__fini(msg);
    return false;
  }
  // goal
  if (!tourobo_2026_interfaces__action__BallVomitPlate_Goal__init(&msg->goal)) {
    tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__fini(msg);
    return false;
  }
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__fini(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * msg)
{
  if (!msg) {
    return;
  }
  // goal_id
  unique_identifier_msgs__msg__UUID__fini(&msg->goal_id);
  // goal
  tourobo_2026_interfaces__action__BallVomitPlate_Goal__fini(&msg->goal);
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // goal_id
  if (!unique_identifier_msgs__msg__UUID__are_equal(
      &(lhs->goal_id), &(rhs->goal_id)))
  {
    return false;
  }
  // goal
  if (!tourobo_2026_interfaces__action__BallVomitPlate_Goal__are_equal(
      &(lhs->goal), &(rhs->goal)))
  {
    return false;
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * input,
  tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * output)
{
  if (!input || !output) {
    return false;
  }
  // goal_id
  if (!unique_identifier_msgs__msg__UUID__copy(
      &(input->goal_id), &(output->goal_id)))
  {
    return false;
  }
  // goal
  if (!tourobo_2026_interfaces__action__BallVomitPlate_Goal__copy(
      &(input->goal), &(output->goal)))
  {
    return false;
  }
  return true;
}

tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request *
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * msg = (tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request));
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__destroy(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * data = NULL;

  if (size) {
    data = (tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request *)allocator.zero_allocate(size, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence * array = (tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request * data =
      (tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


// Include directives for member types
// Member `stamp`
#include "builtin_interfaces/msg/detail/time__functions.h"

bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__init(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * msg)
{
  if (!msg) {
    return false;
  }
  // accepted
  // stamp
  if (!builtin_interfaces__msg__Time__init(&msg->stamp)) {
    tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__fini(msg);
    return false;
  }
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__fini(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * msg)
{
  if (!msg) {
    return;
  }
  // accepted
  // stamp
  builtin_interfaces__msg__Time__fini(&msg->stamp);
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // accepted
  if (lhs->accepted != rhs->accepted) {
    return false;
  }
  // stamp
  if (!builtin_interfaces__msg__Time__are_equal(
      &(lhs->stamp), &(rhs->stamp)))
  {
    return false;
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * input,
  tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * output)
{
  if (!input || !output) {
    return false;
  }
  // accepted
  output->accepted = input->accepted;
  // stamp
  if (!builtin_interfaces__msg__Time__copy(
      &(input->stamp), &(output->stamp)))
  {
    return false;
  }
  return true;
}

tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response *
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * msg = (tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response));
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__destroy(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * data = NULL;

  if (size) {
    data = (tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response *)allocator.zero_allocate(size, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence * array = (tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response * data =
      (tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


// Include directives for member types
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__functions.h"

bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__init(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * msg)
{
  if (!msg) {
    return false;
  }
  // goal_id
  if (!unique_identifier_msgs__msg__UUID__init(&msg->goal_id)) {
    tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__fini(msg);
    return false;
  }
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__fini(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * msg)
{
  if (!msg) {
    return;
  }
  // goal_id
  unique_identifier_msgs__msg__UUID__fini(&msg->goal_id);
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // goal_id
  if (!unique_identifier_msgs__msg__UUID__are_equal(
      &(lhs->goal_id), &(rhs->goal_id)))
  {
    return false;
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * input,
  tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * output)
{
  if (!input || !output) {
    return false;
  }
  // goal_id
  if (!unique_identifier_msgs__msg__UUID__copy(
      &(input->goal_id), &(output->goal_id)))
  {
    return false;
  }
  return true;
}

tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request *
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * msg = (tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request));
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__destroy(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * data = NULL;

  if (size) {
    data = (tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request *)allocator.zero_allocate(size, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence * array = (tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request * data =
      (tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


// Include directives for member types
// Member `result`
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_vomit_plate__functions.h"

bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__init(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * msg)
{
  if (!msg) {
    return false;
  }
  // status
  // result
  if (!tourobo_2026_interfaces__action__BallVomitPlate_Result__init(&msg->result)) {
    tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__fini(msg);
    return false;
  }
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__fini(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * msg)
{
  if (!msg) {
    return;
  }
  // status
  // result
  tourobo_2026_interfaces__action__BallVomitPlate_Result__fini(&msg->result);
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // status
  if (lhs->status != rhs->status) {
    return false;
  }
  // result
  if (!tourobo_2026_interfaces__action__BallVomitPlate_Result__are_equal(
      &(lhs->result), &(rhs->result)))
  {
    return false;
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * input,
  tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * output)
{
  if (!input || !output) {
    return false;
  }
  // status
  output->status = input->status;
  // result
  if (!tourobo_2026_interfaces__action__BallVomitPlate_Result__copy(
      &(input->result), &(output->result)))
  {
    return false;
  }
  return true;
}

tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response *
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * msg = (tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response));
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__destroy(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * data = NULL;

  if (size) {
    data = (tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response *)allocator.zero_allocate(size, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence * array = (tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response * data =
      (tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}


// Include directives for member types
// Member `goal_id`
// already included above
// #include "unique_identifier_msgs/msg/detail/uuid__functions.h"
// Member `feedback`
// already included above
// #include "tourobo_2026_interfaces/action/detail/ball_vomit_plate__functions.h"

bool
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__init(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * msg)
{
  if (!msg) {
    return false;
  }
  // goal_id
  if (!unique_identifier_msgs__msg__UUID__init(&msg->goal_id)) {
    tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__fini(msg);
    return false;
  }
  // feedback
  if (!tourobo_2026_interfaces__action__BallVomitPlate_Feedback__init(&msg->feedback)) {
    tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__fini(msg);
    return false;
  }
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__fini(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * msg)
{
  if (!msg) {
    return;
  }
  // goal_id
  unique_identifier_msgs__msg__UUID__fini(&msg->goal_id);
  // feedback
  tourobo_2026_interfaces__action__BallVomitPlate_Feedback__fini(&msg->feedback);
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  // goal_id
  if (!unique_identifier_msgs__msg__UUID__are_equal(
      &(lhs->goal_id), &(rhs->goal_id)))
  {
    return false;
  }
  // feedback
  if (!tourobo_2026_interfaces__action__BallVomitPlate_Feedback__are_equal(
      &(lhs->feedback), &(rhs->feedback)))
  {
    return false;
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * input,
  tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * output)
{
  if (!input || !output) {
    return false;
  }
  // goal_id
  if (!unique_identifier_msgs__msg__UUID__copy(
      &(input->goal_id), &(output->goal_id)))
  {
    return false;
  }
  // feedback
  if (!tourobo_2026_interfaces__action__BallVomitPlate_Feedback__copy(
      &(input->feedback), &(output->feedback)))
  {
    return false;
  }
  return true;
}

tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage *
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__create()
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * msg = (tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage), allocator.state);
  if (!msg) {
    return NULL;
  }
  memset(msg, 0, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage));
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__init(msg);
  if (!success) {
    allocator.deallocate(msg, allocator.state);
    return NULL;
  }
  return msg;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__destroy(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * msg)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (msg) {
    tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__fini(msg);
  }
  allocator.deallocate(msg, allocator.state);
}


bool
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__init(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence * array, size_t size)
{
  if (!array) {
    return false;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * data = NULL;

  if (size) {
    data = (tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage *)allocator.zero_allocate(size, sizeof(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage), allocator.state);
    if (!data) {
      return false;
    }
    // initialize all array elements
    size_t i;
    for (i = 0; i < size; ++i) {
      bool success = tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__init(&data[i]);
      if (!success) {
        break;
      }
    }
    if (i < size) {
      // if initialization failed finalize the already initialized array elements
      for (; i > 0; --i) {
        tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__fini(&data[i - 1]);
      }
      allocator.deallocate(data, allocator.state);
      return false;
    }
  }
  array->data = data;
  array->size = size;
  array->capacity = size;
  return true;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__fini(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence * array)
{
  if (!array) {
    return;
  }
  rcutils_allocator_t allocator = rcutils_get_default_allocator();

  if (array->data) {
    // ensure that data and capacity values are consistent
    assert(array->capacity > 0);
    // finalize all array elements
    for (size_t i = 0; i < array->capacity; ++i) {
      tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__fini(&array->data[i]);
    }
    allocator.deallocate(array->data, allocator.state);
    array->data = NULL;
    array->size = 0;
    array->capacity = 0;
  } else {
    // ensure that data, size, and capacity values are consistent
    assert(0 == array->size);
    assert(0 == array->capacity);
  }
}

tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence *
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__create(size_t size)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence * array = (tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence *)allocator.allocate(sizeof(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence), allocator.state);
  if (!array) {
    return NULL;
  }
  bool success = tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__init(array, size);
  if (!success) {
    allocator.deallocate(array, allocator.state);
    return NULL;
  }
  return array;
}

void
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__destroy(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence * array)
{
  rcutils_allocator_t allocator = rcutils_get_default_allocator();
  if (array) {
    tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__fini(array);
  }
  allocator.deallocate(array, allocator.state);
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__are_equal(const tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence * lhs, const tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence * rhs)
{
  if (!lhs || !rhs) {
    return false;
  }
  if (lhs->size != rhs->size) {
    return false;
  }
  for (size_t i = 0; i < lhs->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__are_equal(&(lhs->data[i]), &(rhs->data[i]))) {
      return false;
    }
  }
  return true;
}

bool
tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__copy(
  const tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence * input,
  tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence * output)
{
  if (!input || !output) {
    return false;
  }
  if (output->capacity < input->size) {
    const size_t allocation_size =
      input->size * sizeof(tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage);
    rcutils_allocator_t allocator = rcutils_get_default_allocator();
    tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage * data =
      (tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage *)allocator.reallocate(
      output->data, allocation_size, allocator.state);
    if (!data) {
      return false;
    }
    // If reallocation succeeded, memory may or may not have been moved
    // to fulfill the allocation request, invalidating output->data.
    output->data = data;
    for (size_t i = output->capacity; i < input->size; ++i) {
      if (!tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__init(&output->data[i])) {
        // If initialization of any new item fails, roll back
        // all previously initialized items. Existing items
        // in output are to be left unmodified.
        for (; i-- > output->capacity; ) {
          tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__fini(&output->data[i]);
        }
        return false;
      }
    }
    output->capacity = input->size;
  }
  output->size = input->size;
  for (size_t i = 0; i < input->size; ++i) {
    if (!tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__copy(
        &(input->data[i]), &(output->data[i])))
    {
      return false;
    }
  }
  return true;
}
