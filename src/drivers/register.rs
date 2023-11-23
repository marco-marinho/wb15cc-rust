use core::cell::UnsafeCell;
use core::ptr;

#[repr(transparent)]
pub struct Register<T> {
    value: UnsafeCell<T>,
}

impl<T> Register<T> {
    pub const fn new(value: T) -> Self {
        Register { value: UnsafeCell::new(value) }
    }

    #[inline(always)]
    pub fn get(&self) -> T
        where T: Copy
    {
        unsafe { ptr::read_volatile(self.value.get()) }
    }

    #[inline(always)]
    pub fn set(&self, value: T)
        where T: Copy
    {
        unsafe { ptr::write_volatile(self.value.get(), value) }
    }

    #[inline(always)]
    pub fn ptr(&self) -> *mut T {
        self.value.get()
    }
}
