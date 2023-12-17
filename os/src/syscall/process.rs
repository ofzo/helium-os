use crate::task::{exited_and_run_next, suspend_and_run_next};

pub fn sys_exit(code: i32) -> ! {
    println!("[Helium]: Application exited with code {}", code);
    // run_next_app()
    exited_and_run_next();
    panic!("next")
}

pub fn sys_yield() -> isize {
    suspend_and_run_next();
    0
}
