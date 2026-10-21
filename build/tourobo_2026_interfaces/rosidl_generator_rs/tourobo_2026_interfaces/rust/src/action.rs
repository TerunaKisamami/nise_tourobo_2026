
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to tourobo_2026_interfaces__action__BallGet_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallGet_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for BallGet_Goal {
  type RmwMsg = super::action::rmw::BallGet_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        execute_mode: msg.execute_mode,
        current_state: msg.current_state,
        carry: msg.carry,
        push_state: msg.push_state,
        shoot_angle_state: msg.shoot_angle_state,
        is_left_arm_open: msg.is_left_arm_open,
        is_right_arm_open: msg.is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      execute_mode: msg.execute_mode,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      execute_mode: msg.execute_mode,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallGet_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallGet_Result::default())
  }
}

impl rosidl_runtime_rs::Message for BallGet_Result {
  type RmwMsg = super::action::rmw::BallGet_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        next_state: msg.next_state,
        next_carry: msg.next_carry,
        next_push_state: msg.next_push_state,
        next_shoot_angle_state: msg.next_shoot_angle_state,
        next_is_left_arm_open: msg.next_is_left_arm_open,
        next_is_right_arm_open: msg.next_is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallGet_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallGet_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallGet_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallGet_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for BallGet_Feedback {
  type RmwMsg = super::action::rmw::BallGet_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallGet_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallGet_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::BallGet_Feedback,

}



impl Default for BallGet_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallGet_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for BallGet_FeedbackMessage {
  type RmwMsg = super::action::rmw::BallGet_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::BallGet_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::BallGet_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::BallGet_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutGate_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutGate_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_Goal {
  type RmwMsg = super::action::rmw::BallPutGate_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        execute: msg.execute,
        current_state: msg.current_state,
        carry: msg.carry,
        push_state: msg.push_state,
        shoot_angle_state: msg.shoot_angle_state,
        is_left_arm_open: msg.is_left_arm_open,
        is_right_arm_open: msg.is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      execute: msg.execute,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      execute: msg.execute,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutGate_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutGate_Result::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_Result {
  type RmwMsg = super::action::rmw::BallPutGate_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        next_state: msg.next_state,
        next_carry: msg.next_carry,
        next_push_state: msg.next_push_state,
        next_shoot_angle_state: msg.next_shoot_angle_state,
        next_is_left_arm_open: msg.next_is_left_arm_open,
        next_is_right_arm_open: msg.next_is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutGate_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutGate_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallPutGate_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutGate_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_Feedback {
  type RmwMsg = super::action::rmw::BallPutGate_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutGate_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutGate_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::BallPutGate_Feedback,

}



impl Default for BallPutGate_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutGate_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_FeedbackMessage {
  type RmwMsg = super::action::rmw::BallPutGate_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::BallPutGate_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::BallPutGate_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::BallPutGate_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutPlate_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_Goal {
  type RmwMsg = super::action::rmw::BallPutPlate_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        execute: msg.execute,
        current_state: msg.current_state,
        carry: msg.carry,
        push_state: msg.push_state,
        shoot_angle_state: msg.shoot_angle_state,
        is_left_arm_open: msg.is_left_arm_open,
        is_right_arm_open: msg.is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      execute: msg.execute,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      execute: msg.execute,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutPlate_Result::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_Result {
  type RmwMsg = super::action::rmw::BallPutPlate_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        next_state: msg.next_state,
        next_carry: msg.next_carry,
        next_push_state: msg.next_push_state,
        next_shoot_angle_state: msg.next_shoot_angle_state,
        next_is_left_arm_open: msg.next_is_left_arm_open,
        next_is_right_arm_open: msg.next_is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutPlate_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallPutPlate_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutPlate_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_Feedback {
  type RmwMsg = super::action::rmw::BallPutPlate_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutPlate_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::BallPutPlate_Feedback,

}



impl Default for BallPutPlate_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutPlate_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_FeedbackMessage {
  type RmwMsg = super::action::rmw::BallPutPlate_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::BallPutPlate_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::BallPutPlate_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::BallPutPlate_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShoot_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShoot_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for BallShoot_Goal {
  type RmwMsg = super::action::rmw::BallShoot_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        execute: msg.execute,
        current_state: msg.current_state,
        carry: msg.carry,
        push_state: msg.push_state,
        shoot_angle_state: msg.shoot_angle_state,
        is_left_arm_open: msg.is_left_arm_open,
        is_right_arm_open: msg.is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      execute: msg.execute,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      execute: msg.execute,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShoot_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShoot_Result::default())
  }
}

impl rosidl_runtime_rs::Message for BallShoot_Result {
  type RmwMsg = super::action::rmw::BallShoot_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        next_state: msg.next_state,
        next_carry: msg.next_carry,
        next_push_state: msg.next_push_state,
        next_shoot_angle_state: msg.next_shoot_angle_state,
        next_is_left_arm_open: msg.next_is_left_arm_open,
        next_is_right_arm_open: msg.next_is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShoot_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShoot_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallShoot_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShoot_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for BallShoot_Feedback {
  type RmwMsg = super::action::rmw::BallShoot_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShoot_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShoot_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::BallShoot_Feedback,

}



impl Default for BallShoot_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShoot_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for BallShoot_FeedbackMessage {
  type RmwMsg = super::action::rmw::BallShoot_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::BallShoot_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::BallShoot_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::BallShoot_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShootAim_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShootAim_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_Goal {
  type RmwMsg = super::action::rmw::BallShootAim_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        direction: msg.direction,
        current_state: msg.current_state,
        carry: msg.carry,
        push_state: msg.push_state,
        shoot_angle_state: msg.shoot_angle_state,
        is_left_arm_open: msg.is_left_arm_open,
        is_right_arm_open: msg.is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      direction: msg.direction,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      direction: msg.direction,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShootAim_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShootAim_Result::default())
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_Result {
  type RmwMsg = super::action::rmw::BallShootAim_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        next_state: msg.next_state,
        next_carry: msg.next_carry,
        next_push_state: msg.next_push_state,
        next_shoot_angle_state: msg.next_shoot_angle_state,
        next_is_left_arm_open: msg.next_is_left_arm_open,
        next_is_right_arm_open: msg.next_is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShootAim_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShootAim_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallShootAim_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShootAim_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_Feedback {
  type RmwMsg = super::action::rmw::BallShootAim_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShootAim_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShootAim_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::BallShootAim_Feedback,

}



impl Default for BallShootAim_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShootAim_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_FeedbackMessage {
  type RmwMsg = super::action::rmw::BallShootAim_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::BallShootAim_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::BallShootAim_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::BallShootAim_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallIntake_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallIntake_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for BallIntake_Goal {
  type RmwMsg = super::action::rmw::BallIntake_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        execute_mode: msg.execute_mode,
        current_state: msg.current_state,
        carry: msg.carry,
        push_state: msg.push_state,
        shoot_angle_state: msg.shoot_angle_state,
        is_left_arm_open: msg.is_left_arm_open,
        is_right_arm_open: msg.is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      execute_mode: msg.execute_mode,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      execute_mode: msg.execute_mode,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallIntake_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallIntake_Result::default())
  }
}

impl rosidl_runtime_rs::Message for BallIntake_Result {
  type RmwMsg = super::action::rmw::BallIntake_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        next_state: msg.next_state,
        next_carry: msg.next_carry,
        next_push_state: msg.next_push_state,
        next_shoot_angle_state: msg.next_shoot_angle_state,
        next_is_left_arm_open: msg.next_is_left_arm_open,
        next_is_right_arm_open: msg.next_is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallIntake_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallIntake_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallIntake_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallIntake_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for BallIntake_Feedback {
  type RmwMsg = super::action::rmw::BallIntake_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallIntake_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallIntake_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::BallIntake_Feedback,

}



impl Default for BallIntake_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallIntake_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for BallIntake_FeedbackMessage {
  type RmwMsg = super::action::rmw::BallIntake_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::BallIntake_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::BallIntake_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::BallIntake_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallArmOperation_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_Goal {
  type RmwMsg = super::action::rmw::BallArmOperation_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        target_arm: msg.target_arm,
        is_open: msg.is_open,
        current_state: msg.current_state,
        carry: msg.carry,
        push_state: msg.push_state,
        shoot_angle_state: msg.shoot_angle_state,
        is_left_arm_open: msg.is_left_arm_open,
        is_right_arm_open: msg.is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      target_arm: msg.target_arm,
      is_open: msg.is_open,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      target_arm: msg.target_arm,
      is_open: msg.is_open,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallArmOperation_Result::default())
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_Result {
  type RmwMsg = super::action::rmw::BallArmOperation_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        next_state: msg.next_state,
        next_carry: msg.next_carry,
        next_push_state: msg.next_push_state,
        next_shoot_angle_state: msg.next_shoot_angle_state,
        next_is_left_arm_open: msg.next_is_left_arm_open,
        next_is_right_arm_open: msg.next_is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallArmOperation_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallArmOperation_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallArmOperation_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_Feedback {
  type RmwMsg = super::action::rmw::BallArmOperation_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallArmOperation_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::BallArmOperation_Feedback,

}



impl Default for BallArmOperation_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallArmOperation_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_FeedbackMessage {
  type RmwMsg = super::action::rmw::BallArmOperation_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::BallArmOperation_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::BallArmOperation_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::BallArmOperation_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__MechanismReset_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MechanismReset_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub execute: bool,

}



impl Default for MechanismReset_Goal {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MechanismReset_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_Goal {
  type RmwMsg = super::action::rmw::MechanismReset_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        execute: msg.execute,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      execute: msg.execute,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      execute: msg.execute,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__MechanismReset_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MechanismReset_Result::default())
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_Result {
  type RmwMsg = super::action::rmw::MechanismReset_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        next_state: msg.next_state,
        next_carry: msg.next_carry,
        next_push_state: msg.next_push_state,
        next_is_left_arm_open: msg.next_is_left_arm_open,
        next_is_right_arm_open: msg.next_is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__MechanismReset_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MechanismReset_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for MechanismReset_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MechanismReset_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_Feedback {
  type RmwMsg = super::action::rmw::MechanismReset_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__MechanismReset_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MechanismReset_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::MechanismReset_Feedback,

}



impl Default for MechanismReset_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MechanismReset_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_FeedbackMessage {
  type RmwMsg = super::action::rmw::MechanismReset_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::MechanismReset_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::MechanismReset_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::MechanismReset_Feedback::from_rmw_message(msg.feedback),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_Goal

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallVomitPlate_Goal::default())
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_Goal {
  type RmwMsg = super::action::rmw::BallVomitPlate_Goal;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        execute_mode: msg.execute_mode,
        current_state: msg.current_state,
        carry: msg.carry,
        push_state: msg.push_state,
        shoot_angle_state: msg.shoot_angle_state,
        is_left_arm_open: msg.is_left_arm_open,
        is_right_arm_open: msg.is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      execute_mode: msg.execute_mode,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      execute_mode: msg.execute_mode,
      current_state: msg.current_state,
      carry: msg.carry,
      push_state: msg.push_state,
      shoot_angle_state: msg.shoot_angle_state,
      is_left_arm_open: msg.is_left_arm_open,
      is_right_arm_open: msg.is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_Result

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallVomitPlate_Result::default())
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_Result {
  type RmwMsg = super::action::rmw::BallVomitPlate_Result;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        next_state: msg.next_state,
        next_carry: msg.next_carry,
        next_push_state: msg.next_push_state,
        next_shoot_angle_state: msg.next_shoot_angle_state,
        next_is_left_arm_open: msg.next_is_left_arm_open,
        next_is_right_arm_open: msg.next_is_right_arm_open,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      next_state: msg.next_state,
      next_carry: msg.next_carry,
      next_push_state: msg.next_push_state,
      next_shoot_angle_state: msg.next_shoot_angle_state,
      next_is_left_arm_open: msg.next_is_left_arm_open,
      next_is_right_arm_open: msg.next_is_right_arm_open,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_Feedback

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallVomitPlate_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub structure_needs_at_least_one_member: u8,

}



impl Default for BallVomitPlate_Feedback {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallVomitPlate_Feedback::default())
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_Feedback {
  type RmwMsg = super::action::rmw::BallVomitPlate_Feedback;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      structure_needs_at_least_one_member: msg.structure_needs_at_least_one_member,
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_FeedbackMessage

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallVomitPlate_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::action::BallVomitPlate_Feedback,

}



impl Default for BallVomitPlate_FeedbackMessage {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallVomitPlate_FeedbackMessage::default())
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_FeedbackMessage {
  type RmwMsg = super::action::rmw::BallVomitPlate_FeedbackMessage;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        feedback: super::action::BallVomitPlate_Feedback::into_rmw_message(std::borrow::Cow::Owned(msg.feedback)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        feedback: super::action::BallVomitPlate_Feedback::into_rmw_message(std::borrow::Cow::Borrowed(&msg.feedback)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      feedback: super::action::BallVomitPlate_Feedback::from_rmw_message(msg.feedback),
    }
  }
}






// Corresponds to tourobo_2026_interfaces__action__BallGet_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallGet_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::BallGet_Goal,

}



impl Default for BallGet_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallGet_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallGet_SendGoal_Request {
  type RmwMsg = super::action::rmw::BallGet_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::BallGet_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::BallGet_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::BallGet_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallGet_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallGet_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for BallGet_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallGet_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallGet_SendGoal_Response {
  type RmwMsg = super::action::rmw::BallGet_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallGet_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallGet_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for BallGet_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallGet_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallGet_GetResult_Request {
  type RmwMsg = super::action::rmw::BallGet_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallGet_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallGet_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::BallGet_Result,

}



impl Default for BallGet_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallGet_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallGet_GetResult_Response {
  type RmwMsg = super::action::rmw::BallGet_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::BallGet_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::BallGet_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::BallGet_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutGate_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutGate_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::BallPutGate_Goal,

}



impl Default for BallPutGate_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutGate_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_SendGoal_Request {
  type RmwMsg = super::action::rmw::BallPutGate_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::BallPutGate_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::BallPutGate_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::BallPutGate_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutGate_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutGate_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for BallPutGate_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutGate_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_SendGoal_Response {
  type RmwMsg = super::action::rmw::BallPutGate_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutGate_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutGate_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for BallPutGate_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutGate_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_GetResult_Request {
  type RmwMsg = super::action::rmw::BallPutGate_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutGate_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutGate_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::BallPutGate_Result,

}



impl Default for BallPutGate_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutGate_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutGate_GetResult_Response {
  type RmwMsg = super::action::rmw::BallPutGate_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::BallPutGate_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::BallPutGate_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::BallPutGate_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutPlate_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::BallPutPlate_Goal,

}



impl Default for BallPutPlate_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutPlate_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_SendGoal_Request {
  type RmwMsg = super::action::rmw::BallPutPlate_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::BallPutPlate_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::BallPutPlate_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::BallPutPlate_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutPlate_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for BallPutPlate_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutPlate_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_SendGoal_Response {
  type RmwMsg = super::action::rmw::BallPutPlate_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutPlate_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for BallPutPlate_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutPlate_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_GetResult_Request {
  type RmwMsg = super::action::rmw::BallPutPlate_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallPutPlate_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallPutPlate_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::BallPutPlate_Result,

}



impl Default for BallPutPlate_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallPutPlate_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallPutPlate_GetResult_Response {
  type RmwMsg = super::action::rmw::BallPutPlate_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::BallPutPlate_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::BallPutPlate_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::BallPutPlate_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShoot_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShoot_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::BallShoot_Goal,

}



impl Default for BallShoot_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShoot_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallShoot_SendGoal_Request {
  type RmwMsg = super::action::rmw::BallShoot_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::BallShoot_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::BallShoot_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::BallShoot_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShoot_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShoot_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for BallShoot_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShoot_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallShoot_SendGoal_Response {
  type RmwMsg = super::action::rmw::BallShoot_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShoot_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShoot_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for BallShoot_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShoot_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallShoot_GetResult_Request {
  type RmwMsg = super::action::rmw::BallShoot_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShoot_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShoot_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::BallShoot_Result,

}



impl Default for BallShoot_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShoot_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallShoot_GetResult_Response {
  type RmwMsg = super::action::rmw::BallShoot_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::BallShoot_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::BallShoot_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::BallShoot_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShootAim_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShootAim_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::BallShootAim_Goal,

}



impl Default for BallShootAim_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShootAim_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_SendGoal_Request {
  type RmwMsg = super::action::rmw::BallShootAim_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::BallShootAim_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::BallShootAim_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::BallShootAim_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShootAim_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShootAim_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for BallShootAim_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShootAim_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_SendGoal_Response {
  type RmwMsg = super::action::rmw::BallShootAim_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShootAim_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShootAim_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for BallShootAim_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShootAim_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_GetResult_Request {
  type RmwMsg = super::action::rmw::BallShootAim_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallShootAim_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallShootAim_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::BallShootAim_Result,

}



impl Default for BallShootAim_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallShootAim_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallShootAim_GetResult_Response {
  type RmwMsg = super::action::rmw::BallShootAim_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::BallShootAim_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::BallShootAim_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::BallShootAim_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallIntake_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallIntake_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::BallIntake_Goal,

}



impl Default for BallIntake_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallIntake_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallIntake_SendGoal_Request {
  type RmwMsg = super::action::rmw::BallIntake_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::BallIntake_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::BallIntake_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::BallIntake_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallIntake_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallIntake_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for BallIntake_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallIntake_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallIntake_SendGoal_Response {
  type RmwMsg = super::action::rmw::BallIntake_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallIntake_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallIntake_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for BallIntake_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallIntake_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallIntake_GetResult_Request {
  type RmwMsg = super::action::rmw::BallIntake_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallIntake_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallIntake_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::BallIntake_Result,

}



impl Default for BallIntake_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallIntake_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallIntake_GetResult_Response {
  type RmwMsg = super::action::rmw::BallIntake_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::BallIntake_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::BallIntake_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::BallIntake_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallArmOperation_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::BallArmOperation_Goal,

}



impl Default for BallArmOperation_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallArmOperation_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_SendGoal_Request {
  type RmwMsg = super::action::rmw::BallArmOperation_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::BallArmOperation_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::BallArmOperation_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::BallArmOperation_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallArmOperation_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for BallArmOperation_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallArmOperation_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_SendGoal_Response {
  type RmwMsg = super::action::rmw::BallArmOperation_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallArmOperation_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for BallArmOperation_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallArmOperation_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_GetResult_Request {
  type RmwMsg = super::action::rmw::BallArmOperation_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallArmOperation_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallArmOperation_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::BallArmOperation_Result,

}



impl Default for BallArmOperation_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallArmOperation_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallArmOperation_GetResult_Response {
  type RmwMsg = super::action::rmw::BallArmOperation_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::BallArmOperation_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::BallArmOperation_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::BallArmOperation_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__MechanismReset_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MechanismReset_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::MechanismReset_Goal,

}



impl Default for MechanismReset_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MechanismReset_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_SendGoal_Request {
  type RmwMsg = super::action::rmw::MechanismReset_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::MechanismReset_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::MechanismReset_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::MechanismReset_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__MechanismReset_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MechanismReset_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for MechanismReset_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MechanismReset_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_SendGoal_Response {
  type RmwMsg = super::action::rmw::MechanismReset_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__MechanismReset_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MechanismReset_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for MechanismReset_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MechanismReset_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_GetResult_Request {
  type RmwMsg = super::action::rmw::MechanismReset_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__MechanismReset_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MechanismReset_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::MechanismReset_Result,

}



impl Default for MechanismReset_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::MechanismReset_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MechanismReset_GetResult_Response {
  type RmwMsg = super::action::rmw::MechanismReset_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::MechanismReset_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::MechanismReset_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::MechanismReset_Result::from_rmw_message(msg.result),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallVomitPlate_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::action::BallVomitPlate_Goal,

}



impl Default for BallVomitPlate_SendGoal_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallVomitPlate_SendGoal_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_SendGoal_Request {
  type RmwMsg = super::action::rmw::BallVomitPlate_SendGoal_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
        goal: super::action::BallVomitPlate_Goal::into_rmw_message(std::borrow::Cow::Owned(msg.goal)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
        goal: super::action::BallVomitPlate_Goal::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
      goal: super::action::BallVomitPlate_Goal::from_rmw_message(msg.goal),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_SendGoal_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallVomitPlate_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,

}



impl Default for BallVomitPlate_SendGoal_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallVomitPlate_SendGoal_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_SendGoal_Response {
  type RmwMsg = super::action::rmw::BallVomitPlate_SendGoal_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      accepted: msg.accepted,
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      accepted: msg.accepted,
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallVomitPlate_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::UUID,

}



impl Default for BallVomitPlate_GetResult_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallVomitPlate_GetResult_Request::default())
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_GetResult_Request {
  type RmwMsg = super::action::rmw::BallVomitPlate_GetResult_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Owned(msg.goal_id)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        goal_id: unique_identifier_msgs::msg::UUID::into_rmw_message(std::borrow::Cow::Borrowed(&msg.goal_id)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      goal_id: unique_identifier_msgs::msg::UUID::from_rmw_message(msg.goal_id),
    }
  }
}


// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate_GetResult_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BallVomitPlate_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::action::BallVomitPlate_Result,

}



impl Default for BallVomitPlate_GetResult_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::action::rmw::BallVomitPlate_GetResult_Response::default())
  }
}

impl rosidl_runtime_rs::Message for BallVomitPlate_GetResult_Response {
  type RmwMsg = super::action::rmw::BallVomitPlate_GetResult_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        status: msg.status,
        result: super::action::BallVomitPlate_Result::into_rmw_message(std::borrow::Cow::Owned(msg.result)).into_owned(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      status: msg.status,
        result: super::action::BallVomitPlate_Result::into_rmw_message(std::borrow::Cow::Borrowed(&msg.result)).into_owned(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      status: msg.status,
      result: super::action::BallVomitPlate_Result::from_rmw_message(msg.result),
    }
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






#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallGet() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallGet
#[allow(missing_docs, non_camel_case_types)]
pub struct BallGet;

impl rosidl_runtime_rs::Action for BallGet {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = BallGet_Goal;

  /// The result message defined in the action definition.
  type Result = BallGet_Result;

  /// The feedback message defined in the action definition.
  type Feedback = BallGet_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::BallGet_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::BallGet_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::BallGet_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallGet() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::BallGet_Goal,
  ) -> super::action::rmw::BallGet_SendGoal_Request {
   super::action::rmw::BallGet_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::BallGet_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::BallGet_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::BallGet_SendGoal_Response {
   super::action::rmw::BallGet_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::BallGet_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::BallGet_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::BallGet_Feedback,
  ) -> super::action::rmw::BallGet_FeedbackMessage {
    let mut message = super::action::rmw::BallGet_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::BallGet_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::BallGet_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::BallGet_GetResult_Request {
   super::action::rmw::BallGet_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::BallGet_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::BallGet_Result,
  ) -> super::action::rmw::BallGet_GetResult_Response {
   super::action::rmw::BallGet_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::BallGet_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::BallGet_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallPutGate() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutGate
#[allow(missing_docs, non_camel_case_types)]
pub struct BallPutGate;

impl rosidl_runtime_rs::Action for BallPutGate {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = BallPutGate_Goal;

  /// The result message defined in the action definition.
  type Result = BallPutGate_Result;

  /// The feedback message defined in the action definition.
  type Feedback = BallPutGate_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::BallPutGate_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::BallPutGate_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::BallPutGate_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallPutGate() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::BallPutGate_Goal,
  ) -> super::action::rmw::BallPutGate_SendGoal_Request {
   super::action::rmw::BallPutGate_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::BallPutGate_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::BallPutGate_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::BallPutGate_SendGoal_Response {
   super::action::rmw::BallPutGate_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::BallPutGate_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::BallPutGate_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::BallPutGate_Feedback,
  ) -> super::action::rmw::BallPutGate_FeedbackMessage {
    let mut message = super::action::rmw::BallPutGate_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::BallPutGate_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::BallPutGate_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::BallPutGate_GetResult_Request {
   super::action::rmw::BallPutGate_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::BallPutGate_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::BallPutGate_Result,
  ) -> super::action::rmw::BallPutGate_GetResult_Response {
   super::action::rmw::BallPutGate_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::BallPutGate_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::BallPutGate_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallPutPlate
#[allow(missing_docs, non_camel_case_types)]
pub struct BallPutPlate;

impl rosidl_runtime_rs::Action for BallPutPlate {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = BallPutPlate_Goal;

  /// The result message defined in the action definition.
  type Result = BallPutPlate_Result;

  /// The feedback message defined in the action definition.
  type Feedback = BallPutPlate_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::BallPutPlate_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::BallPutPlate_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::BallPutPlate_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallPutPlate() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::BallPutPlate_Goal,
  ) -> super::action::rmw::BallPutPlate_SendGoal_Request {
   super::action::rmw::BallPutPlate_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::BallPutPlate_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::BallPutPlate_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::BallPutPlate_SendGoal_Response {
   super::action::rmw::BallPutPlate_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::BallPutPlate_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::BallPutPlate_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::BallPutPlate_Feedback,
  ) -> super::action::rmw::BallPutPlate_FeedbackMessage {
    let mut message = super::action::rmw::BallPutPlate_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::BallPutPlate_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::BallPutPlate_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::BallPutPlate_GetResult_Request {
   super::action::rmw::BallPutPlate_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::BallPutPlate_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::BallPutPlate_Result,
  ) -> super::action::rmw::BallPutPlate_GetResult_Response {
   super::action::rmw::BallPutPlate_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::BallPutPlate_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::BallPutPlate_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallShoot() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallShoot
#[allow(missing_docs, non_camel_case_types)]
pub struct BallShoot;

impl rosidl_runtime_rs::Action for BallShoot {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = BallShoot_Goal;

  /// The result message defined in the action definition.
  type Result = BallShoot_Result;

  /// The feedback message defined in the action definition.
  type Feedback = BallShoot_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::BallShoot_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::BallShoot_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::BallShoot_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallShoot() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::BallShoot_Goal,
  ) -> super::action::rmw::BallShoot_SendGoal_Request {
   super::action::rmw::BallShoot_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::BallShoot_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::BallShoot_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::BallShoot_SendGoal_Response {
   super::action::rmw::BallShoot_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::BallShoot_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::BallShoot_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::BallShoot_Feedback,
  ) -> super::action::rmw::BallShoot_FeedbackMessage {
    let mut message = super::action::rmw::BallShoot_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::BallShoot_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::BallShoot_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::BallShoot_GetResult_Request {
   super::action::rmw::BallShoot_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::BallShoot_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::BallShoot_Result,
  ) -> super::action::rmw::BallShoot_GetResult_Response {
   super::action::rmw::BallShoot_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::BallShoot_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::BallShoot_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallShootAim() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallShootAim
#[allow(missing_docs, non_camel_case_types)]
pub struct BallShootAim;

impl rosidl_runtime_rs::Action for BallShootAim {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = BallShootAim_Goal;

  /// The result message defined in the action definition.
  type Result = BallShootAim_Result;

  /// The feedback message defined in the action definition.
  type Feedback = BallShootAim_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::BallShootAim_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::BallShootAim_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::BallShootAim_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallShootAim() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::BallShootAim_Goal,
  ) -> super::action::rmw::BallShootAim_SendGoal_Request {
   super::action::rmw::BallShootAim_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::BallShootAim_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::BallShootAim_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::BallShootAim_SendGoal_Response {
   super::action::rmw::BallShootAim_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::BallShootAim_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::BallShootAim_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::BallShootAim_Feedback,
  ) -> super::action::rmw::BallShootAim_FeedbackMessage {
    let mut message = super::action::rmw::BallShootAim_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::BallShootAim_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::BallShootAim_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::BallShootAim_GetResult_Request {
   super::action::rmw::BallShootAim_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::BallShootAim_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::BallShootAim_Result,
  ) -> super::action::rmw::BallShootAim_GetResult_Response {
   super::action::rmw::BallShootAim_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::BallShootAim_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::BallShootAim_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallIntake() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallIntake
#[allow(missing_docs, non_camel_case_types)]
pub struct BallIntake;

impl rosidl_runtime_rs::Action for BallIntake {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = BallIntake_Goal;

  /// The result message defined in the action definition.
  type Result = BallIntake_Result;

  /// The feedback message defined in the action definition.
  type Feedback = BallIntake_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::BallIntake_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::BallIntake_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::BallIntake_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallIntake() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::BallIntake_Goal,
  ) -> super::action::rmw::BallIntake_SendGoal_Request {
   super::action::rmw::BallIntake_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::BallIntake_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::BallIntake_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::BallIntake_SendGoal_Response {
   super::action::rmw::BallIntake_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::BallIntake_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::BallIntake_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::BallIntake_Feedback,
  ) -> super::action::rmw::BallIntake_FeedbackMessage {
    let mut message = super::action::rmw::BallIntake_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::BallIntake_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::BallIntake_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::BallIntake_GetResult_Request {
   super::action::rmw::BallIntake_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::BallIntake_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::BallIntake_Result,
  ) -> super::action::rmw::BallIntake_GetResult_Response {
   super::action::rmw::BallIntake_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::BallIntake_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::BallIntake_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallArmOperation
#[allow(missing_docs, non_camel_case_types)]
pub struct BallArmOperation;

impl rosidl_runtime_rs::Action for BallArmOperation {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = BallArmOperation_Goal;

  /// The result message defined in the action definition.
  type Result = BallArmOperation_Result;

  /// The feedback message defined in the action definition.
  type Feedback = BallArmOperation_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::BallArmOperation_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::BallArmOperation_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::BallArmOperation_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallArmOperation() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::BallArmOperation_Goal,
  ) -> super::action::rmw::BallArmOperation_SendGoal_Request {
   super::action::rmw::BallArmOperation_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::BallArmOperation_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::BallArmOperation_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::BallArmOperation_SendGoal_Response {
   super::action::rmw::BallArmOperation_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::BallArmOperation_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::BallArmOperation_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::BallArmOperation_Feedback,
  ) -> super::action::rmw::BallArmOperation_FeedbackMessage {
    let mut message = super::action::rmw::BallArmOperation_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::BallArmOperation_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::BallArmOperation_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::BallArmOperation_GetResult_Request {
   super::action::rmw::BallArmOperation_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::BallArmOperation_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::BallArmOperation_Result,
  ) -> super::action::rmw::BallArmOperation_GetResult_Response {
   super::action::rmw::BallArmOperation_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::BallArmOperation_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::BallArmOperation_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__MechanismReset() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__MechanismReset
#[allow(missing_docs, non_camel_case_types)]
pub struct MechanismReset;

impl rosidl_runtime_rs::Action for MechanismReset {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = MechanismReset_Goal;

  /// The result message defined in the action definition.
  type Result = MechanismReset_Result;

  /// The feedback message defined in the action definition.
  type Feedback = MechanismReset_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::MechanismReset_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::MechanismReset_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::MechanismReset_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__MechanismReset() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::MechanismReset_Goal,
  ) -> super::action::rmw::MechanismReset_SendGoal_Request {
   super::action::rmw::MechanismReset_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::MechanismReset_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::MechanismReset_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::MechanismReset_SendGoal_Response {
   super::action::rmw::MechanismReset_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::MechanismReset_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::MechanismReset_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::MechanismReset_Feedback,
  ) -> super::action::rmw::MechanismReset_FeedbackMessage {
    let mut message = super::action::rmw::MechanismReset_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::MechanismReset_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::MechanismReset_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::MechanismReset_GetResult_Request {
   super::action::rmw::MechanismReset_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::MechanismReset_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::MechanismReset_Result,
  ) -> super::action::rmw::MechanismReset_GetResult_Response {
   super::action::rmw::MechanismReset_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::MechanismReset_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::MechanismReset_Result,
  ) {
    (response.status, response.result)
  }
}




#[link(name = "tourobo_2026_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate() -> *const std::ffi::c_void;
}

// Corresponds to tourobo_2026_interfaces__action__BallVomitPlate
#[allow(missing_docs, non_camel_case_types)]
pub struct BallVomitPlate;

impl rosidl_runtime_rs::Action for BallVomitPlate {
  // --- Associated types for client library users ---
  /// The goal message defined in the action definition.
  type Goal = BallVomitPlate_Goal;

  /// The result message defined in the action definition.
  type Result = BallVomitPlate_Result;

  /// The feedback message defined in the action definition.
  type Feedback = BallVomitPlate_Feedback;

  // --- Associated types for client library implementation ---
  /// The feedback message with generic fields which wraps the feedback message.
  type FeedbackMessage = super::action::BallVomitPlate_FeedbackMessage;

  /// The send_goal service using a wrapped version of the goal message as a request.
  type SendGoalService = super::action::BallVomitPlate_SendGoal;

  /// The generic service to cancel a goal.
  type CancelGoalService = action_msgs::srv::rmw::CancelGoal;

  /// The get_result service using a wrapped version of the result message as a response.
  type GetResultService = super::action::BallVomitPlate_GetResult;

  // --- Methods for client library implementation ---
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_action_type_support_handle__tourobo_2026_interfaces__action__BallVomitPlate() }
  }

  fn create_goal_request(
    goal_id: &[u8; 16],
    goal: super::action::rmw::BallVomitPlate_Goal,
  ) -> super::action::rmw::BallVomitPlate_SendGoal_Request {
   super::action::rmw::BallVomitPlate_SendGoal_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
      goal,
    }
  }

  fn split_goal_request(
    request: super::action::rmw::BallVomitPlate_SendGoal_Request,
  ) -> (
    [u8; 16],
   super::action::rmw::BallVomitPlate_Goal,
  ) {
    (request.goal_id.uuid, request.goal)
  }

  fn create_goal_response(
    accepted: bool,
    stamp: (i32, u32),
  ) -> super::action::rmw::BallVomitPlate_SendGoal_Response {
   super::action::rmw::BallVomitPlate_SendGoal_Response {
      accepted,
      stamp: builtin_interfaces::msg::rmw::Time {
        sec: stamp.0,
        nanosec: stamp.1,
      },
    }
  }

  fn get_goal_response_accepted(
    response: &super::action::rmw::BallVomitPlate_SendGoal_Response,
  ) -> bool {
    response.accepted
  }

  fn get_goal_response_stamp(
    response: &super::action::rmw::BallVomitPlate_SendGoal_Response,
  ) -> (i32, u32) {
    (response.stamp.sec, response.stamp.nanosec)
  }

  fn create_feedback_message(
    goal_id: &[u8; 16],
    feedback: super::action::rmw::BallVomitPlate_Feedback,
  ) -> super::action::rmw::BallVomitPlate_FeedbackMessage {
    let mut message = super::action::rmw::BallVomitPlate_FeedbackMessage::default();
    message.goal_id.uuid = *goal_id;
    message.feedback = feedback;
    message
  }

  fn split_feedback_message(
    feedback: super::action::rmw::BallVomitPlate_FeedbackMessage,
  ) -> (
    [u8; 16],
   super::action::rmw::BallVomitPlate_Feedback,
  ) {
    (feedback.goal_id.uuid, feedback.feedback)
  }

  fn create_result_request(
    goal_id: &[u8; 16],
  ) -> super::action::rmw::BallVomitPlate_GetResult_Request {
   super::action::rmw::BallVomitPlate_GetResult_Request {
      goal_id: unique_identifier_msgs::msg::rmw::UUID { uuid: *goal_id },
    }
  }

  fn get_result_request_uuid(
    request: &super::action::rmw::BallVomitPlate_GetResult_Request,
  ) -> &[u8; 16] {
    &request.goal_id.uuid
  }

  fn create_result_response(
    status: i8,
    result: super::action::rmw::BallVomitPlate_Result,
  ) -> super::action::rmw::BallVomitPlate_GetResult_Response {
   super::action::rmw::BallVomitPlate_GetResult_Response {
      status,
      result,
    }
  }

  fn split_result_response(
    response: super::action::rmw::BallVomitPlate_GetResult_Response
  ) -> (
    i8,
   super::action::rmw::BallVomitPlate_Result,
  ) {
    (response.status, response.result)
  }
}


