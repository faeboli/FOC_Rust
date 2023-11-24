
#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use nb::block;

use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac::{self, USART1},
    adc,
    prelude::*,
    serial::{Config, Serial, Rx},
    timer::{Channel, Tim3NoRemap, Timer},
};

use core::fmt::Write;
use fixed::{types::{I2F14, I6F10}, traits::Fixed};
use fixed::types::I4F12;
use heapless::String;
mod FOC_func;


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
    let (mut tx, mut rx) = serial.split();
    // Configure pa1 as an analog input
    let mut ch0 = gpioa.pa1.into_analog(&mut gpioa.crl);
    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(64.Hz()).unwrap();

    let mut angle: I4F12;
    
    let resolution = 12-7; // 2^-7
    let mut sinval: I2F14;
    let startangle: i32 = -3140;
    let endangle: i32 = 3140;

    loop {
        let mut buffer;
        writeln!(tx,"Enter command 'speed' or 'angle' or 'plot'");
        buffer=block!(rx.read()).unwrap();
        match buffer {
            b's' => { // speed mode
                writeln!(tx,"Set speed rad per second:");
                let speed=readln_I4F12(&mut rx);
                writeln!(tx,"speed request: {} rad per s",speed);
                writeln!(tx,"Set turns number:");
                let rounds=readln_u8(&mut rx);
                writeln!(tx,"rounds request: {}",rounds);
                writeln!(tx,"Set Vd:");
                let Vd=readln_I6F10(&mut rx);
                writeln!(tx,"Vd request: {}",Vd);
                writeln!(tx,"Set Vq:");
                let Vq=readln_I6F10(&mut rx);
                writeln!(tx,"Vq request: {}",Vq);
                writeln!(tx,"Set Max V:");
                let max=readln_I6F10(&mut rx);
                writeln!(tx,"Max V request: {}",max);
                for j in 1..rounds {
                    angle = I4F12::from_num(-3.14);
                    for i in startangle..endangle {
                        let (Valpha,Vbeta)=FOC_func::inverse_park(Vd, Vq, angle);
//                        writeln!(tx,"Va={},Vb={}",Valpha,Vbeta);
                        let (i,j,k)=FOC_func::mod_inverse_clarke(Valpha, Vbeta);
  //                      writeln!(tx,"i={},j={},k={}",i,j,k);
                        let (U,V,W,sector)=FOC_func::svpwm(i,j,k,max);
                        angle += speed/64;
                        block!(timer.wait()).unwrap();
                    }
                }
            }
            b'a' => {
                writeln!(tx,"Set angle:");
                let theta=readln_I4F12(&mut rx);
                writeln!(tx,"angle request: {}",theta);
                writeln!(tx,"Set Vd:");
                let Vd=readln_I6F10(&mut rx);
                writeln!(tx,"Vd request: {}",Vd);
                writeln!(tx,"Set Vq:");
                let Vq=readln_I6F10(&mut rx);
                writeln!(tx,"Vq request: {}",Vq);
                let max=readln_I6F10(&mut rx);
                writeln!(tx,"Set Max V:");
                writeln!(tx,"Max V request: {}",max);
                let (Valpha,Vbeta)=FOC_func::inverse_park(Vd,Vq,theta);
                writeln!(tx,"Va={},Vb={}",Valpha,Vbeta);
                let (i,j,k)=FOC_func::mod_inverse_clarke(Valpha, Vbeta);
                writeln!(tx,"i={},j={},k={}",i,j,k);
                let (U,V,W,sector)=FOC_func::svpwm(i,j,k,max);
                writeln!(tx,"sector: {}",sector);
                writeln!(tx,"U={},V={},W={}",U,V,W);
            }
            _ => {
                writeln!(tx,"Valid commands: 's'=speed mode or 'a' angle mode");
            }
        }
    }
}

fn readln_I6F10(rx:&mut Rx<USART1>) -> I6F10{
    let mut buffer:String<32>=String::new();
//    let mut buffer:[u8;32]=[0;32];
    let mut char:u8;
//    let mut i:usize=0;
    char=block!(rx.read()).unwrap();
    while char!=b'\n'{
        buffer.push(char as char);
//        buffer[i]=char;
//        i+=1;
        char=block!(rx.read()).unwrap();
    }
    //return I6F10::from_str(stringify!(buffer)).unwrap();
    return I6F10::from_str(buffer.as_str()).unwrap();
}

fn readln_I4F12(rx:&mut Rx<USART1>) -> I4F12{
    let mut buffer:String<32>=String::new();
    let mut char:u8;
//    let mut i:usize=0;
    char=block!(rx.read()).unwrap();
    while char!=b'\n'{
        buffer.push(char as char);
//        i+=1;
        char=block!(rx.read()).unwrap();
    }
    return I4F12::from_str(buffer.as_str()).unwrap();
}

fn readln_u8(rx:&mut Rx<USART1>) -> u8{
    let mut buffer:String<32>=String::new();
    let mut char:u8;
//    let mut i:usize=0;
    char=block!(rx.read()).unwrap();
    while char!=b'\n'{
        buffer.push(char as char);
//        i+=1;
        char=block!(rx.read()).unwrap();
    }
    return u8::from_str_radix(buffer.as_str(), 10).unwrap();
}