use crate::gpio::GPIOB;
use crate::rcc::RCC;

pub fn init_led() {
    // let addr_gpioa:u32 = 0x4100_8000; // ATSAMD51xx
    let mut rcc = RCC::new();
    let mut gpiob = GPIOB::new();

    rcc.ahbenr |= 1 << 18; // iopben

    gpiob.moder &= !(3 << 6);
    gpiob.moder |= 1 << 6; // output

    gpiob.otyper &= !(1 << 3); // push-pull

    gpiob.ospeedr &= !(3 << 6);
    gpiob.ospeedr |= 3 << 6; // high-speed

    gpiob.pupdr &= !(3 << 6);
    gpiob.pupdr |= 2 << 6; // pull-down

    gpiob.idr &= !(1 << 3);
}

pub fn set_led() {
    let mut gpiob = GPIOB::new();
    gpiob.odr |= 1 << 3;
}

pub fn clear_led() {
    let mut gpiob = GPIOB::new();
    gpiob.odr &= !(1 << 3);
}
