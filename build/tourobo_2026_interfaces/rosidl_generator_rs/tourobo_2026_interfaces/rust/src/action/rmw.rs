
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_Goal() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallGet_Goal__init(msg: *mut BallGet_Goal) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallGet_Goal>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallGet_Goal>);
    fn tourobo_2026_interfaces__action__BallGet_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallGet_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<BallGet_Goal>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallGet_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallGet_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub execute_mode: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_right_arm_open: bool,

}



impl Default for BallGet_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallGet_Goal__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallGet_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallGet_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallGet_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallGet_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallGet_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_Goal() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_Result() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallGet_Result__init(msg: *mut BallGet_Result) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallGet_Result>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallGet_Result>);
    fn tourobo_2026_interfaces__action__BallGet_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallGet_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<BallGet_Result>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallGet_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallGet_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_right_arm_open: bool,

}



impl Default for BallGet_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallGet_Result__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallGet_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallGet_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallGet_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallGet_Result where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallGet_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_Result() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallGet_Feedback__init(msg: *mut BallGet_Feedback) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallGet_Feedback>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallGet_Feedback>);
    fn tourobo_2026_interfaces__action__BallGet_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallGet_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<BallGet_Feedback>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallGet_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallGet_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallGet_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallGet_Feedback__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallGet_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallGet_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallGet_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallGet_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallGet_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_Feedback() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallGet_FeedbackMessage__init(msg: *mut BallGet_FeedbackMessage) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallGet_FeedbackMessage>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallGet_FeedbackMessage>);
    fn tourobo_2026_interfaces__action__BallGet_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallGet_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<BallGet_FeedbackMessage>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallGet_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallGet_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::BallGet_Feedback,

}



impl Default for BallGet_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallGet_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallGet_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallGet_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallGet_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallGet_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallGet_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_FeedbackMessage() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_Goal() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutGate_Goal__init(msg: *mut BallPutGate_Goal) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_Goal>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_Goal>);
    fn tourobo_2026_interfaces__action__BallPutGate_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutGate_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_Goal>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutGate_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutGate_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub execute: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_right_arm_open: bool,

}



impl Default for BallPutGate_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutGate_Goal__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutGate_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutGate_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutGate_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutGate_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_Goal() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_Result() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutGate_Result__init(msg: *mut BallPutGate_Result) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_Result>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_Result>);
    fn tourobo_2026_interfaces__action__BallPutGate_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutGate_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_Result>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutGate_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutGate_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_right_arm_open: bool,

}



impl Default for BallPutGate_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutGate_Result__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutGate_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutGate_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutGate_Result where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutGate_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_Result() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutGate_Feedback__init(msg: *mut BallPutGate_Feedback) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_Feedback>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_Feedback>);
    fn tourobo_2026_interfaces__action__BallPutGate_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutGate_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_Feedback>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutGate_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutGate_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallPutGate_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutGate_Feedback__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutGate_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutGate_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutGate_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutGate_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_Feedback() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutGate_FeedbackMessage__init(msg: *mut BallPutGate_FeedbackMessage) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_FeedbackMessage>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_FeedbackMessage>);
    fn tourobo_2026_interfaces__action__BallPutGate_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutGate_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_FeedbackMessage>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutGate_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutGate_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::BallPutGate_Feedback,

}



impl Default for BallPutGate_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutGate_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutGate_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutGate_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutGate_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutGate_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_FeedbackMessage() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_Goal() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutPlate_Goal__init(msg: *mut BallPutPlate_Goal) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_Goal>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_Goal>);
    fn tourobo_2026_interfaces__action__BallPutPlate_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutPlate_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_Goal>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutPlate_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub execute: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_right_arm_open: bool,

}



impl Default for BallPutPlate_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutPlate_Goal__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutPlate_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutPlate_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutPlate_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutPlate_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_Goal() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_Result() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutPlate_Result__init(msg: *mut BallPutPlate_Result) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_Result>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_Result>);
    fn tourobo_2026_interfaces__action__BallPutPlate_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutPlate_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_Result>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutPlate_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_right_arm_open: bool,

}



