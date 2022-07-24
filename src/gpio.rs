use crate::vcell::VolatileCell;
use core::ops::{Deref, DerefMut};

const GPIOA_ADDR:u32 = 0x4800_0000;
const GPIOB_ADDR:u32 = 0x4800_0400;

#[repr(C)]
pub struct GPIORegs {
    pub moder: VolatileCell<u32>,
    pub otyper: VolatileCell<u32>,
    pub ospeedr: VolatileCell<u32>,
    pub pupdr: VolatileCell<u32>,
    pub idr: VolatileCell<u32>,
    pub odr: VolatileCell<u32>,
    pub bsrr: VolatileCell<u32>,
    pub lckr: VolatileCell<u32>,
    pub afrl: VolatileCell<u32>,
    pub afrh: VolatileCell<u32>,
    pub brr: VolatileCell<u32>,
}

pub struct GPIOA;
impl GPIOA {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Deref for GPIOA {
    type Target = GPIORegs;
    fn deref(&self) -> &Self::Target {
        let _ret = GPIOA_ADDR as *const GPIORegs;
        unsafe { &*_ret }
    }
}

impl DerefMut for GPIOA {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let _ret = GPIOA_ADDR as *mut GPIORegs;
        unsafe { &mut *_ret }
        
    }
}

pub struct GPIOB;
impl GPIOB {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Deref for GPIOB {
    type Target = GPIORegs;
    fn deref(&self) -> &Self::Target {
        let _ret = GPIOB_ADDR as *const GPIORegs;
        unsafe { &*_ret }
    }
}

impl DerefMut for GPIOB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let _ret = GPIOB_ADDR as *mut GPIORegs;
        unsafe { &mut *_ret }
        
    }
}