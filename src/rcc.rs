use crate::vcell::VolatileCell;
use core::ops::{Deref, DerefMut};

const RCC_ADDR:u32 = 0x4002_1000;

#[repr(C)]
pub struct RCCRegs {
    pub cr: VolatileCell<u32>,
    pub cfgr: VolatileCell<u32>,
    pub cir: VolatileCell<u32>,
    pub apb2rstr: VolatileCell<u32>,
    pub apb1rstr: VolatileCell<u32>,
    pub ahbenr: VolatileCell<u32>,
    pub apb2enr: VolatileCell<u32>,
    pub apb1enr: VolatileCell<u32>,
    pub bdcr: VolatileCell<u32>,
    pub csr: VolatileCell<u32>,
    pub ahbrstr: VolatileCell<u32>,
    pub cfgr2: VolatileCell<u32>,
    pub cfgr3: VolatileCell<u32>,
}

pub struct RCC;
impl RCC {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Deref for RCC {
    type Target = RCCRegs;
    fn deref(&self) -> &Self::Target {
        let _ret = RCC_ADDR as *const RCCRegs;
        unsafe { &*_ret }
    }
}

impl DerefMut for RCC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let _ret = RCC_ADDR as *mut RCCRegs;
        unsafe { &mut *_ret }
    }
}