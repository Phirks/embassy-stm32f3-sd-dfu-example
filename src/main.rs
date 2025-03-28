#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt_rtt::*;
use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_executor::Spawner;
use embassy_stm32::mode::Blocking;
use embassy_stm32::spi::{Config,Spi};
use embassy_stm32::time::Hertz;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time;
use embedded_hal::spi;
use embedded_sdmmc::asynchronous::{SdCard, SdCardError::*};
use panic_reset as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let cs_sd = Output::new(p.PA4, Level::High, Speed::High);
    let mut spi_config = Config::default();
    spi_config.frequency = Hertz(200_000);
    spi_config.mode = spi::MODE_0;
    spi_config.bit_order = embassy_stm32::spi::BitOrder::MsbFirst;
    spi_config.rise_fall_speed = Speed::Medium;

    let bus: Mutex<NoopRawMutex, Spi<'_, Async>>  = Mutex::new(Spi::new(p.SPI1, p.PA5, p.PA7, p.PA6, p.DMA1_CH3, p.DMA1_CH2, spi_config));  
        #[cfg(feature = "defmt")]
        defmt::info!("Start Init");

    {
        let mut bus_locked = bus.lock().await;
        let mut read: [u8; 10] = [0; 10];
        bus_locked.write(&read).await.unwrap();

    }
    
    let spi = embassy_embedded_hal::shared_bus::asynch::spi::SpiDevice::new(&bus, cs_sd);
    let sdcard = SdCard::new(spi,embassy_time::Delay);
    //let mut button = ExtiInput::new(p.PC6, p.EXTI6, Pull::Up);

    let result = match sdcard.num_bytes().await {
        Ok(bytes) => {
            #[cfg(feature = "defmt")]
            defmt::info!("{}",bytes);
        }
        Err(error) => {
            match error {
                Transport => {
                    #[cfg(feature = "defmt")]
                    defmt::info!("Transport");

                }
                CantEnableCRC=> {
                    #[cfg(feature = "defmt")]
                    defmt::info!("CantEnableCRC");

                }
                TimeoutReadBuffer => {
                    #[cfg(feature = "defmt")]
                    defmt::info!("TimeoutReadBuffer");

                }
                TimeoutWaitNotBusy => {
                    #[cfg(feature = "defmt")]
                    defmt::info!("TimeoutWaitNotBusy");

                }
                TimeoutCommand(u8) => {
                    #[cfg(feature = "defmt")]
                    defmt::info!("TimeoutCommand {}",u8);

                }
                TimeoutACommand(u8) => {
                    #[cfg(feature = "defmt")]
                    defmt::info!("TimeoutACommand {}",u8);

                }
                Cmd58Error => {
                    #[cfg(feature = "defmt")]
                    defmt::info!("Cmd58Error");

                }
                RegisterReadError => {
                    #[cfg(feature = "defmt")]
                    defmt::info!("RegisterReadError");

                }
                CrcError(arg1, arg2) => {
                    #[cfg(feature = "defmt")]
                    defmt::info!("CrcError {} {}",arg1,arg2);

                }
                ReadError => {
                    #[cfg(feature = "defmt")]
                    defmt::info!("ReadError");

                }
                WriteError => {
                    #[cfg(feature = "defmt")]
                    defmt::info!("WriteError");

                }
                BadState => {
                    #[cfg(feature = "defmt")]
                    defmt::info!("BadState");

                }
                CardNotFound => {
                    #[cfg(feature = "defmt")]
                    defmt::info!("CardNotFound");

                }    
                GpioError => {
                    #[cfg(feature = "defmt")]
                    defmt::info!("GpioError");

                }
                _=> {
                    #[cfg(feature = "defmt")]
                    defmt::info!("Unknown Error");
                }



            }
        }
    };
}
