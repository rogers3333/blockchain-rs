use actix::prelude::{Actor, Context, Handler, Message};
use tokio::time::{sleep, Duration, interval};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

/// 定义 TimerManager 结构体，它是一个 Actor
/// The TimerManager struct, which is an Actor
pub struct TimerManager;

/// 为 TimerManager 实现 Actor trait
/// Implement the Actor trait for TimerManager
impl Actor for TimerManager {
    type Context = Context<Self>;
}

/// 定义 Schedule 消息，用于在指定延迟后执行任务
/// Define the Schedule message for executing a task after a specified delay
#[derive(Message)]
#[rtype(result = "()")]
pub struct Schedule {
    pub action: Box<dyn FnOnce() + Send + 'static>,
    pub delay: u64,
}

/// 定义 ScheduleAtFixedRate 消息，用于以固定速率执行任务
/// Define the ScheduleAtFixedRate message for executing a task at a fixed rate
#[derive(Message)]
#[rtype(result = "()")]
pub struct ScheduleAtFixedRate {
    pub action: Box<dyn Fn() + Send + 'static>,
    pub initial_delay: u64,
    pub period: u64,
}

/// 定义 ScheduleWithFixedDelay 消息，用于以固定延迟执行任务
/// Define the ScheduleWithFixedDelay message for executing a task with a fixed delay
#[derive(Message)]
#[rtype(result = "()")]
pub struct ScheduleWithFixedDelay {
    pub action: Box<dyn Fn() + Send + 'static>,
    pub initial_delay: u64,
    pub period: u64,
}

/// 为 TimerManager 实现处理 Schedule 消息的功能
/// Implement the handler for the Schedule message in TimerManager
impl Handler<Schedule> for TimerManager {
    type Result = ();

    fn handle(&mut self, msg: Schedule, _ctx: &mut Self::Context) {
        let action = msg.action;
        let delay = msg.delay;
        actix::spawn(async move {
            sleep(Duration::from_millis(delay)).await;
            action();
        });
    }
}

/// 为 TimerManager 实现处理 ScheduleAtFixedRate 消息的功能
/// Implement the handler for the ScheduleAtFixedRate message in TimerManager
impl Handler<ScheduleAtFixedRate> for TimerManager {
    type Result = ();

    fn handle(&mut self, msg: ScheduleAtFixedRate, _ctx: &mut Self::Context) {
        let action = msg.action;
        let initial_delay = msg.initial_delay;
        let period = msg.period;
        actix::spawn(async move {
            sleep(Duration::from_millis(initial_delay)).await;
            let mut interval = interval(Duration::from_millis(period));
            loop {
                interval.tick().await;
                action();
            }
        });
    }
}

/// 为 TimerManager 实现处理 ScheduleWithFixedDelay 消息的功能
/// Implement the handler for the ScheduleWithFixedDelay message in TimerManager
impl Handler<ScheduleWithFixedDelay> for TimerManager {
    type Result = ();

    fn handle(&mut self, msg: ScheduleWithFixedDelay, _ctx: &mut Self::Context) {
        let action = msg.action;
        let initial_delay = msg.initial_delay;
        let period = msg.period;
        actix::spawn(async move {
            sleep(Duration::from_millis(initial_delay)).await;
            loop {
                action();
                sleep(Duration::from_millis(period)).await;
            }
        });
    }
}