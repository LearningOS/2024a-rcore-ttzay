//! Process management syscalls
use crate::{
    config::{MAX_SYSCALL_NUM, MEMORY_END, PAGE_SIZE}, mm::{translated_byte_buffer, MapPermission, VPNRange, VirtAddr}, task::{
        change_program_brk, create_new_maparea, current_user_token, exit_current_and_run_next, get_run_time, get_task_syscall_times, get_vpn_pte, suspend_current_and_run_next, unmap_maparea, TaskStatus
    }, timer::get_time_us
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
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let time_val = TimeVal{
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let dst_vec = translated_byte_buffer(
        current_user_token(), 
        ts as * const u8, 
        core::mem::size_of::<TimeVal>()
    );
    let ptr = &time_val as *const TimeVal;
    for (index , dst) in dst_vec.into_iter().enumerate() {
        let unit_len = dst.len();
        unsafe {
            dst.copy_from_slice(
                core::slice::from_raw_parts(
                    ptr.add(index * unit_len) as *const u8,
                    unit_len
                )
            );
        }
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let sys_status = TaskStatus::Running;
    let syscall_times = *get_task_syscall_times();
    let time = get_run_time();
    if time == 0 { return -1; }
    let task_info = TaskInfo {
        status: sys_status,
        syscall_times,
        time,
    };
    let dst_vec = translated_byte_buffer(
        current_user_token(), 
        ti as *const u8, 
        core::mem::size_of::<TaskInfo>()
    );
    let ptr = &task_info as *const TaskInfo;
    for( index, dst ) in dst_vec.into_iter().enumerate() {
        let unit_len = dst.len();
        unsafe {
            dst.copy_from_slice(
                core::slice::from_raw_parts(
                    ptr.add(index * unit_len) as *const u8,
                    unit_len
                )
            );
        }
    }

    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap has IMPLEMENTED");
    println!("mmap: start: {:#x}, len: {:#x}, port: {:#x}", start, len, port);
    if start % PAGE_SIZE != 0 || port & !0x7 != 0 || port & 0x7 == 0 || start >= MEMORY_END {
        return -1;
    }
    let va_start = VirtAddr::from(start).floor();
    let va_end = VirtAddr::from(start + len).ceil();
    println!("mmap: start: {:#x}, len: {:#x}, port: {:#x}", start, len, port);
    println!("va_start: {:#x}, va_end: {:#x}", va_start.0, va_end.0);
    let vpnrange = VPNRange::new(va_start, va_end);
    for vpn in vpnrange {
        if let Some(pte) = get_vpn_pte(vpn) {
            if pte.is_valid() {
                return -1;
            }
        }
    }
    create_new_maparea(
        va_start.into(),
        va_end.into(),
        MapPermission::from_bits_truncate((port<<1)as u8)
    );
    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if start % PAGE_SIZE != 0 || start >= MEMORY_END {
        return -1;
    }
    unmap_maparea(start,len)
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
