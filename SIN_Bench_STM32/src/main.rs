//! Sin routine bench test
//!
//! This code will test the speed of Sin and Cosine calculation routines
//! - floating point method using libm
//! - fixed point using fixed_trigonometry
//! - fixed point using pre-calculated table
//! The code will interact with usart for setting the calculation method
//! and at each step increment the angle and update two PWM outputs with
//! the sine and cosine of the angle. The frequecy of the output will be
//! proportional to the troughput. Moreover the THD of the filtered wave
//! will correlate with the accuracy of the calulation.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial},
    timer::{Channel, Tim3NoRemap},
};
use libm::{sinf, cosf};
// use fixed_trigonometry::{sin,cos};
use core::fmt::Write;
use fixed::types::I2F14;
use fixed::types::I4F12;
mod sin_table;
mod table_trig;

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(64.MHz()).pclk1(32.MHz()).adcclk(2.MHz()).freeze(&mut flash.acr);

    // Prepare the alternate function I/O registers
    let mut afio = dp.AFIO.constrain();

    // Acquire the GPIOB peripheral
    let mut gpiob = dp.GPIOB.split();

    // Acquire the GPIOA peripheral
    let mut gpioa = dp.GPIOA.split();

    // Acquire the GPIOC peripheral
//    let mut gpioc = dp.GPIOC.split();

    // USART1
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    // Set up the usart device. Take ownership over the USART register and tx/rx pins. The rest of
    // the registers are used to enable and configure the device.
    let serial = Serial::new(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(9600.bps()),
        &clocks,
    );

    // Split the serial struct into a receiving and a transmitting part
    let (mut tx, mut rx) = serial.split();

    // TIM3 channel 3 and 4
    let c3 = gpiob.pb0.into_alternate_push_pull(&mut gpiob.crl);
    let c4 = gpiob.pb1.into_alternate_push_pull(&mut gpiob.crl);
    
    let pins = (c3, c4);
    let mut pwm = dp
        .TIM3   
        .pwm_hz::<Tim3NoRemap, _, _>(pins, &mut afio.mapr, 8.kHz(), &clocks);
    pwm.enable(Channel::C3);
    pwm.enable(Channel::C4);
    let max = pwm.get_max_duty();

    // Configure gpio B pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    let mut led1 = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);

    let mut angle: I4F12;
    let resolution = 12-7; // 2^-7
    let mut sinval: I2F14;
    let startangle: i32 = -3140;
    let endangle: i32 = 3140;
    let mut received;
    pwm.set_duty(Channel::C3, max/4);

    loop {
        writeln!(tx, "Enter command 't' or 'l' ");
        received=block!(rx.read()).unwrap(); // receive commands from uart
        writeln!(tx, "Received {}, max duty {}",received, max);
        match received {
            b't' => {
                writeln!(tx, "Start t");
                //led.set_high();
                for j in 1..1000 {
                    //writeln!(tx, "{}",j);
                    angle = I4F12::from_num(-3.14);
                    //led.set_high();
                    led.set_high();
                    for i in startangle..endangle {
                        //led.set_high();
                        //led1.set_high();
                        sinval = table_trig::sin_t(angle,&sin_table::SIN_TABLE, resolution); // about 400ns @ 64MHz
                        //led1.set_low();
                        pwm.set_duty(Channel::C4, ((sinval.to_bits() >> 3)+32767/16) as u16);
                        //led1.set_high();
                        //writeln!(tx, "angle:{} sin:{} pwm:{}",angle, sinval,pwm.get_duty(Channel::C4));
                        angle += I4F12::from_num(0.00101);
                        //led1.set_low();
                        //led.set_low();    
                    }
                    led.set_low();
                }
                writeln!(tx, "end t");
                //led.set_low();
            }
            b'l' => {
                // to use libm
                writeln!(tx, "Start l");
                //led.set_high();
                for j in 1..1000 {
                    //writeln!(tx, "{}",j);
                    angle = I4F12::from_num(-3.14);
                    led.toggle();
                    led1.set_high();
                    for i in startangle..endangle {
                        //led.toggle();
                        sinval = sin_(angle);
                        pwm.set_duty(Channel::C4, ((sinval.to_bits() >> 3)+32767/16) as u16);
                        //writeln!(tx, "angle:{} sin:{} pwm:{}",angle, sinval,pwm.get_duty(Channel::C4));
                        angle += I4F12::from_num(0.0101);
                    }
                    led1.set_low();
                }
                writeln!(tx, "end t");
                //led.set_low();
            }
            _ => {
                writeln!(tx, "Valid commands: 't' or 'l'");
            }
        }
        writeln!(tx, "End Loop");
    }
}

fn sin_(angle:I4F12) -> I2F14{

    let angle_f: f32;
    angle_f=angle.to_num();
    I2F14::from_num(sinf(angle_f))
    
}
