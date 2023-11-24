//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use stm32f1xx_hal::{adc, pac, prelude::*, timer::Timer,serial::{Config, Serial}};
// use libm::{sinf, cosf};
// use fixed_trigonometry::{sin,cos};
use fixed::types::I4F12;
use fixed::types::I2F14;
use core::fmt::Write;
mod table_trig;
mod sin_table;

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
    let clocks = rcc.cfgr.adcclk(2.MHz()).freeze(&mut flash.acr);

    // Prepare the alternate function I/O registers
    let mut afio = dp.AFIO.constrain();

    // Acquire the GPIOC peripheral
    let mut gpiob = dp.GPIOB.split();

    // Acquire the GPIOA peripheral
    let mut gpioa = dp.GPIOA.split();

    // Setup ADC
    let mut adc1 = adc::Adc::adc1(dp.ADC1, clocks);

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
    let (mut tx, _rx) = serial.split();

    // Configure pa1 as an analog input
    let mut ch0 = gpioa.pa1.into_analog(&mut gpioa.crl);

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(10.Hz()).unwrap();

    // Wait for the timer to trigger an update and change the state of the LED
    //let mut angle:f32=-3.14;
    let mut angle:I4F12;
    //let mut sinval:f32=0.0;
//    let sin_table=[I2F14::from_num(0.0),I2F14::from_num(0.0078),I2F14::from_num(0.0156),I2F14::from_num(0.02344),I2F14::from_num(0.03125),I2F14::from_num(0.03906),I2F14::from_num(0.0469),I2F14::from_num(0.0547),I2F14::from_num(0.06244),I2F14::from_num(0.07025),I2F14::from_num(0.07806),I2F14::from_num(0.0858),I2F14::from_num(0.0936),I2F14::from_num(0.1014),I2F14::from_num(0.10913),I2F14::from_num(0.11694),I2F14::from_num(0.1247),I2F14::from_num(0.13245),I2F14::from_num(0.14014),I2F14::from_num(0.1479),I2F14::from_num(0.15564),I2F14::from_num(0.1633),I2F14::from_num(0.171),I2F14::from_num(0.1787),I2F14::from_num(0.1864),I2F14::from_num(0.1941),I2F14::from_num(0.2017),I2F14::from_num(0.20935),I2F14::from_num(0.217),I2F14::from_num(0.2246),I2F14::from_num(0.23224),I2F14::from_num(0.2398),I2F14::from_num(0.2474),I2F14::from_num(0.25494),I2F14::from_num(0.2625),I2F14::from_num(0.27),I2F14::from_num(0.2775),I2F14::from_num(0.28503),I2F14::from_num(0.29254),I2F14::from_num(0.3),I2F14::from_num(0.30743),I2F14::from_num(0.3149),I2F14::from_num(0.32227),I2F14::from_num(0.32965),I2F14::from_num(0.33704),I2F14::from_num(0.34436),I2F14::from_num(0.3517),I2F14::from_num(0.359),I2F14::from_num(0.3663),I2F14::from_num(0.37354),I2F14::from_num(0.38074),I2F14::from_num(0.388),I2F14::from_num(0.39514),I2F14::from_num(0.40234),I2F14::from_num(0.4095),I2F14::from_num(0.41656),I2F14::from_num(0.4237),I2F14::from_num(0.4307),I2F14::from_num(0.4378),I2F14::from_num(0.44476),I2F14::from_num(0.4518),I2F14::from_num(0.45874),I2F14::from_num(0.46564),I2F14::from_num(0.47253),I2F14::from_num(0.47943),I2F14::from_num(0.48627),I2F14::from_num(0.4931),I2F14::from_num(0.4999),I2F14::from_num(0.5066),I2F14::from_num(0.5133),I2F14::from_num(0.52),I2F14::from_num(0.5267),I2F14::from_num(0.5333),I2F14::from_num(0.5399),I2F14::from_num(0.54645),I2F14::from_num(0.553),I2F14::from_num(0.55945),I2F14::from_num(0.5659),I2F14::from_num(0.5723),I2F14::from_num(0.57874),I2F14::from_num(0.5851),I2F14::from_num(0.59143),I2F14::from_num(0.5977),I2F14::from_num(0.60394),I2F14::from_num(0.61017),I2F14::from_num(0.61633),I2F14::from_num(0.62244),I2F14::from_num(0.62854),I2F14::from_num(0.6346),I2F14::from_num(0.6406),I2F14::from_num(0.6466),I2F14::from_num(0.6525),I2F14::from_num(0.65845),I2F14::from_num(0.6643),I2F14::from_num(0.6701),I2F14::from_num(0.6759),I2F14::from_num(0.68164),I2F14::from_num(0.6873),I2F14::from_num(0.693),I2F14::from_num(0.6986),I2F14::from_num(0.70416),I2F14::from_num(0.7097),I2F14::from_num(0.71515),I2F14::from_num(0.72064),I2F14::from_num(0.726),I2F14::from_num(0.7314),I2F14::from_num(0.7367),I2F14::from_num(0.74194),I2F14::from_num(0.74713),I2F14::from_num(0.7523),I2F14::from_num(0.75745),I2F14::from_num(0.7625),I2F14::from_num(0.7675),I2F14::from_num(0.7725),I2F14::from_num(0.77747),I2F14::from_num(0.78235),I2F14::from_num(0.7872),I2F14::from_num(0.792),I2F14::from_num(0.79675),I2F14::from_num(0.80145),I2F14::from_num(0.8061),I2F14::from_num(0.81067),I2F14::from_num(0.81525),I2F14::from_num(0.8197),I2F14::from_num(0.82416),I2F14::from_num(0.82855),I2F14::from_num(0.83295),I2F14::from_num(0.8372),I2F14::from_num(0.8415),I2F14::from_num(0.84564),I2F14::from_num(0.8498),I2F14::from_num(0.8539),I2F14::from_num(0.858),I2F14::from_num(0.86194),I2F14::from_num(0.86584),I2F14::from_num(0.86975),I2F14::from_num(0.8736),I2F14::from_num(0.8774),I2F14::from_num(0.88104),I2F14::from_num(0.88477),I2F14::from_num(0.88837),I2F14::from_num(0.8919),I2F14::from_num(0.89545),I2F14::from_num(0.89886),I2F14::from_num(0.9023),I2F14::from_num(0.9056),I2F14::from_num(0.9089),I2F14::from_num(0.9121),I2F14::from_num(0.9153),I2F14::from_num(0.9184),I2F14::from_num(0.9215),I2F14::from_num(0.9245),I2F14::from_num(0.9274),I2F14::from_num(0.93036),I2F14::from_num(0.93317),I2F14::from_num(0.936),I2F14::from_num(0.93866),I2F14::from_num(0.94135),I2F14::from_num(0.944),I2F14::from_num(0.9465),I2F14::from_num(0.949),I2F14::from_num(0.9514),I2F14::from_num(0.9538),I2F14::from_num(0.9561),I2F14::from_num(0.9584),I2F14::from_num(0.9606),I2F14::from_num(0.9627),I2F14::from_num(0.9648),I2F14::from_num(0.9668),I2F14::from_num(0.9688),I2F14::from_num(0.9707),I2F14::from_num(0.97253),I2F14::from_num(0.97437),I2F14::from_num(0.9761),I2F14::from_num(0.9777),I2F14::from_num(0.9794),I2F14::from_num(0.9809),I2F14::from_num(0.98236),I2F14::from_num(0.9838),I2F14::from_num(0.98517),I2F14::from_num(0.9865),I2F14::from_num(0.98773),I2F14::from_num(0.98895),I2F14::from_num(0.99005),I2F14::from_num(0.99115),I2F14::from_num(0.9921),I2F14::from_num(0.9931),I2F14::from_num(0.99396),I2F14::from_num(0.9948),I2F14::from_num(0.99554),I2F14::from_num(0.9963),I2F14::from_num(0.9969),I2F14::from_num(0.9975),I2F14::from_num(0.99805),I2F14::from_num(0.9985),I2F14::from_num(0.9989),I2F14::from_num(0.9992),I2F14::from_num(0.9995),I2F14::from_num(0.9997),I2F14::from_num(0.9999),I2F14::from_num(0.99994),I2F14::from_num(1.0)];
    let resolution=12-7;
    let mut sinval:I2F14;
    let startangle:i32=-3140;
    let endangle:i32=3140;

    loop {
        //let data: u16 = adc1.read(&mut ch0).unwrap();
        //block!(timer.wait()).unwrap();
        writeln!(tx, "Start Loop");
        led.set_high();
        for j in 1..100 {
            angle=I4F12::from_num(-3.14);
            for i in startangle..endangle {
                //sinval=sinf(angle); // 16.6s libm float32
                //sinval=2.0*angle; // 2s baseline
                sinval=table_trig::sin_t(angle,&sin_table::SIN_TABLE,resolution);
                angle+=I4F12::from_num(0.001);    
                //writeln!(tx, "{} {}",angle,sinval);
                //angle+=0.0001;
            }    
        }
        //writeln!(tx, "ADC Reading {}", data).unwrap();
        //block!(timer.wait()).unwrap();
        writeln!(tx, "End Loop");
        led.set_low();
        //angle=-3.14;
        angle=I4F12::from_num(-3.14);
        for i in startangle..endangle {
            sinval=table_trig::sin_t(angle,&sin_table::SIN_TABLE,resolution);
            angle+=I4F12::from_num(0.001);    
        }
    }
}
