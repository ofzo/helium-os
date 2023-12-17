use crate::loader::{self, MAX_APP_NUM};
use crate::sync::UPSafeCell;
use core::arch::global_asm;
use lazy_static::lazy_static;

global_asm!(include_str!("task.s"));

extern "C" {
    pub fn __switch(current: *mut TaskContxt, next: *const TaskContxt);
}

#[derive(Debug, Clone, Copy)]
pub struct TaskContxt {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContxt {
    pub fn new() -> TaskContxt {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
    pub fn restore(sp: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp,
            s: [0; 12],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Runing,
    Exited,
}

#[derive(Debug, Clone, Copy)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContxt,
}

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

impl TaskManager {
    /// Generally, the first task in task list is an idle task(we call it zero process later).
    /// we load apps statically, so the first task is an real app.
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Runing;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContxt;
        drop(inner);
        let mut _unused = TaskContxt::new();
        unsafe {
            __switch(&mut _unused as *mut TaskContxt, next_task_cx_ptr);
        };
        panic!("unreachable in run_first_task");
    }

    /// Change task status of current from **Running** to **Ready**.
    fn mark_current_supended(&self) {
        let mut inner = self.inner.access();
        let current = inner.current;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    /// Change task status of current from **Running** to **Exited**.
    fn mark_current_exited(&self) {
        let mut inner = self.inner.access();
        let current = inner.current;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    /// Find next task to run and return app id
    ///
    /// In this case, we only return the first **Ready** task in task list.
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.access();
        let current = inner.current;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    fn run_next_task(&self) {
        if let Some(id) = self.find_next_task() {
            let mut inner = self.inner.access();
            let current = inner.current;
            inner.tasks[id].task_status = TaskStatus::Runing;
            inner.current = id;

            let current_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContxt;
            let next_cx_ptr = &mut inner.tasks[id].task_cx as *const TaskContxt;
            drop(inner);
            unsafe {
                __switch(current_cx_ptr, next_cx_ptr);
            }
        } else {
            println!("All applications completed!");
        }
    }
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current: usize,
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = loader::get_num_app();
        let mut tasks = [TaskControlBlock {
            task_cx: TaskContxt::new(),
            task_status: TaskStatus::UnInit,
        }; MAX_APP_NUM];
        for i in 0..num_app {
            tasks[i].task_cx = TaskContxt::restore(0);
            tasks[i].task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe { UPSafeCell::new(TaskManagerInner { tasks, current: 0 }) },
        }
    };
}

pub fn suspend_and_run_next() {
    TASK_MANAGER.mark_current_supended();
    TASK_MANAGER.run_next_task();
}
pub fn exited_and_run_next() {
    TASK_MANAGER.mark_current_exited();
    TASK_MANAGER.run_next_task();
}

pub fn run_first() {
    TASK_MANAGER.run_first_task();
}
