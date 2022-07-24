#![no_main]
#![no_std]
#![feature(alloc_error_handler)]

mod gpio;
mod interrupt_vector;
mod led;
mod nvic;
mod rcc;
mod systick;
mod usart;
mod vcell;

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout;
use core::panic::PanicInfo;
use core::ptr;
use interrupt_vector::Vector;
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

static mut I: u8 = 1;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn Reset() {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
        static mut _sidata: u8;
        static mut _sdata: u8;
        static mut _edata: u8;
    }

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

    systick::init();
    led::init_led();
    usart::init_usart2();

    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024;
    static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    ALLOCATOR.init(HEAP.as_ptr() as usize, HEAP_SIZE);

    usart::usart2_print("Test\r\n");
    loop {}
}

extern "C" {
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn PendSV();
    // fn SysTick();
    fn WWDG();
    fn PVD();
    fn TamperStamp();
    fn RTCWKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2_TS();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_C1();
    fn DMA1_C2();
    fn DMA1_C3();
    fn DMA1_C4();
    fn DMA1_C5();
    fn DMA1_C6();
    fn DMA1_C7();
    fn ADC1_2();
    fn CAN_TX();
    fn CAN_RX0();
    fn CAN_RX1();
    fn CAN_SCE();
    fn EXTI9_5();
    fn TIM1_BRK();
    fn TIM1_UP();
    fn TIM1_TRG_COM();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn I2C1_EV();
    fn I2C1_ER();
    fn SPI1();
    fn USART1();
    // fn USART2();
    fn USART3();
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 55] = [
    Vector { handler: Reset },
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector {
        handler: UsageFault,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick }, // 0x3C
    Vector { handler: WWDG },    // 0x40
    Vector { handler: PVD },     // 0x44
    Vector {
        handler: TamperStamp,
    }, //0x48
    Vector { handler: RTCWKUP }, //0x4C
    Vector { handler: FLASH },   //0x50
    Vector { handler: RCC },     //054
    Vector { handler: EXTI0 },   //058
    Vector { handler: EXTI1 },   //05C
    Vector { handler: EXTI2_TS }, //060
    Vector { handler: EXTI3 },   //064
    Vector { handler: EXTI4 },   //068
    Vector { handler: DMA1_C1 }, //06C
    Vector { handler: DMA1_C2 }, //070
    Vector { handler: DMA1_C3 }, //074
    Vector { handler: DMA1_C4 }, //078
    Vector { handler: DMA1_C5 }, //07C
    Vector { handler: DMA1_C6 }, //080
    Vector { handler: DMA1_C7 }, //084
    Vector { handler: ADC1_2 },  //088
    Vector { handler: CAN_TX },  //08C
    Vector { handler: CAN_RX0 }, //090
    Vector { handler: CAN_RX1 }, //094
    Vector { handler: CAN_SCE }, //098
    Vector { handler: EXTI9_5 }, //09C
    Vector { handler: TIM1_BRK }, //0A0
    Vector { handler: TIM1_UP }, //0A4
    Vector {
        handler: TIM1_TRG_COM,
    }, //0A8
    Vector { handler: TIM1_CC }, //0AC
    Vector { handler: TIM2 },    //0B0
    Vector { handler: TIM3 },    //0B4
    Vector { reserved: 0 },      //0B8
    Vector { handler: I2C1_EV }, //0BC
    Vector { handler: I2C1_ER }, //0C0
    Vector { reserved: 0 },      //0C4
    Vector { reserved: 0 },      //0C8
    Vector { handler: SPI1 },    //0CC
    Vector { reserved: 0 },      //0D0
    Vector { handler: USART1 },  //0D4
    Vector {
        handler: usart::USART2_IRQ,
    }, //0D8
    Vector { handler: USART3 },  //0DC
];

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop {}
}

#[no_mangle]
pub extern "C" fn SysTick() {
    unsafe {
        if I == 0 {
            led::clear_led();
        } else {
            led::set_led();
        }
        I = 1 - I;
    }
}

#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}