impl Default for BallPutPlate_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutPlate_Result__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutPlate_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutPlate_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutPlate_Result where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutPlate_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_Result() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutPlate_Feedback__init(msg: *mut BallPutPlate_Feedback) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_Feedback>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_Feedback>);
    fn tourobo_2026_interfaces__action__BallPutPlate_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutPlate_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_Feedback>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutPlate_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallPutPlate_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutPlate_Feedback__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutPlate_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutPlate_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutPlate_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutPlate_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_Feedback() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage__init(msg: *mut BallPutPlate_FeedbackMessage) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_FeedbackMessage>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_FeedbackMessage>);
    fn tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutPlate_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_FeedbackMessage>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutPlate_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::BallPutPlate_Feedback,

}



impl Default for BallPutPlate_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutPlate_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutPlate_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutPlate_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_Goal() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShoot_Goal__init(msg: *mut BallShoot_Goal) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_Goal>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_Goal>);
    fn tourobo_2026_interfaces__action__BallShoot_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShoot_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShoot_Goal>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShoot_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShoot_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub execute: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_right_arm_open: bool,

}



impl Default for BallShoot_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShoot_Goal__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShoot_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShoot_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShoot_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShoot_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShoot_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_Goal() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_Result() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShoot_Result__init(msg: *mut BallShoot_Result) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_Result>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_Result>);
    fn tourobo_2026_interfaces__action__BallShoot_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShoot_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShoot_Result>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShoot_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShoot_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_right_arm_open: bool,

}



impl Default for BallShoot_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShoot_Result__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShoot_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShoot_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShoot_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShoot_Result where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShoot_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_Result() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShoot_Feedback__init(msg: *mut BallShoot_Feedback) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_Feedback>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_Feedback>);
    fn tourobo_2026_interfaces__action__BallShoot_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShoot_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShoot_Feedback>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShoot_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShoot_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallShoot_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShoot_Feedback__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShoot_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShoot_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShoot_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShoot_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShoot_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_Feedback() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShoot_FeedbackMessage__init(msg: *mut BallShoot_FeedbackMessage) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_FeedbackMessage>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_FeedbackMessage>);
    fn tourobo_2026_interfaces__action__BallShoot_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShoot_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShoot_FeedbackMessage>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShoot_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShoot_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::BallShoot_Feedback,

}



impl Default for BallShoot_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShoot_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShoot_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShoot_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShoot_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShoot_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShoot_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_FeedbackMessage() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_Goal() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShootAim_Goal__init(msg: *mut BallShootAim_Goal) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_Goal>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_Goal>);
    fn tourobo_2026_interfaces__action__BallShootAim_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShootAim_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_Goal>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShootAim_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShootAim_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub direction: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_right_arm_open: bool,

}



impl Default for BallShootAim_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShootAim_Goal__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShootAim_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShootAim_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShootAim_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShootAim_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_Goal() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_Result() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShootAim_Result__init(msg: *mut BallShootAim_Result) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_Result>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_Result>);
    fn tourobo_2026_interfaces__action__BallShootAim_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShootAim_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_Result>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShootAim_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShootAim_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_right_arm_open: bool,

}



impl Default for BallShootAim_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShootAim_Result__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShootAim_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShootAim_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShootAim_Result where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShootAim_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_Result() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShootAim_Feedback__init(msg: *mut BallShootAim_Feedback) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_Feedback>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_Feedback>);
    fn tourobo_2026_interfaces__action__BallShootAim_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShootAim_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_Feedback>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShootAim_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShootAim_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallShootAim_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShootAim_Feedback__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShootAim_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShootAim_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShootAim_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShootAim_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_Feedback() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShootAim_FeedbackMessage__init(msg: *mut BallShootAim_FeedbackMessage) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_FeedbackMessage>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_FeedbackMessage>);
    fn tourobo_2026_interfaces__action__BallShootAim_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShootAim_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_FeedbackMessage>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShootAim_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShootAim_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::BallShootAim_Feedback,

}



