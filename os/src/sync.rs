use core::cell::{RefCell, RefMut};

pub struct UPSafeCell<T> {
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    pub unsafe fn new(value: T) -> UPSafeCell<T> {
        Self {
            inner: RefCell::new(value),
        }
    }
    pub fn access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}
