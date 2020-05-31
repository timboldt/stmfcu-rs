//  Copyright 2020 Google LLC
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::entry;
//use cortex_m_semihosting::hprintln;
use hal::block;
use hal::prelude::*;
use hal::serial::{config::Config, Serial};
use hal::stm32;
use stm32f4xx_hal as hal;

#[entry]
fn main() -> ! {
    //hprintln!("Start of main()").unwrap();
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        let gpioa = dp.GPIOA.split();
        let gpiob = dp.GPIOB.split();

        // Set up the LEDs.
        let mut led1 = gpiob.pb5.into_push_pull_output();
        let mut led2 = gpiob.pb4.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();

        // USART 1 at PA9 (TX) and PA10(RX)
        let tx = gpioa.pa9.into_alternate_af7();
        let rx = gpioa.pa10.into_alternate_af7();
        let serial = Serial::usart1(
            dp.USART1,
            (tx, rx),
            Config::default().baudrate(115_200.bps()),
            clocks,
        )
        .unwrap();
        let (mut tx, mut rx) = serial.split();

        // Create a delay abstraction based on SysTick
        let mut _delay = hal::delay::Delay::new(cp.SYST, clocks);

        loop {
            // Read character and echo it back
            let received = block!(rx.read()).unwrap();
            block!(tx.write(received)).ok();
            if received % 2 == 1 {
                led1.set_high().unwrap();
                led2.set_low().unwrap();
                //delay.delay_ms(1000_u32);
            } else {
                led1.set_low().unwrap();
                led2.set_high().unwrap();
                //delay.delay_ms(1000_u32);
            }
        }
    }
    panic!("Failed to start!");
}