impl Default for BallShootAim_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShootAim_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShootAim_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShootAim_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShootAim_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShootAim_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_FeedbackMessage() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_Goal() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallIntake_Goal__init(msg: *mut BallIntake_Goal) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_Goal>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_Goal>);
    fn tourobo_2026_interfaces__action__BallIntake_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallIntake_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<BallIntake_Goal>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallIntake_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallIntake_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub execute_mode: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_right_arm_open: bool,

}



impl Default for BallIntake_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallIntake_Goal__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallIntake_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallIntake_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallIntake_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallIntake_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallIntake_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_Goal() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_Result() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallIntake_Result__init(msg: *mut BallIntake_Result) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_Result>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_Result>);
    fn tourobo_2026_interfaces__action__BallIntake_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallIntake_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<BallIntake_Result>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallIntake_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallIntake_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_right_arm_open: bool,

}



impl Default for BallIntake_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallIntake_Result__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallIntake_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallIntake_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallIntake_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallIntake_Result where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallIntake_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_Result() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallIntake_Feedback__init(msg: *mut BallIntake_Feedback) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_Feedback>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_Feedback>);
    fn tourobo_2026_interfaces__action__BallIntake_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallIntake_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<BallIntake_Feedback>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallIntake_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallIntake_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallIntake_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallIntake_Feedback__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallIntake_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallIntake_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallIntake_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallIntake_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallIntake_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_Feedback() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallIntake_FeedbackMessage__init(msg: *mut BallIntake_FeedbackMessage) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_FeedbackMessage>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_FeedbackMessage>);
    fn tourobo_2026_interfaces__action__BallIntake_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallIntake_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<BallIntake_FeedbackMessage>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallIntake_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallIntake_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::BallIntake_Feedback,

}



impl Default for BallIntake_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallIntake_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallIntake_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallIntake_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallIntake_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallIntake_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallIntake_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_FeedbackMessage() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_Goal() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallArmOperation_Goal__init(msg: *mut BallArmOperation_Goal) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_Goal>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_Goal>);
    fn tourobo_2026_interfaces__action__BallArmOperation_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallArmOperation_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_Goal>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallArmOperation_Goal {
    /// 1:左を操作 2:右を操作
    pub target_arm: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_right_arm_open: bool,

}



impl Default for BallArmOperation_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallArmOperation_Goal__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallArmOperation_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallArmOperation_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallArmOperation_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallArmOperation_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_Goal() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_Result() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallArmOperation_Result__init(msg: *mut BallArmOperation_Result) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_Result>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_Result>);
    fn tourobo_2026_interfaces__action__BallArmOperation_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallArmOperation_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_Result>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallArmOperation_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_right_arm_open: bool,

}



impl Default for BallArmOperation_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallArmOperation_Result__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallArmOperation_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallArmOperation_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallArmOperation_Result where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallArmOperation_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_Result() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallArmOperation_Feedback__init(msg: *mut BallArmOperation_Feedback) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_Feedback>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_Feedback>);
    fn tourobo_2026_interfaces__action__BallArmOperation_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallArmOperation_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_Feedback>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallArmOperation_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallArmOperation_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallArmOperation_Feedback__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallArmOperation_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallArmOperation_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallArmOperation_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallArmOperation_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_Feedback() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage__init(msg: *mut BallArmOperation_FeedbackMessage) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_FeedbackMessage>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_FeedbackMessage>);
    fn tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallArmOperation_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_FeedbackMessage>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallArmOperation_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::BallArmOperation_Feedback,

}



impl Default for BallArmOperation_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallArmOperation_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallArmOperation_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallArmOperation_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_Goal() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__MechanismReset_Goal__init(msg: *mut MechanismReset_Goal) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_Goal>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_Goal>);
    fn tourobo_2026_interfaces__action__MechanismReset_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MechanismReset_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_Goal>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__MechanismReset_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MechanismReset_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub execute: bool,

}



impl Default for MechanismReset_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__MechanismReset_Goal__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__MechanismReset_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MechanismReset_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MechanismReset_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/MechanismReset_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_Goal() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_Result() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__MechanismReset_Result__init(msg: *mut MechanismReset_Result) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_Result>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_Result>);
    fn tourobo_2026_interfaces__action__MechanismReset_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MechanismReset_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_Result>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__MechanismReset_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MechanismReset_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_right_arm_open: bool,

}



