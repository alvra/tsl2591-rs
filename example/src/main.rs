#![no_std]
#![no_main]

//! Example usage for TSL2951 on STM32F303
use cortex_m::asm::delay;
pub use panic_itm;
use core::ops::Range;
pub use cortex_m::{asm::bkpt, iprint, iprintln};
use cortex_m_rt::entry;
use cortex_m_semihosting::{ hprintln, hprint };
use stm32f3xx_hal::pac::I2C1;
use stm32f3xx_hal::{i2c::I2c, pac, prelude::*};
use tsl2591;

#[entry]
/// Main Thread
fn main() -> ! {
    // Get peripherals, clocks and freeze them
    // let cp = cortex_m::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let mut i2c = I2c::new(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);
    let mut t = tsl2591::Driver::new(i2c).unwrap();
    t.set_timing().unwrap();
    t.set_gain().unwrap();
    t.enable().unwrap();
    loop {
        // Sample
        // disable(&mut i2c);
        delay(120);
        let status = t.get_status().unwrap();
        // Read data
        let (x, a) = t.get_channel_data().unwrap();
        let lux = t.calculate_lux(x, a).unwrap();


        // iprintln!(&mut cp.ITM.stim[0], "CHAN0LOW {} {}", x[0], x[1]);
        // iprintln!(&mut cp.ITM.stim[0], "CHAN1LOW {} {}", a[0], a[1]);
        iprintln!(&mut cp.ITM.stim[0], "Lux {}", lux);
    }
}


