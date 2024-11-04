//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus},
    timer::get_time_us,
};

/// TimeVal struct for get_time syscall
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    /// second
    pub sec: usize,
    /// microsecond
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
#[derive(Debug, Copy,Clone)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize, // ms
}


impl TaskInfo {
    /// Create a new TaskInfo
    pub fn new() -> Self {
        TaskInfo {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
    /// plus times of syscall
    pub fn plus_times(&mut self, syscall_id: usize) {
        self.syscall_times[syscall_id] += 1;
    }
    /// record the first time
    pub fn record_firt_time(&mut self, time: usize) {
        self.time = time;
    }
    /// get time
    pub fn get_time(&self) -> usize {
        self.time
    }
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let task_info = crate::task::get_task_info();
    let time_interval = crate::timer::get_time_ms() - task_info.get_time();
    unsafe {
        *ti = TaskInfo {
            status: TaskStatus::Running,
            syscall_times: task_info.syscall_times,
            time: time_interval,
        };
    }
    0
}
