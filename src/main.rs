#![no_std]
#![no_main]

use cortex_m::asm::nop;
use embedded_sdmmc::SdCard;
use stm32f3xx_hal::spi::Spi;
use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::pac;

use panic_reset as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(48.MHz())
        .pclk1(24.MHz())
        .freeze(&mut flash.acr);
    let mut cs = gpioa
        .pa4
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
     let sck = gpioa
        .pa5
        .into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let miso = gpioa
        .pa6
        .into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let mosi = gpioa
        .pa7
        .into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);

    let mut bus = Spi::new(dp.SPI1, (sck,miso,mosi), 200.kHz(), clocks, &mut rcc.apb2);  
    cs.set_high();

    let delaycount = 10;
    for _i in 0..delaycount{nop();}
    {
        let mut read: [u8; 10] = [0; 10];
        bus.write(&read);
    }
    for _i in 0..delaycount{nop();}
   
    cs.set_low();
    for _i in 0..delaycount{nop();}
    let cmd0: [u8;6] = [0x40,0x00,0x00,0x00,0x00,0x95];
    bus.write(&cmd0);
    cs.set_high();
    for _i in 0..delaycount{nop();}

    for _j in 0..50{
        cs.set_low();
        for _i in 0..delaycount{nop();}
        let ff: [u8;1]= [0xFF];
        bus.write(&ff);
        cs.set_high();
        for _i in 0..delaycount{nop();}
        
        cs.set_low();
        for _i in 0..delaycount{nop();}
        let ff: [u8;1]= [0xFF];
        bus.write(&ff);
        cs.set_high();
        for _i in 0..delaycount{nop();}
    } 
    loop{}
}
