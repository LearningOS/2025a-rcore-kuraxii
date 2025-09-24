//! Types related to task management

use super::TaskContext;
use crate::syscall::{ TOTAL_SYSTEMCALL};

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// 系统调用调用计数  使用 syscall_id_to_index 将系统调用转化为index 减少空间浪费
    pub syscall_count : [u32; TOTAL_SYSTEMCALL],
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
