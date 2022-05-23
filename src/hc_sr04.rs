extern crate embedded_hal as hal;
extern crate nb;

use embedded_hal::digital::v2::{OutputPin, InputPin};
use crate::smart_timer::SmartTimer;
use cortex_m::asm;
use crate::rusty_logger::RustyLogger;

/// Wrapper for return value of sensor
/// Distance expects a value in metres
#[derive(Copy, Clone)]
pub struct Distance(f32);

impl Distance {
    /// Get distance as centimeters.
    pub fn cm(&self) -> f32 { self.0 * 0.1 }

    /// Get distance as millimeters.
    pub fn mm(&self) -> f32 {
        self.0
    }
}

/// Sensor State
enum State {
    /// Ready to start new measurement
    Idle,
    /// Sensor has been triggered, waiting for the returning pulse
    /// this state keeps the time when the pulse has left the sensor
    Waiting(f32),
    /// Measurement is ready
    /// this state keeps the measured distance
    Ready(Distance),
}

/// HC-SR04 device
pub struct HcSr04<'a, I, O> {
    /// Output pin to trigger the sensor
    trigger: O,
    /// Input pin to wait for the returning pulse
    echo: I,
    /// Internal mode of the sensor
    mode: State,
    /// Timer to measure the elapsed time between starting and incoming pulse
    timer: &'a SmartTimer,
    logger: RustyLogger,
}

impl<'a, I, O> HcSr04<'a, I, O>
    where
        I: InputPin,
        O: OutputPin,
{
    pub fn new(trigger: O, echo: I, timer: &'a SmartTimer, logger: RustyLogger) -> HcSr04<'a, I, O> {
        let mut trigger = trigger;
        trigger.set_low();
        HcSr04 {
            trigger,
            echo,
            mode: State::Idle,
            timer,
            logger,
        }
    }

    /// wait for a valid measured distance. The function returns the distance only if the current
    /// state is State::Ready, otherwise it returns None
    // TODO handle sensor timeout. The hc-sr04 will timeout only if the pulse takes more than 36ms
    // TODO to return. After timeout the echo pin is set to low
    pub fn wait_distance(&mut self) -> Option<Distance> {
        // self.logger.write("Timestamp: ");
        // self.logger.write(self.timer.now_ms());
        // self.logger.write('\n');
        // this method is called inside the loop, so the counter counts the number of cycles
        match self.mode {
            // Start a new sensor measurement
            State::Idle => {
                self.trigger();
                let start = self.timer.now_ms();
                // self.logger.write("Pulse started: ");
                // self.logger.write(start);
                // self.logger.write('\n');
                // update mode to waiting
                self.mode = State::Waiting(start);
                None
            }
            State::Waiting(start) => {
                // as soon as the signal starts, the echo pin is set to high
                // when the signal returns to the sensor, the echo pin is set to low
                match self.echo.is_low() {
                    Ok(is_low) => {
                        // self.logger.write("Echo status: ");
                        // self.logger.write(is_low);
                        // self.logger.write('\n');
                        if is_low {
                            let now = self.timer.now_ms();
                            // self.logger.write("Pulse received: ");
                            // self.logger.write(now);
                            // self.logger.write('\n');
                            // unit: second
                            let elapsed_time = now - start;
                            // self.logger.write("Elapsed time: ");
                            // self.logger.write(elapsed_time);
                            // self.logger.write(" ms \n");
                            // speed of sound through the air is 340.29 m/s
                            // we need to divide it by 2 due to the roundtrip of the pulse.
                            // Do this pre-calculation for better performances
                            // SPEED_OF_SOUND/2.0 = 340.29 / 2.0 = 170.145 m/s
                            const HALF_SPEED_OF_SOUND: f32 = 170.145;
                            let distance = (elapsed_time / 1_000 as f32) * HALF_SPEED_OF_SOUND;
                            // sensor is ready to deliver the measured distance
                            self.mode = State::Ready(Distance(distance));
                        }
                        None
                    }
                    Err(_) => None
                }
            }
            State::Ready(dist) => {
                // sensor has delivered the distance and now it's ready to read another distance
                if dist.0 > 0.5 {
                    self.logger.write("Distance: ");
                    self.logger.write(dist.mm());
                    self.logger.write(" m \n");
                    self.mode = State::Idle;
                }

                Some(dist)
            }
        }
    }


    /// Trigger sensor starting a measurement
    fn trigger(&mut self) {
        // 0.00001 s = 10 us
        let delay = 100000;
        self.trigger.set_high();
        let start = self.timer.now_us();
        asm::delay(self.timer.clock.0 / delay);
        let stop = self.timer.now_us();
        // self.logger.write("Pulse duration: ");
        // self.logger.write((stop - start));
        // self.logger.write(" us \n");
        self.trigger.set_low();
    }
}