use crate::vcell::VolatileCell;
use core::ops::{Deref, DerefMut};

const NVIC_ADDR: u32 = 0xE000_E100;

#[repr(C)]
pub struct NVICRegs {
    pub iser0: VolatileCell<u32>,
    pub iser1: VolatileCell<u32>,
    pub iser2: VolatileCell<u32>,
    pub iser3: VolatileCell<u32>,
    pub iser4: VolatileCell<u32>,
    pub iser5: VolatileCell<u32>,
    pub iser6: VolatileCell<u32>,
    pub iser7: VolatileCell<u32>,

    pub icer0: VolatileCell<u32>,
    pub icer1: VolatileCell<u32>,
    pub icer2: VolatileCell<u32>,
    pub icer3: VolatileCell<u32>,
    pub icer4: VolatileCell<u32>,
    pub icer5: VolatileCell<u32>,
    pub icer6: VolatileCell<u32>,
    pub icer7: VolatileCell<u32>,

    pub ispr0: VolatileCell<u32>,
    pub ispr1: VolatileCell<u32>,
    pub ispr2: VolatileCell<u32>,
    pub ispr3: VolatileCell<u32>,
    pub ispr4: VolatileCell<u32>,
    pub ispr5: VolatileCell<u32>,
    pub ispr6: VolatileCell<u32>,
    pub ispr7: VolatileCell<u32>,

    pub icpr0: VolatileCell<u32>,
    pub icpr1: VolatileCell<u32>,
    pub icpr2: VolatileCell<u32>,
    pub icpr3: VolatileCell<u32>,
    pub icpr4: VolatileCell<u32>,
    pub icpr5: VolatileCell<u32>,
    pub icpr6: VolatileCell<u32>,
    pub icpr7: VolatileCell<u32>,

    pub iabr0: VolatileCell<u32>,
    pub iabr1: VolatileCell<u32>,
    pub iabr2: VolatileCell<u32>,
    pub iabr3: VolatileCell<u32>,
    pub iabr4: VolatileCell<u32>,
    pub iabr5: VolatileCell<u32>,
    pub iabr6: VolatileCell<u32>,
    pub iabr7: VolatileCell<u32>,

    pub ipr0: VolatileCell<u32>,
    pub ipr1: VolatileCell<u32>,
    pub ipr2: VolatileCell<u32>,
    pub ipr3: VolatileCell<u32>,
    pub ipr4: VolatileCell<u32>,
    pub ipr5: VolatileCell<u32>,
    pub ipr6: VolatileCell<u32>,
    pub ipr7: VolatileCell<u32>,
}

pub struct NVIC;
impl NVIC {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Deref for NVIC {
    type Target = NVICRegs;
    fn deref(&self) -> &Self::Target {
        let _ret = NVIC_ADDR as *const NVICRegs;
        unsafe { &*_ret }
    }
}

impl DerefMut for NVIC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let _ret = NVIC_ADDR as *mut NVICRegs;
        unsafe { &mut *_ret }
    }
}