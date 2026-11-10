
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "action_sum_cpp__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_Goal() -> *const std::ffi::c_void;
}

#[link(name = "action_sum_cpp__rosidl_generator_c")]
extern "C" {
    fn action_sum_cpp__action__Sum_Goal__init(msg: *mut Sum_Goal) -> bool;
    fn action_sum_cpp__action__Sum_Goal__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Sum_Goal>, size: usize) -> bool;
    fn action_sum_cpp__action__Sum_Goal__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Sum_Goal>);
    fn action_sum_cpp__action__Sum_Goal__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Sum_Goal>, out_seq: *mut rosidl_runtime_rs::Sequence<Sum_Goal>) -> bool;
}

// Corresponds to action_sum_cpp__action__Sum_Goal
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sum_Goal {

    // This member is not documented.
    #[allow(missing_docs)]
    pub n: i32,

}



impl Default for Sum_Goal {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !action_sum_cpp__action__Sum_Goal__init(&mut msg as *mut _) {
        panic!("Call to action_sum_cpp__action__Sum_Goal__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Sum_Goal {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_Goal__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_Goal__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_Goal__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Sum_Goal {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Sum_Goal where Self: Sized {
  const TYPE_NAME: &'static str = "action_sum_cpp/action/Sum_Goal";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_Goal() }
  }
}


#[link(name = "action_sum_cpp__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_Result() -> *const std::ffi::c_void;
}

#[link(name = "action_sum_cpp__rosidl_generator_c")]
extern "C" {
    fn action_sum_cpp__action__Sum_Result__init(msg: *mut Sum_Result) -> bool;
    fn action_sum_cpp__action__Sum_Result__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Sum_Result>, size: usize) -> bool;
    fn action_sum_cpp__action__Sum_Result__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Sum_Result>);
    fn action_sum_cpp__action__Sum_Result__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Sum_Result>, out_seq: *mut rosidl_runtime_rs::Sequence<Sum_Result>) -> bool;
}

// Corresponds to action_sum_cpp__action__Sum_Result
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sum_Result {

    // This member is not documented.
    #[allow(missing_docs)]
    pub total_sum: i32,

}



impl Default for Sum_Result {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !action_sum_cpp__action__Sum_Result__init(&mut msg as *mut _) {
        panic!("Call to action_sum_cpp__action__Sum_Result__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Sum_Result {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_Result__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_Result__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_Result__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Sum_Result {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Sum_Result where Self: Sized {
  const TYPE_NAME: &'static str = "action_sum_cpp/action/Sum_Result";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_Result() }
  }
}


#[link(name = "action_sum_cpp__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_Feedback() -> *const std::ffi::c_void;
}

#[link(name = "action_sum_cpp__rosidl_generator_c")]
extern "C" {
    fn action_sum_cpp__action__Sum_Feedback__init(msg: *mut Sum_Feedback) -> bool;
    fn action_sum_cpp__action__Sum_Feedback__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Sum_Feedback>, size: usize) -> bool;
    fn action_sum_cpp__action__Sum_Feedback__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Sum_Feedback>);
    fn action_sum_cpp__action__Sum_Feedback__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Sum_Feedback>, out_seq: *mut rosidl_runtime_rs::Sequence<Sum_Feedback>) -> bool;
}

// Corresponds to action_sum_cpp__action__Sum_Feedback
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sum_Feedback {

    // This member is not documented.
    #[allow(missing_docs)]
    pub current_num: i32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub current_sum: i32,

}



impl Default for Sum_Feedback {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !action_sum_cpp__action__Sum_Feedback__init(&mut msg as *mut _) {
        panic!("Call to action_sum_cpp__action__Sum_Feedback__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Sum_Feedback {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_Feedback__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_Feedback__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_Feedback__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Sum_Feedback {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Sum_Feedback where Self: Sized {
  const TYPE_NAME: &'static str = "action_sum_cpp/action/Sum_Feedback";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_Feedback() }
  }
}


#[link(name = "action_sum_cpp__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_FeedbackMessage() -> *const std::ffi::c_void;
}

