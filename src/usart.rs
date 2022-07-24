extern crate alloc;
use crate::gpio::GPIOA;
use crate::nvic::NVIC;
use crate::rcc::RCC;
use crate::vcell::VolatileCell;
use alloc::string::String;
use core::ops::{Deref,DerefMut};

const USART2_ADDR:u32 = 0x4000_4400;

pub fn init_usart2() {
    let _usart2_irqn: u32 = 38;
    let mut gpioa = GPIOA::new();
    let mut usart2 = USART2::new();
    let mut rcc = RCC::new();
    let mut nvic = NVIC::new();

    rcc.ahbenr |= 1 << 17;
    rcc.apb1enr |= 1 << 17;

    // moder
    gpioa.moder &= !((3 << 4) | (3 << 30));
    gpioa.moder |= (2 << 4) | (2 << 30);  // alternative function mode
    
    // speed
    gpioa.ospeedr &= !((3 << 4) | 3 << 30);
    gpioa.ospeedr |= (3 << 4) | (3 << 30); // high frequency

    // apply AF7
    gpioa.afrh &= !(0xF << 28); // clear PA15
    gpioa.afrh |= 7 << 28; // PA15 -> AF7

    gpioa.afrl &= !(0xF << 8); // clear PA2
    gpioa.afrl |= 7 << 8; // PA2 -> AF7

    // enable usart2_IRQ
    nvic.iser1 |= 1 << (_usart2_irqn % 32);

    // set priority
    nvic.ipr1 &= !(0xF << (_usart2_irqn % 4));
    nvic.ipr1 |= 1 << (_usart2_irqn % 4);

    // enable usart2 interruption
    usart2.cr1 |= (1 << 5) | (1 << 3) | (1 << 2) | 1;
    usart2.cr2 &= 0;
    usart2.cr3 &= 0;
    usart2.brr <<= 8000000 / 115200;
}

pub fn usart2_print0(input: String) {
    let mut usart2 = USART2::new();
    for c in input.as_bytes().iter() {
        let mut _isr_state = usart2.isr.read() & (1 << 7);
        while _isr_state & (1 << 7) == 0 {
            _isr_state = usart2.isr.read() & (1 << 7);
        }
        usart2.tdr <<= From::from(*c);
    }
}

pub fn usart2_print(input: &str) {
    usart2_print0(String::from(input));
}


#[no_mangle]
pub extern "C" fn USART2_IRQ() {
    let mut usart2 = USART2::new();
    let isr_reg = usart2.isr.read();
    if isr_reg & (1 << 5) != 0 {
        let data: u8 = TryFrom::try_from(usart2.rdr.read()).unwrap();
        let mut _isr_state = usart2.isr.read() & (1 << 7);
        while _isr_state & (1 << 7) == 0 {
            _isr_state = usart2.isr.read() & (1 << 7);
        }
        usart2.tdr <<= From::from(data);
    }
}

#[repr(C)]
pub struct USARTRegs {
    pub cr1: VolatileCell<u32>,
    pub cr2: VolatileCell<u32>,
    pub cr3: VolatileCell<u32>,
    pub brr: VolatileCell<u32>,
    pub gtpr: VolatileCell<u32>,
    pub rtor: VolatileCell<u32>,
    pub rqr: VolatileCell<u32>,
    pub isr: VolatileCell<u32>,
    pub icr: VolatileCell<u32>,
    pub rdr: VolatileCell<u32>,
    pub tdr: VolatileCell<u32>,
}

pub struct USART2;
impl USART2 {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Deref for USART2 {
    type Target = USARTRegs;
    fn deref(&self) -> &Self::Target {
        let _ret = USART2_ADDR as *const USARTRegs;
        unsafe { &*_ret }
    }
}

impl DerefMut for USART2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let _ret = USART2_ADDR as *mut USARTRegs;
        unsafe { &mut *_ret }
    }
}

