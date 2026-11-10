#include <rclcpp/rclcpp.hpp>
#include <rclcpp_action/rclcpp_action.hpp>
#include "action_sum_cpp/action/sum.hpp"
#include <thread>

using namespace std::placeholders;
using Sum = action_sum_cpp::action::Sum;
using GoalHandleSum = rclcpp_action::ServerGoalHandle<Sum>;

class SumActionServer : public rclcpp::Node
{
public:
  explicit SumActionServer(const rclcpp::NodeOptions & options = rclcpp::NodeOptions())
  : Node("sum_action_server", options)
  {
    action_server_ = rclcpp_action::create_server<Sum>(
      this,
      "sum_action",
      std::bind(&SumActionServer::handle_goal, this, _1, _2),
      std::bind(&SumActionServer::handle_cancel, this, _1),
      std::bind(&SumActionServer::handle_accepted, this, _1)
    );
  }

private:
  rclcpp_action::Server<Sum>::SharedPtr action_server_;


  rclcpp_action::GoalResponse handle_goal(
    const rclcpp_action::GoalUUID & uuid,
    std::shared_ptr<const Sum::Goal> goal)
  {
    RCLCPP_INFO(this->get_logger(), "收到目标，n = %d", goal->n);
    (void)uuid;
    return rclcpp_action::GoalResponse::ACCEPT_AND_EXECUTE;
  }

  rclcpp_action::CancelResponse handle_cancel(
    const std::shared_ptr<GoalHandleSum> goal_handle)
  {
    (void)goal_handle;
    RCLCPP_INFO(this->get_logger(), "收到取消请求");
    return rclcpp_action::CancelResponse::ACCEPT;
  }

 
  void handle_accepted(const std::shared_ptr<GoalHandleSum> goal_handle)
  {
   
    std::thread{std::bind(&SumActionServer::execute, this, goal_handle)}.detach();
  }

  void execute(const std::shared_ptr<GoalHandleSum> goal_handle)
  {
    RCLCPP_INFO(this->get_logger(), "开始执行累加");
    rclcpp::Rate loop_rate(1);

    auto goal = goal_handle->get_goal();
    auto feedback = std::make_shared<Sum::Feedback>();
    auto result = std::make_shared<Sum::Result>();

    int current_sum = 0;
    for(int i = 1; i <= goal->n; i++)
    {
      if(goal_handle->is_canceling())
      {
        result->total_sum = current_sum;
        goal_handle->canceled(result);
        RCLCPP_INFO(this->get_logger(), "任务被取消，当前总和：%d", current_sum);
        return;
      }

      current_sum += i;
      feedback->current_num = i;
      feedback->current_sum = current_sum;
      goal_handle->publish_feedback(feedback);

      loop_rate.sleep();
    }

    result->total_sum = current_sum;
    goal_handle->succeed(result);
    RCLCPP_INFO(this->get_logger(), "任务完成，总和：%d", current_sum);
  }
};

int main(int argc, char * argv[])
{
  rclcpp::init(argc, argv);
  auto node = std::make_shared<SumActionServer>();
  rclcpp::spin(node);
  rclcpp::shutdown();
  return 0;
}