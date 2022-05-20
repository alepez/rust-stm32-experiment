#![no_std]
#![no_main]

use panic_halt as _;

use stm32h7xx_hal::{prelude::*, timer::Timer, serial::{self, Serial, Error}};

use cortex_m_rt::entry;
use nucleo_h743zi::hc_sr04::HcSr04;

use core::fmt::Write;
use stm32h7xx_hal::time::{Hertz};
use nucleo_h743zi::smart_timer::SmartTimer;

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let core_clock: Hertz = 100_000_000.hz();

    // Take ownership over the RCC devices and convert them into the corresponding HAL structs
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();

    let pwrcfg = pwr.freeze();

    // Freeze the configuration of all the clocks in the system and
    // retrieve the Core Clock Distribution and Reset (CCDR) object
    let rcc = rcc.use_hse(core_clock).bypass_hse();
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    // Acquire the GPIOB peripheral
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);

    // Configure pc8 as trigger.
    let mut trigger = gpioc.pc8.into_push_pull_output();

    // Configure pc5 as echo.
    let mut echo = gpioc.pc5.into_pull_down_input();

    // Acquire the GPIOD peripheral
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);

    let delay = cp.SYST.delay(ccdr.clocks);

    gpiod.pd8.into_alternate_af7();
    gpiod.pd9.into_alternate_af7();

    // configure serial
    let serial = Serial::usart3(
        dp.USART3,
        serial::config::Config::default().baudrate(115200.bps()),
        ccdr.peripheral.USART3,
        &ccdr.clocks,
    ).unwrap();

    // retrieve rx and tx
    let (mut tx, mut rx) = serial.split();

    // configure software timer
    let timer = SmartTimer::new(core_clock);

    // configure hc-sr04
    let mut hc_sr04 = HcSr04::new(trigger, echo, timer);


    loop {
        match hc_sr04.wait_distance() {
            Some(distance) => {
                writeln!(tx, "Distance: {}", distance.mm()).unwrap();
            }
            None => {}
        }
    }
}