#[link(name = "action_sum_cpp__rosidl_generator_c")]
extern "C" {
    fn action_sum_cpp__action__Sum_FeedbackMessage__init(msg: *mut Sum_FeedbackMessage) -> bool;
    fn action_sum_cpp__action__Sum_FeedbackMessage__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Sum_FeedbackMessage>, size: usize) -> bool;
    fn action_sum_cpp__action__Sum_FeedbackMessage__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Sum_FeedbackMessage>);
    fn action_sum_cpp__action__Sum_FeedbackMessage__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Sum_FeedbackMessage>, out_seq: *mut rosidl_runtime_rs::Sequence<Sum_FeedbackMessage>) -> bool;
}

// Corresponds to action_sum_cpp__action__Sum_FeedbackMessage
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sum_FeedbackMessage {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub feedback: super::super::action::rmw::Sum_Feedback,

}



impl Default for Sum_FeedbackMessage {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !action_sum_cpp__action__Sum_FeedbackMessage__init(&mut msg as *mut _) {
        panic!("Call to action_sum_cpp__action__Sum_FeedbackMessage__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Sum_FeedbackMessage {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_FeedbackMessage__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_FeedbackMessage__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_FeedbackMessage__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Sum_FeedbackMessage {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Sum_FeedbackMessage where Self: Sized {
  const TYPE_NAME: &'static str = "action_sum_cpp/action/Sum_FeedbackMessage";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_FeedbackMessage() }
  }
}




#[link(name = "action_sum_cpp__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_SendGoal_Request() -> *const std::ffi::c_void;
}

#[link(name = "action_sum_cpp__rosidl_generator_c")]
extern "C" {
    fn action_sum_cpp__action__Sum_SendGoal_Request__init(msg: *mut Sum_SendGoal_Request) -> bool;
    fn action_sum_cpp__action__Sum_SendGoal_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Sum_SendGoal_Request>, size: usize) -> bool;
    fn action_sum_cpp__action__Sum_SendGoal_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Sum_SendGoal_Request>);
    fn action_sum_cpp__action__Sum_SendGoal_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Sum_SendGoal_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Sum_SendGoal_Request>) -> bool;
}

// Corresponds to action_sum_cpp__action__Sum_SendGoal_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sum_SendGoal_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,


    // This member is not documented.
    #[allow(missing_docs)]
    pub goal: super::super::action::rmw::Sum_Goal,

}



impl Default for Sum_SendGoal_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !action_sum_cpp__action__Sum_SendGoal_Request__init(&mut msg as *mut _) {
        panic!("Call to action_sum_cpp__action__Sum_SendGoal_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Sum_SendGoal_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_SendGoal_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_SendGoal_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_SendGoal_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Sum_SendGoal_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Sum_SendGoal_Request where Self: Sized {
  const TYPE_NAME: &'static str = "action_sum_cpp/action/Sum_SendGoal_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_SendGoal_Request() }
  }
}


#[link(name = "action_sum_cpp__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_SendGoal_Response() -> *const std::ffi::c_void;
}

#[link(name = "action_sum_cpp__rosidl_generator_c")]
extern "C" {
    fn action_sum_cpp__action__Sum_SendGoal_Response__init(msg: *mut Sum_SendGoal_Response) -> bool;
    fn action_sum_cpp__action__Sum_SendGoal_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Sum_SendGoal_Response>, size: usize) -> bool;
    fn action_sum_cpp__action__Sum_SendGoal_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Sum_SendGoal_Response>);
    fn action_sum_cpp__action__Sum_SendGoal_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Sum_SendGoal_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Sum_SendGoal_Response>) -> bool;
}

// Corresponds to action_sum_cpp__action__Sum_SendGoal_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sum_SendGoal_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub accepted: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,

}



impl Default for Sum_SendGoal_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !action_sum_cpp__action__Sum_SendGoal_Response__init(&mut msg as *mut _) {
        panic!("Call to action_sum_cpp__action__Sum_SendGoal_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Sum_SendGoal_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_SendGoal_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_SendGoal_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_SendGoal_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Sum_SendGoal_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Sum_SendGoal_Response where Self: Sized {
  const TYPE_NAME: &'static str = "action_sum_cpp/action/Sum_SendGoal_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_SendGoal_Response() }
  }
}