impl Default for MechanismReset_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__MechanismReset_Result__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__MechanismReset_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MechanismReset_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MechanismReset_Result where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/MechanismReset_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_Result() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__MechanismReset_Feedback__init(msg: *mut MechanismReset_Feedback) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_Feedback>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_Feedback>);
    fn tourobo_2026_interfaces__action__MechanismReset_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MechanismReset_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_Feedback>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__MechanismReset_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MechanismReset_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for MechanismReset_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__MechanismReset_Feedback__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__MechanismReset_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MechanismReset_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MechanismReset_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/MechanismReset_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_Feedback() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage__init(msg: *mut MechanismReset_FeedbackMessage) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_FeedbackMessage>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_FeedbackMessage>);
    fn tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MechanismReset_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_FeedbackMessage>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MechanismReset_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::MechanismReset_Feedback,

}



impl Default for MechanismReset_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MechanismReset_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MechanismReset_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/MechanismReset_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_Goal() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallVomitPlate_Goal__init(msg: *mut BallVomitPlate_Goal) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_Goal>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_Goal>);
    fn tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallVomitPlate_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_Goal>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallVomitPlate_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub execute_mode: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub is_right_arm_open: bool,

}



impl Default for BallVomitPlate_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallVomitPlate_Goal__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallVomitPlate_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallVomitPlate_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallVomitPlate_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallVomitPlate_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_Goal() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_Result() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallVomitPlate_Result__init(msg: *mut BallVomitPlate_Result) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_Result>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_Result>);
    fn tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallVomitPlate_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_Result>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallVomitPlate_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_carry: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_push_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_shoot_angle_state: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_left_arm_open: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub next_is_right_arm_open: bool,

}



impl Default for BallVomitPlate_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallVomitPlate_Result__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallVomitPlate_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallVomitPlate_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallVomitPlate_Result where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallVomitPlate_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_Result() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallVomitPlate_Feedback__init(msg: *mut BallVomitPlate_Feedback) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_Feedback>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_Feedback>);
    fn tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallVomitPlate_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_Feedback>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallVomitPlate_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallVomitPlate_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallVomitPlate_Feedback__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallVomitPlate_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallVomitPlate_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallVomitPlate_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallVomitPlate_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_Feedback() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__init(msg: *mut BallVomitPlate_FeedbackMessage) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_FeedbackMessage>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_FeedbackMessage>);
    fn tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallVomitPlate_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_FeedbackMessage>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallVomitPlate_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::BallVomitPlate_Feedback,

}



impl Default for BallVomitPlate_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallVomitPlate_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallVomitPlate_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallVomitPlate_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage() }
  }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallGet_SendGoal_Request__init(msg: *mut BallGet_SendGoal_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallGet_SendGoal_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallGet_SendGoal_Request>);
    fn tourobo_2026_interfaces__action__BallGet_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallGet_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallGet_SendGoal_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallGet_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallGet_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::BallGet_Goal,

}



impl Default for BallGet_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallGet_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallGet_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallGet_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallGet_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallGet_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallGet_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_SendGoal_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallGet_SendGoal_Response__init(msg: *mut BallGet_SendGoal_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallGet_SendGoal_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallGet_SendGoal_Response>);
    fn tourobo_2026_interfaces__action__BallGet_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallGet_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallGet_SendGoal_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallGet_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallGet_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for BallGet_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallGet_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallGet_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallGet_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallGet_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallGet_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallGet_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_SendGoal_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallGet_GetResult_Request__init(msg: *mut BallGet_GetResult_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallGet_GetResult_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallGet_GetResult_Request>);
    fn tourobo_2026_interfaces__action__BallGet_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallGet_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallGet_GetResult_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallGet_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallGet_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for BallGet_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallGet_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallGet_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallGet_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallGet_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallGet_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallGet_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_GetResult_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallGet_GetResult_Response__init(msg: *mut BallGet_GetResult_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallGet_GetResult_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallGet_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallGet_GetResult_Response>);
    fn tourobo_2026_interfaces__action__BallGet_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallGet_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallGet_GetResult_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallGet_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallGet_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::BallGet_Result,

}



