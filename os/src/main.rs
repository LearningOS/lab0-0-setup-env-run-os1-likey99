#![no_std]
#![no_main]


#[macro_use]
mod console;
mod lang_items;
mod sbi;

use crate::sbi::shutdown;

core::arch::global_asm!(include_str!("entry.asm"));

const SYSCALL_EXIT: usize = 93;
const SYSCALL_WRITE:usize = 64;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",                           //inst ecall
            inlateout("x10") args[0] => ret,   //9 in x10;then out x10
            in("x11") args[1],  //0 in x11
            in("x12") args[2],  //1 in x12
            in("x17") id,       //93 in x17
        );
    }
    ret
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0]) //id:93  args:[9,0,0]
}

pub fn sys_write(fd:usize, buffer: &[u8])->isize{
    syscall(SYSCALL_WRITE, [fd,buffer.as_ptr() as usize,buffer.len()]) //id:64  args:[1,x,x]
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| 
        unsafe { (a as *mut u8).write_volatile(0) });
}


#[no_mangle]
extern "C" fn rust_main() {
    //loop{};
    //print!("hello,");
    //println!("world!");
    //sys_exit(9);
    clear_bss();
    shutdown();
}
/*
fn main() {
    //println!("Hello, world!");
}
*/