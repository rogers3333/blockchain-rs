use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use super::*;
use actix::{Actor, System};
use tokio::time::{sleep, timeout, Duration};
use blockchain_rs::common::timer::timer_manager::{Schedule, ScheduleAtFixedRate, ScheduleWithFixedDelay, TimerManager};

/// 测试 Schedule 消息，验证任务是否在指定延迟后执行。
/// Test the Schedule message to verify if the task is executed after the specified delay.
#[actix_rt::test]
async fn test_schedule() {
    let addr = TimerManager.start();
    let counter = Arc::new(AtomicU32::new(0));
    let counter_clone = counter.clone();

    // 发送 Schedule 消息，100 毫秒后执行任务
    addr.send(Schedule {
        action: Box::new(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        }),
        delay: 100,
    }).await.expect("Failed to send Schedule message");

    // 等待一段时间，确保任务有机会执行
    let result = timeout(Duration::from_millis(200), async {
        loop {
            if counter.load(Ordering::SeqCst) == 1 {
                break;
            }
            sleep(Duration::from_millis(10)).await;
        }
    }).await;

    // 断言任务成功执行
    assert!(result.is_ok(), "Task did not execute within the expected time");

}

/// 测试 ScheduleAtFixedRate 消息，验证任务是否以固定速率周期性执行。
/// Test the ScheduleAtFixedRate message to verify if the task is executed periodically at a fixed rate.
#[actix_rt::test]
async fn test_schedule_at_fixed_rate() {
    let addr = TimerManager.start();
    let counter = Arc::new(AtomicU32::new(0));
    let counter_clone = counter.clone();

    // 发送 ScheduleAtFixedRate 消息，初始延迟 100 毫秒，周期 100 毫秒
    addr.send(ScheduleAtFixedRate {
        action: Box::new(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        }),
        initial_delay: 100,
        period: 100,
    }).await.expect("Failed to send ScheduleAtFixedRate message");

    // 等待一段时间，让任务有多次执行机会
    sleep(Duration::from_millis(500)).await;

    // 断言计数器的值大于等于 3，表明任务至少执行了 3 次
    assert!(counter.load(Ordering::SeqCst) >= 3, "Task did not execute periodically as expected");
}

/// 测试 ScheduleWithFixedDelay 消息，验证任务是否以固定延迟周期性执行。
/// Test the ScheduleWithFixedDelay message to verify if the task is executed periodically with a fixed delay.
#[actix_rt::test]
async fn test_schedule_with_fixed_delay() {
    let addr = TimerManager.start();
    let counter = Arc::new(AtomicU32::new(0));
    let counter_clone = counter.clone();

    // 发送 ScheduleWithFixedDelay 消息，初始延迟 100 毫秒，周期 100 毫秒
    addr.send(ScheduleWithFixedDelay {
        action: Box::new(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        }),
        initial_delay: 100,
        period: 100,
    }).await.expect("Failed to send ScheduleWithFixedDelay message");

    // 等待一段时间，让任务有多次执行机会
    sleep(Duration::from_millis(500)).await;

    // 断言计数器的值大于等于 3，表明任务至少执行了 3 次
    assert!(counter.load(Ordering::SeqCst) >= 3, "Task did not execute periodically as expected");

}