impl Default for BallGet_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallGet_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallGet_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallGet_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallGet_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallGet_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallGet_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallGet_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallGet_GetResult_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutGate_SendGoal_Request__init(msg: *mut BallPutGate_SendGoal_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_SendGoal_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_SendGoal_Request>);
    fn tourobo_2026_interfaces__action__BallPutGate_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutGate_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_SendGoal_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutGate_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutGate_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::BallPutGate_Goal,

}



impl Default for BallPutGate_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutGate_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutGate_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutGate_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutGate_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutGate_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_SendGoal_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutGate_SendGoal_Response__init(msg: *mut BallPutGate_SendGoal_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_SendGoal_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_SendGoal_Response>);
    fn tourobo_2026_interfaces__action__BallPutGate_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutGate_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_SendGoal_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutGate_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutGate_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for BallPutGate_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutGate_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutGate_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutGate_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutGate_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutGate_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_SendGoal_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutGate_GetResult_Request__init(msg: *mut BallPutGate_GetResult_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_GetResult_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_GetResult_Request>);
    fn tourobo_2026_interfaces__action__BallPutGate_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutGate_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_GetResult_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutGate_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutGate_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for BallPutGate_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutGate_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutGate_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutGate_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutGate_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutGate_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_GetResult_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutGate_GetResult_Response__init(msg: *mut BallPutGate_GetResult_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_GetResult_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutGate_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_GetResult_Response>);
    fn tourobo_2026_interfaces__action__BallPutGate_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutGate_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutGate_GetResult_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutGate_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutGate_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::BallPutGate_Result,

}



impl Default for BallPutGate_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutGate_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutGate_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutGate_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutGate_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutGate_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutGate_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_GetResult_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request__init(msg: *mut BallPutPlate_SendGoal_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_SendGoal_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_SendGoal_Request>);
    fn tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutPlate_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_SendGoal_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutPlate_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::BallPutPlate_Goal,

}



impl Default for BallPutPlate_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutPlate_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutPlate_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutPlate_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response__init(msg: *mut BallPutPlate_SendGoal_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_SendGoal_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_SendGoal_Response>);
    fn tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutPlate_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_SendGoal_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutPlate_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for BallPutPlate_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutPlate_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutPlate_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutPlate_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request__init(msg: *mut BallPutPlate_GetResult_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_GetResult_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_GetResult_Request>);
    fn tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutPlate_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_GetResult_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutPlate_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for BallPutPlate_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutPlate_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutPlate_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutPlate_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response__init(msg: *mut BallPutPlate_GetResult_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_GetResult_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_GetResult_Response>);
    fn tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallPutPlate_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallPutPlate_GetResult_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutPlate_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::BallPutPlate_Result,

}



impl Default for BallPutPlate_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallPutPlate_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallPutPlate_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallPutPlate_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShoot_SendGoal_Request__init(msg: *mut BallShoot_SendGoal_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_SendGoal_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_SendGoal_Request>);
    fn tourobo_2026_interfaces__action__BallShoot_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShoot_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShoot_SendGoal_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShoot_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShoot_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::BallShoot_Goal,

}



impl Default for BallShoot_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShoot_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShoot_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShoot_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShoot_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShoot_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShoot_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_SendGoal_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShoot_SendGoal_Response__init(msg: *mut BallShoot_SendGoal_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_SendGoal_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_SendGoal_Response>);
    fn tourobo_2026_interfaces__action__BallShoot_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShoot_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShoot_SendGoal_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShoot_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShoot_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for BallShoot_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShoot_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShoot_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShoot_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShoot_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShoot_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShoot_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_SendGoal_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShoot_GetResult_Request__init(msg: *mut BallShoot_GetResult_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_GetResult_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_GetResult_Request>);
    fn tourobo_2026_interfaces__action__BallShoot_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShoot_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShoot_GetResult_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShoot_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShoot_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for BallShoot_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShoot_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShoot_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShoot_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShoot_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShoot_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShoot_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_GetResult_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShoot_GetResult_Response__init(msg: *mut BallShoot_GetResult_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_GetResult_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShoot_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShoot_GetResult_Response>);
    fn tourobo_2026_interfaces__action__BallShoot_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShoot_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShoot_GetResult_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShoot_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShoot_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::BallShoot_Result,

}



