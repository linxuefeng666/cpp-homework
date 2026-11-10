// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from action_sum_cpp:action/Sum.idl
// generated code does not contain a copyright notice

#ifndef ACTION_SUM_CPP__ACTION__DETAIL__SUM__BUILDER_HPP_
#define ACTION_SUM_CPP__ACTION__DETAIL__SUM__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "action_sum_cpp/action/detail/sum__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace action_sum_cpp
{

namespace action
{

namespace builder
{

class Init_Sum_Goal_n
{
public:
  Init_Sum_Goal_n()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::action_sum_cpp::action::Sum_Goal n(::action_sum_cpp::action::Sum_Goal::_n_type arg)
  {
    msg_.n = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_sum_cpp::action::Sum_Goal msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_sum_cpp::action::Sum_Goal>()
{
  return action_sum_cpp::action::builder::Init_Sum_Goal_n();
}

}  // namespace action_sum_cpp


namespace action_sum_cpp
{

namespace action
{

namespace builder
{

class Init_Sum_Result_total_sum
{
public:
  Init_Sum_Result_total_sum()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::action_sum_cpp::action::Sum_Result total_sum(::action_sum_cpp::action::Sum_Result::_total_sum_type arg)
  {
    msg_.total_sum = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_sum_cpp::action::Sum_Result msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_sum_cpp::action::Sum_Result>()
{
  return action_sum_cpp::action::builder::Init_Sum_Result_total_sum();
}

}  // namespace action_sum_cpp


namespace action_sum_cpp
{

namespace action
{

namespace builder
{

class Init_Sum_Feedback_current_sum
{
public:
  explicit Init_Sum_Feedback_current_sum(::action_sum_cpp::action::Sum_Feedback & msg)
  : msg_(msg)
  {}
  ::action_sum_cpp::action::Sum_Feedback current_sum(::action_sum_cpp::action::Sum_Feedback::_current_sum_type arg)
  {
    msg_.current_sum = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_sum_cpp::action::Sum_Feedback msg_;
};

class Init_Sum_Feedback_current_num
{
public:
  Init_Sum_Feedback_current_num()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Sum_Feedback_current_sum current_num(::action_sum_cpp::action::Sum_Feedback::_current_num_type arg)
  {
    msg_.current_num = std::move(arg);
    return Init_Sum_Feedback_current_sum(msg_);
  }

private:
  ::action_sum_cpp::action::Sum_Feedback msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_sum_cpp::action::Sum_Feedback>()
{
  return action_sum_cpp::action::builder::Init_Sum_Feedback_current_num();
}

}  // namespace action_sum_cpp


namespace action_sum_cpp
{

namespace action
{

namespace builder
{

class Init_Sum_SendGoal_Request_goal
{
public:
  explicit Init_Sum_SendGoal_Request_goal(::action_sum_cpp::action::Sum_SendGoal_Request & msg)
  : msg_(msg)
  {}
  ::action_sum_cpp::action::Sum_SendGoal_Request goal(::action_sum_cpp::action::Sum_SendGoal_Request::_goal_type arg)
  {
    msg_.goal = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_sum_cpp::action::Sum_SendGoal_Request msg_;
};

class Init_Sum_SendGoal_Request_goal_id
{
public:
  Init_Sum_SendGoal_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Sum_SendGoal_Request_goal goal_id(::action_sum_cpp::action::Sum_SendGoal_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_Sum_SendGoal_Request_goal(msg_);
  }

private:
  ::action_sum_cpp::action::Sum_SendGoal_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_sum_cpp::action::Sum_SendGoal_Request>()
{
  return action_sum_cpp::action::builder::Init_Sum_SendGoal_Request_goal_id();
}

}  // namespace action_sum_cpp


namespace action_sum_cpp
{

namespace action
{

namespace builder
{

class Init_Sum_SendGoal_Response_stamp
{
public:
  explicit Init_Sum_SendGoal_Response_stamp(::action_sum_cpp::action::Sum_SendGoal_Response & msg)
  : msg_(msg)
  {}
  ::action_sum_cpp::action::Sum_SendGoal_Response stamp(::action_sum_cpp::action::Sum_SendGoal_Response::_stamp_type arg)
  {
    msg_.stamp = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_sum_cpp::action::Sum_SendGoal_Response msg_;
};

class Init_Sum_SendGoal_Response_accepted
{
public:
  Init_Sum_SendGoal_Response_accepted()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Sum_SendGoal_Response_stamp accepted(::action_sum_cpp::action::Sum_SendGoal_Response::_accepted_type arg)
  {
    msg_.accepted = std::move(arg);
    return Init_Sum_SendGoal_Response_stamp(msg_);
  }

private:
  ::action_sum_cpp::action::Sum_SendGoal_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_sum_cpp::action::Sum_SendGoal_Response>()
{
  return action_sum_cpp::action::builder::Init_Sum_SendGoal_Response_accepted();
}

}  // namespace action_sum_cpp


namespace action_sum_cpp
{

namespace action
{

namespace builder
{

class Init_Sum_GetResult_Request_goal_id
{
public:
  Init_Sum_GetResult_Request_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::action_sum_cpp::action::Sum_GetResult_Request goal_id(::action_sum_cpp::action::Sum_GetResult_Request::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_sum_cpp::action::Sum_GetResult_Request msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_sum_cpp::action::Sum_GetResult_Request>()
{
  return action_sum_cpp::action::builder::Init_Sum_GetResult_Request_goal_id();
}

}  // namespace action_sum_cpp


namespace action_sum_cpp
{

namespace action
{

namespace builder
{

class Init_Sum_GetResult_Response_result
{
public:
  explicit Init_Sum_GetResult_Response_result(::action_sum_cpp::action::Sum_GetResult_Response & msg)
  : msg_(msg)
  {}
  ::action_sum_cpp::action::Sum_GetResult_Response result(::action_sum_cpp::action::Sum_GetResult_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_sum_cpp::action::Sum_GetResult_Response msg_;
};

class Init_Sum_GetResult_Response_status
{
public:
  Init_Sum_GetResult_Response_status()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Sum_GetResult_Response_result status(::action_sum_cpp::action::Sum_GetResult_Response::_status_type arg)
  {
    msg_.status = std::move(arg);
    return Init_Sum_GetResult_Response_result(msg_);
  }

private:
  ::action_sum_cpp::action::Sum_GetResult_Response msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_sum_cpp::action::Sum_GetResult_Response>()
{
  return action_sum_cpp::action::builder::Init_Sum_GetResult_Response_status();
}

}  // namespace action_sum_cpp


namespace action_sum_cpp
{

namespace action
{

namespace builder
{

class Init_Sum_FeedbackMessage_feedback
{
public:
  explicit Init_Sum_FeedbackMessage_feedback(::action_sum_cpp::action::Sum_FeedbackMessage & msg)
  : msg_(msg)
  {}
  ::action_sum_cpp::action::Sum_FeedbackMessage feedback(::action_sum_cpp::action::Sum_FeedbackMessage::_feedback_type arg)
  {
    msg_.feedback = std::move(arg);
    return std::move(msg_);
  }

private:
  ::action_sum_cpp::action::Sum_FeedbackMessage msg_;
};

class Init_Sum_FeedbackMessage_goal_id
{
public:
  Init_Sum_FeedbackMessage_goal_id()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Sum_FeedbackMessage_feedback goal_id(::action_sum_cpp::action::Sum_FeedbackMessage::_goal_id_type arg)
  {
    msg_.goal_id = std::move(arg);
    return Init_Sum_FeedbackMessage_feedback(msg_);
  }

private:
  ::action_sum_cpp::action::Sum_FeedbackMessage msg_;
};

}  // namespace builder

}  // namespace action

template<typename MessageType>
auto build();

template<>
inline
auto build<::action_sum_cpp::action::Sum_FeedbackMessage>()
{
  return action_sum_cpp::action::builder::Init_Sum_FeedbackMessage_goal_id();
}

}  // namespace action_sum_cpp

#endif  // ACTION_SUM_CPP__ACTION__DETAIL__SUM__BUILDER_HPP_
