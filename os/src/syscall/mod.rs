mod fs;
mod process;

use self::{fs::sys_write, process::sys_exit};

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    // println!("syscall {}", id);
    match id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        _ => panic!("Unsupport syscall id = {:?}", id),
    }
}