impl Default for BallShoot_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShoot_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShoot_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShoot_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShoot_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShoot_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShoot_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShoot_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShoot_GetResult_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShootAim_SendGoal_Request__init(msg: *mut BallShootAim_SendGoal_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_SendGoal_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_SendGoal_Request>);
    fn tourobo_2026_interfaces__action__BallShootAim_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShootAim_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_SendGoal_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShootAim_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShootAim_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::BallShootAim_Goal,

}



impl Default for BallShootAim_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShootAim_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShootAim_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShootAim_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShootAim_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShootAim_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_SendGoal_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShootAim_SendGoal_Response__init(msg: *mut BallShootAim_SendGoal_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_SendGoal_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_SendGoal_Response>);
    fn tourobo_2026_interfaces__action__BallShootAim_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShootAim_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_SendGoal_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShootAim_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShootAim_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for BallShootAim_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShootAim_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShootAim_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShootAim_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShootAim_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShootAim_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_SendGoal_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShootAim_GetResult_Request__init(msg: *mut BallShootAim_GetResult_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_GetResult_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_GetResult_Request>);
    fn tourobo_2026_interfaces__action__BallShootAim_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShootAim_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_GetResult_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShootAim_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShootAim_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for BallShootAim_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShootAim_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShootAim_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShootAim_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShootAim_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShootAim_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_GetResult_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallShootAim_GetResult_Response__init(msg: *mut BallShootAim_GetResult_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_GetResult_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallShootAim_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_GetResult_Response>);
    fn tourobo_2026_interfaces__action__BallShootAim_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallShootAim_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallShootAim_GetResult_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallShootAim_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShootAim_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::BallShootAim_Result,

}



impl Default for BallShootAim_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallShootAim_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallShootAim_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallShootAim_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallShootAim_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallShootAim_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallShootAim_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_GetResult_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallIntake_SendGoal_Request__init(msg: *mut BallIntake_SendGoal_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_SendGoal_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_SendGoal_Request>);
    fn tourobo_2026_interfaces__action__BallIntake_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallIntake_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallIntake_SendGoal_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallIntake_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallIntake_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::BallIntake_Goal,

}



impl Default for BallIntake_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallIntake_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallIntake_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallIntake_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallIntake_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallIntake_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallIntake_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_SendGoal_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallIntake_SendGoal_Response__init(msg: *mut BallIntake_SendGoal_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_SendGoal_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_SendGoal_Response>);
    fn tourobo_2026_interfaces__action__BallIntake_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallIntake_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallIntake_SendGoal_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallIntake_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallIntake_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for BallIntake_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallIntake_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallIntake_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallIntake_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallIntake_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallIntake_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallIntake_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_SendGoal_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallIntake_GetResult_Request__init(msg: *mut BallIntake_GetResult_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_GetResult_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_GetResult_Request>);
    fn tourobo_2026_interfaces__action__BallIntake_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallIntake_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallIntake_GetResult_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallIntake_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallIntake_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for BallIntake_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallIntake_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallIntake_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallIntake_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallIntake_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallIntake_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallIntake_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_GetResult_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallIntake_GetResult_Response__init(msg: *mut BallIntake_GetResult_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_GetResult_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallIntake_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallIntake_GetResult_Response>);
    fn tourobo_2026_interfaces__action__BallIntake_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallIntake_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallIntake_GetResult_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallIntake_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallIntake_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::BallIntake_Result,

}



impl Default for BallIntake_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallIntake_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallIntake_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallIntake_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallIntake_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallIntake_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallIntake_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallIntake_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallIntake_GetResult_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request__init(msg: *mut BallArmOperation_SendGoal_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_SendGoal_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_SendGoal_Request>);
    fn tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallArmOperation_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_SendGoal_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallArmOperation_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::BallArmOperation_Goal,

}



