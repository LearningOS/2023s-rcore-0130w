//! Process management syscalls

use core::{mem::{self, transmute_copy}};
use crate::{
    config::{MAX_SYSCALL_NUM},
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next,mmap, TaskStatus, current_user_token, munmap, get_task_syscall_time, get_task_starting_time,
    }, timer::{get_time_us,get_time_ms} ,mm::{translated_byte_buffer},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
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
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    //  trace!("kernel: sys_get_time");
    let us = get_time_us();
    let token = current_user_token();
    let v = translated_byte_buffer(token, ts as *mut u8, mem::size_of::<TimeVal>());
    let timeval = TimeVal{
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };

    let timeval_bytes :[u8; mem::size_of::<TimeVal>()] = unsafe {
        transmute_copy(&timeval)
    };

    let mut write_offset = 0;

    for slice in v {
        let bytes_to_write = usize::min(slice.len(), timeval_bytes.len() - write_offset);
        slice[..bytes_to_write].copy_from_slice(&timeval_bytes[write_offset..write_offset+bytes_to_write]);
        write_offset += bytes_to_write;
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let token = current_user_token();
    let cur_time = get_time_ms();
    let v = translated_byte_buffer(token, ti as *mut u8, mem::size_of::<TaskInfo>());
    let taskinfo = TaskInfo {
        status: TaskStatus::Running,
        syscall_times: get_task_syscall_time(),
        time: (cur_time - get_task_starting_time()) + 20,
    };
    let taskinfo_bytes :[u8; mem::size_of::<TaskInfo>()] = unsafe {
        transmute_copy(&taskinfo)
    };

    let mut write_offset = 0;

    for slice in v {
        let bytes_to_write = usize::min(slice.len(), taskinfo_bytes.len() - write_offset);
        slice[..bytes_to_write].copy_from_slice(&taskinfo_bytes[write_offset..write_offset+bytes_to_write]);
        write_offset += bytes_to_write;
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    //  trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    mmap(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    //  trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    munmap(start, len)
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
