use core::ops::{Deref, DerefMut};
use crate::vcell::VolatileCell;

const STK_ADDR: u32 = 0xE000_E010;

#[repr(C)]
pub struct SysTickRegs {
    pub ctrl: VolatileCell<u32>,
    pub load: VolatileCell<u32>,
    pub val: VolatileCell<u32>,
    pub calib: VolatileCell<u32>,
}

pub struct STK;
impl STK {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Deref for STK {
    type Target = SysTickRegs;
    fn deref(&self) -> &Self::Target {
        let _ret = STK_ADDR as *const SysTickRegs;
        unsafe { &*_ret }
    }
}

impl DerefMut for STK {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let _ret = STK_ADDR as *mut SysTickRegs;
        unsafe { &mut *_ret }
    }
}

pub fn init() {
    let mut stk = STK::new();
    stk.val &= 0;
    stk.load <<= 1 << 20;
    stk.ctrl |= 0x03; 
}

