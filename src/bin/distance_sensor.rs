#![no_std]
#![no_main]

use panic_halt as _;

use stm32h7xx_hal::{prelude::*, timer::Timer, serial::{self, Serial, Error}};

use cortex_m_rt::entry;

use core::fmt::Write;
use cortex_m::asm;
use embedded_hal::digital::v2::OutputPin;
use stm32h7xx_hal::time::{Hertz};

use stm32h7xx_hal::device::DWT;

use nucleo_h743zi::smart_timer::SmartTimer;
use nucleo_h743zi::hc_sr04::HcSr04;
use nucleo_h743zi::rusty_logger::RustyLogger;

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();
    cp.DWT.enable_cycle_counter();
    let core_clock: Hertz = 100_000_000.hz();

    // Take ownership over the RCC devices and convert them into the corresponding HAL structs
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();

    let pwrcfg = pwr.freeze();

    // Freeze the configuration of all the clocks in the system and
    // retrieve the Core Clock Distribution and Reset (CCDR) object
    let ccdr = rcc.sys_ck(core_clock).freeze(pwrcfg, &dp.SYSCFG);

    // Acquire the GPIOB peripheral
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);

    // Configure pc8 as trigger.
    let mut trigger = gpioc.pc8.into_push_pull_output();

    // Configure pc5 as echo.
    let mut echo = gpioc.pc5.into_pull_down_input();

    // Acquire the GPIOD peripheral
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);

    gpiod.pd8.into_alternate_af7();
    gpiod.pd9.into_alternate_af7();

    // configure serial
    let serial = Serial::usart3(
        dp.USART3,
        serial::config::Config::default().baudrate(115200.bps()),
        ccdr.peripheral.USART3,
        &ccdr.clocks,
    ).unwrap();

    // configure software timer
    let timer = SmartTimer::new(core_clock);

    let mut logger = RustyLogger::new(serial);

    // configure hc-sr04
    let mut hc_sr04 = HcSr04::new(trigger, echo, &timer, logger);

    loop {
        match hc_sr04.wait_distance() {
            Some(distance) => {}
            None => {}
        }

    }
}