#[link(name = "action_sum_cpp__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_GetResult_Request() -> *const std::ffi::c_void;
}

#[link(name = "action_sum_cpp__rosidl_generator_c")]
extern "C" {
    fn action_sum_cpp__action__Sum_GetResult_Request__init(msg: *mut Sum_GetResult_Request) -> bool;
    fn action_sum_cpp__action__Sum_GetResult_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Sum_GetResult_Request>, size: usize) -> bool;
    fn action_sum_cpp__action__Sum_GetResult_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Sum_GetResult_Request>);
    fn action_sum_cpp__action__Sum_GetResult_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Sum_GetResult_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Sum_GetResult_Request>) -> bool;
}

// Corresponds to action_sum_cpp__action__Sum_GetResult_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sum_GetResult_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub goal_id: unique_identifier_msgs::msg::rmw::UUID,

}



impl Default for Sum_GetResult_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !action_sum_cpp__action__Sum_GetResult_Request__init(&mut msg as *mut _) {
        panic!("Call to action_sum_cpp__action__Sum_GetResult_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Sum_GetResult_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_GetResult_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_GetResult_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_GetResult_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Sum_GetResult_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Sum_GetResult_Request where Self: Sized {
  const TYPE_NAME: &'static str = "action_sum_cpp/action/Sum_GetResult_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_GetResult_Request() }
  }
}


#[link(name = "action_sum_cpp__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_GetResult_Response() -> *const std::ffi::c_void;
}

#[link(name = "action_sum_cpp__rosidl_generator_c")]
extern "C" {
    fn action_sum_cpp__action__Sum_GetResult_Response__init(msg: *mut Sum_GetResult_Response) -> bool;
    fn action_sum_cpp__action__Sum_GetResult_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Sum_GetResult_Response>, size: usize) -> bool;
    fn action_sum_cpp__action__Sum_GetResult_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Sum_GetResult_Response>);
    fn action_sum_cpp__action__Sum_GetResult_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Sum_GetResult_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Sum_GetResult_Response>) -> bool;
}

// Corresponds to action_sum_cpp__action__Sum_GetResult_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sum_GetResult_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub status: i8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub result: super::super::action::rmw::Sum_Result,

}



impl Default for Sum_GetResult_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !action_sum_cpp__action__Sum_GetResult_Response__init(&mut msg as *mut _) {
        panic!("Call to action_sum_cpp__action__Sum_GetResult_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Sum_GetResult_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_GetResult_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_GetResult_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { action_sum_cpp__action__Sum_GetResult_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Sum_GetResult_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Sum_GetResult_Response where Self: Sized {
  const TYPE_NAME: &'static str = "action_sum_cpp/action/Sum_GetResult_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__action_sum_cpp__action__Sum_GetResult_Response() }
  }
}






#[link(name = "action_sum_cpp__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__action_sum_cpp__action__Sum_SendGoal() -> *const std::ffi::c_void;
}

// Corresponds to action_sum_cpp__action__Sum_SendGoal
#[allow(missing_docs, non_camel_case_types)]
pub struct Sum_SendGoal;

impl rosidl_runtime_rs::Service for Sum_SendGoal {
    type Request = Sum_SendGoal_Request;
    type Response = Sum_SendGoal_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__action_sum_cpp__action__Sum_SendGoal() }
    }
}




#[link(name = "action_sum_cpp__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__action_sum_cpp__action__Sum_GetResult() -> *const std::ffi::c_void;
}

// Corresponds to action_sum_cpp__action__Sum_GetResult
#[allow(missing_docs, non_camel_case_types)]
pub struct Sum_GetResult;

impl rosidl_runtime_rs::Service for Sum_GetResult {
    type Request = Sum_GetResult_Request;
    type Response = Sum_GetResult_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__action_sum_cpp__action__Sum_GetResult() }
    }
}


