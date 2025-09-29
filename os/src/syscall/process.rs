//! Process management syscalls

use crate::mm::translated_byte_buffer;
use crate::mm::{copy_from_user, copy_to_user};
use crate::task::current_user_token;
use crate::task::TASK_MANAGER;
use crate::task::{
    change_program_brk, do_mmap, do_munmap, exit_current_and_run_next, suspend_current_and_run_next,
};
use crate::timer::get_time_us;
use core::mem;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");

    let us = get_time_us();
    let mut buffers = translated_byte_buffer(
        current_user_token(),
        _ts as *const u8,
        core::mem::size_of::<TimeVal>(),
    );

    let time_val = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    let mut src = unsafe {
        core::slice::from_raw_parts(
            &time_val as *const TimeVal as *const u8,
            mem::size_of::<TimeVal>(),
        )
    };

    for buf in &mut buffers {
        let to_copy = buf.len().min(src.len());
        if to_copy == 0 {
            break;
        }
        buf[..to_copy].copy_from_slice(&src[..to_copy]);
        src = &src[to_copy..];
    }

    0
}

/// 这个系统调用有三种功能，根据 trace_request 的值不同，执行不同的操作：
/// 如果 trace_request 为 0，则 id 应被视作 *const u8 ，表示读取当前任务 id 地址处一个字节的无符号整数值。此时应忽略 data 参数。返回值为 id 地址处的值。
/// 如果 trace_request 为 1，则 id 应被视作 *mut u8 ，表示写入 data （作为 u8，即只考虑最低位的一个字节）到该用户程序 id 地址处。返回值应为0。
/// 如果 trace_request 为 2，表示查询当前任务调用编号为 id 的系统调用的次数，返回值为这个调用次数。本次调用也计入统计。否则，忽略其他参数，返回值为 -1。
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            let mut var: u8 = 0;
            let ret = copy_from_user(
                current_user_token(),
                &mut var as *mut u8,
                id as *const u8,
                1,
            );
            if ret == -1 {
                return -1;
            }
            var as isize
        }
        1 => {
            let val = data as u8;
            let ret = copy_to_user(current_user_token(), id as *mut u8, &val as *const u8, 1);
            if ret == -1 {
                return -1;
            }
            0
        }
        2 => TASK_MANAGER.get_syscall_count(id) as isize,
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    do_mmap(_start, _len, _port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    do_munmap(start, len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
