use stm32h7xx_hal::rcc::{Ccdr, CoreClocks};
use stm32h7xx_hal::serial::{Rx, Serial, Tx, config::Config};
use stm32h7xx_hal::stm32::{Peripherals, USART3};
use stm32h7xx_hal::time::U32Ext;
use core::fmt::{Debug, Display, Write};
use crate::smart_timer::SmartTimer;

pub struct RustyLogger {
    pub rx: Rx<USART3>,
    pub tx: Tx<USART3>,
}

impl RustyLogger {
    //
    pub fn new(serial: Serial<USART3>) -> RustyLogger {
        let (mut tx, mut rx) = serial.split();

        RustyLogger {
            rx,
            tx,
        }
    }

    pub fn write<T: Display>(&mut self, msg: T) -> () {
        write!(self.tx, "{}", msg).unwrap()
    }
}