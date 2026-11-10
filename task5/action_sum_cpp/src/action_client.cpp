#include <rclcpp/rclcpp.hpp>
#include <rclcpp_action/rclcpp_action.hpp>
#include "action_sum_cpp/action/sum.hpp"

using namespace std::placeholders;
using Sum = action_sum_cpp::action::Sum;
using GoalHandleSum = rclcpp_action::ClientGoalHandle<Sum>;

class SumActionClient : public rclcpp::Node
{
public:
  explicit SumActionClient(const rclcpp::NodeOptions & options = rclcpp::NodeOptions())
  : Node("sum_action_client", options)
  {
    client_ = rclcpp_action::create_client<Sum>(this, "sum_action");
  }

  void send_goal(int num)
  {
    if (!client_->wait_for_action_server(std::chrono::seconds(5)))
    {
      RCLCPP_ERROR(this->get_logger(), "Action Server 没有上线！");
      return;
    }

    auto goal_msg = Sum::Goal();
    goal_msg.n = num;

    RCLCPP_INFO(this->get_logger(), "发送目标 n = %d", num);

    auto send_goal_options = rclcpp_action::Client<Sum>::SendGoalOptions();
    send_goal_options.goal_response_callback =
      std::bind(&SumActionClient::goal_response_cb, this, _1);
    send_goal_options.feedback_callback =
      std::bind(&SumActionClient::feedback_cb, this, _1, _2);
    send_goal_options.result_callback =
      std::bind(&SumActionClient::result_cb, this, _1);

    client_->async_send_goal(goal_msg, send_goal_options);
  }

private:
  rclcpp_action::Client<Sum>::SharedPtr client_;

  void goal_response_cb(const GoalHandleSum::SharedPtr & goal_handle)
  {
    if (!goal_handle)
    {
      RCLCPP_ERROR(this->get_logger(), "目标被拒绝！");
    }
    else
    {
      RCLCPP_INFO(this->get_logger(), "目标已接受，等待执行");
    }
  }

  void feedback_cb(
    GoalHandleSum::SharedPtr,
    const std::shared_ptr<const Sum::Feedback> feedback)
  {
    RCLCPP_INFO(
      this->get_logger(),
      "收到反馈:current_num=%d, current_sum=%d",
      feedback->current_num, feedback->current_sum
    );
  }

  void result_cb(const GoalHandleSum::WrappedResult & result)
  {
    switch(result.code)
    {
      case rclcpp_action::ResultCode::SUCCEEDED:
        RCLCPP_INFO(this->get_logger(), "成功！最终总和：%d", result.result->total_sum);
        break;
      case rclcpp_action::ResultCode::ABORTED:
        RCLCPP_ERROR(this->get_logger(), "任务被中止");
        break;
      case rclcpp_action::ResultCode::CANCELED:
        RCLCPP_ERROR(this->get_logger(), "任务被取消");
        break;
      default:
        RCLCPP_ERROR(this->get_logger(), "未知结果");
        break;
    }
    rclcpp::shutdown();
  }
};

int main(int argc, char * argv[])
{
  rclcpp::init(argc, argv);
  auto node = std::make_shared<SumActionClient>();
  node->send_goal(10);  
  rclcpp::spin(node);
  rclcpp::shutdown();
  return 0;
}