impl Default for BallArmOperation_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallArmOperation_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallArmOperation_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallArmOperation_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response__init(msg: *mut BallArmOperation_SendGoal_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_SendGoal_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_SendGoal_Response>);
    fn tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallArmOperation_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_SendGoal_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallArmOperation_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for BallArmOperation_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallArmOperation_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallArmOperation_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallArmOperation_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request__init(msg: *mut BallArmOperation_GetResult_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_GetResult_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_GetResult_Request>);
    fn tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallArmOperation_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_GetResult_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallArmOperation_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for BallArmOperation_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallArmOperation_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallArmOperation_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallArmOperation_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response__init(msg: *mut BallArmOperation_GetResult_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_GetResult_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_GetResult_Response>);
    fn tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallArmOperation_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallArmOperation_GetResult_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallArmOperation_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::BallArmOperation_Result,

}



impl Default for BallArmOperation_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallArmOperation_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallArmOperation_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallArmOperation_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request__init(msg: *mut MechanismReset_SendGoal_Request) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_SendGoal_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_SendGoal_Request>);
    fn tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MechanismReset_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_SendGoal_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MechanismReset_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::MechanismReset_Goal,

}



impl Default for MechanismReset_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MechanismReset_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MechanismReset_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/MechanismReset_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response__init(msg: *mut MechanismReset_SendGoal_Response) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_SendGoal_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_SendGoal_Response>);
    fn tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MechanismReset_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_SendGoal_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MechanismReset_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for MechanismReset_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MechanismReset_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MechanismReset_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/MechanismReset_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__MechanismReset_GetResult_Request__init(msg: *mut MechanismReset_GetResult_Request) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_GetResult_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_GetResult_Request>);
    fn tourobo_2026_interfaces__action__MechanismReset_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MechanismReset_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_GetResult_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__MechanismReset_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MechanismReset_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for MechanismReset_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__MechanismReset_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__MechanismReset_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MechanismReset_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MechanismReset_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/MechanismReset_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_GetResult_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__MechanismReset_GetResult_Response__init(msg: *mut MechanismReset_GetResult_Response) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_GetResult_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__MechanismReset_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_GetResult_Response>);
    fn tourobo_2026_interfaces__action__MechanismReset_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MechanismReset_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MechanismReset_GetResult_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__MechanismReset_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MechanismReset_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::MechanismReset_Result,

}



impl Default for MechanismReset_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__MechanismReset_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__MechanismReset_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MechanismReset_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__MechanismReset_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MechanismReset_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/MechanismReset_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_GetResult_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__init(msg: *mut BallVomitPlate_SendGoal_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_SendGoal_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_SendGoal_Request>);
    fn tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallVomitPlate_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_SendGoal_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallVomitPlate_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::BallVomitPlate_Goal,

}



impl Default for BallVomitPlate_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallVomitPlate_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallVomitPlate_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallVomitPlate_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__init(msg: *mut BallVomitPlate_SendGoal_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_SendGoal_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_SendGoal_Response>);
    fn tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallVomitPlate_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_SendGoal_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallVomitPlate_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for BallVomitPlate_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallVomitPlate_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallVomitPlate_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallVomitPlate_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__init(msg: *mut BallVomitPlate_GetResult_Request) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_GetResult_Request>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_GetResult_Request>);
    fn tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallVomitPlate_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_GetResult_Request>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallVomitPlate_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for BallVomitPlate_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallVomitPlate_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallVomitPlate_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallVomitPlate_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request() }
  }
}


#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "tourobo_2026_interfaces__rosidl_generator_c")]
extern "C" {
    fn tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__init(msg: *mut BallVomitPlate_GetResult_Response) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_GetResult_Response>, size: usize) -> bool;
    fn tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_GetResult_Response>);
    fn tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<BallVomitPlate_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<BallVomitPlate_GetResult_Response>) -> bool;
}

// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallVomitPlate_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::BallVomitPlate_Result,

}



impl Default for BallVomitPlate_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for BallVomitPlate_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for BallVomitPlate_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "tourobo_2026_interfaces/action/BallVomitPlate_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response() }
  }
}






#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallGet_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallGet_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct BallGet_SendGoal;

