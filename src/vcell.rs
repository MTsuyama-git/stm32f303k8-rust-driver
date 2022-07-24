use core::cell::UnsafeCell;
use core::ptr::{read_volatile, write_volatile};
use core::ops::{BitAndAssign,BitOrAssign};

#[repr(transparent)]
pub struct VolatileCell<T> {
    inner: UnsafeCell<T>,
}

impl<T> VolatileCell<T> {
    pub fn read(&self) -> T {
        unsafe {
            return read_volatile(self.inner.get());
        }
    }

    pub fn write(&self, val: T) {
        unsafe {
            write_volatile(self.inner.get(), val);
        }
    }
}


impl<U32: core::ops::BitAndAssign> BitAndAssign<U32> for VolatileCell<U32> {
    fn bitand_assign(&mut self, rhs: U32) {
        let mut raw_value = self.read();
        raw_value &= rhs;
        self.write(raw_value);
    }
}

impl<U32: core::ops::BitOrAssign> BitOrAssign<U32> for VolatileCell<U32> {
    fn bitor_assign(&mut self, rhs: U32) {
        let mut raw_value = self.read();
        raw_value |= rhs;
        self.write(raw_value);
    }
}