impl rosidl_runtime_rs::Service for BallGet_SendGoal {
    type Request = BallGet_SendGoal_Request;
    type Response = BallGet_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallGet_SendGoal() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallGet_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallGet_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct BallGet_GetResult;

impl rosidl_runtime_rs::Service for BallGet_GetResult {
    type Request = BallGet_GetResult_Request;
    type Response = BallGet_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallGet_GetResult() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutGate_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct BallPutGate_SendGoal;

impl rosidl_runtime_rs::Service for BallPutGate_SendGoal {
    type Request = BallPutGate_SendGoal_Request;
    type Response = BallPutGate_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_SendGoal() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutGate_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct BallPutGate_GetResult;

impl rosidl_runtime_rs::Service for BallPutGate_GetResult {
    type Request = BallPutGate_GetResult_Request;
    type Response = BallPutGate_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallPutGate_GetResult() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct BallPutPlate_SendGoal;

impl rosidl_runtime_rs::Service for BallPutPlate_SendGoal {
    type Request = BallPutPlate_SendGoal_Request;
    type Response = BallPutPlate_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_SendGoal() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct BallPutPlate_GetResult;

impl rosidl_runtime_rs::Service for BallPutPlate_GetResult {
    type Request = BallPutPlate_GetResult_Request;
    type Response = BallPutPlate_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate_GetResult() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallShoot_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallShoot_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct BallShoot_SendGoal;

impl rosidl_runtime_rs::Service for BallShoot_SendGoal {
    type Request = BallShoot_SendGoal_Request;
    type Response = BallShoot_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallShoot_SendGoal() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallShoot_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallShoot_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct BallShoot_GetResult;

impl rosidl_runtime_rs::Service for BallShoot_GetResult {
    type Request = BallShoot_GetResult_Request;
    type Response = BallShoot_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallShoot_GetResult() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallShootAim_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct BallShootAim_SendGoal;

impl rosidl_runtime_rs::Service for BallShootAim_SendGoal {
    type Request = BallShootAim_SendGoal_Request;
    type Response = BallShootAim_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_SendGoal() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallShootAim_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct BallShootAim_GetResult;

impl rosidl_runtime_rs::Service for BallShootAim_GetResult {
    type Request = BallShootAim_GetResult_Request;
    type Response = BallShootAim_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallShootAim_GetResult() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallIntake_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallIntake_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct BallIntake_SendGoal;

impl rosidl_runtime_rs::Service for BallIntake_SendGoal {
    type Request = BallIntake_SendGoal_Request;
    type Response = BallIntake_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallIntake_SendGoal() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallIntake_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallIntake_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct BallIntake_GetResult;

impl rosidl_runtime_rs::Service for BallIntake_GetResult {
    type Request = BallIntake_GetResult_Request;
    type Response = BallIntake_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallIntake_GetResult() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct BallArmOperation_SendGoal;

impl rosidl_runtime_rs::Service for BallArmOperation_SendGoal {
    type Request = BallArmOperation_SendGoal_Request;
    type Response = BallArmOperation_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_SendGoal() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct BallArmOperation_GetResult;

impl rosidl_runtime_rs::Service for BallArmOperation_GetResult {
    type Request = BallArmOperation_GetResult_Request;
    type Response = BallArmOperation_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation_GetResult() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__MechanismReset_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct MechanismReset_SendGoal;

impl rosidl_runtime_rs::Service for MechanismReset_SendGoal {
    type Request = MechanismReset_SendGoal_Request;
    type Response = MechanismReset_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_SendGoal() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__MechanismReset_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct MechanismReset_GetResult;

impl rosidl_runtime_rs::Service for MechanismReset_GetResult {
    type Request = MechanismReset_GetResult_Request;
    type Response = MechanismReset_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__MechanismReset_GetResult() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct BallVomitPlate_SendGoal;

impl rosidl_runtime_rs::Service for BallVomitPlate_SendGoal {
    type Request = BallVomitPlate_SendGoal_Request;
    type Response = BallVomitPlate_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_SendGoal() }
    }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct BallVomitPlate_GetResult;

impl rosidl_runtime_rs::Service for BallVomitPlate_GetResult {
    type Request = BallVomitPlate_GetResult_Request;
    type Response = BallVomitPlate_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate_GetResult() }
    }
}


