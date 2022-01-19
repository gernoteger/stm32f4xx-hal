//! Convenience re-export of multiple traits.
//!
//! This allows a HAL user to conveniently import this module and have all the
//! helper traits already imported.
//! Otherwise the use of peripherals would require the import of the
//! corresponding module and the import of the trait, which connects this HAL
//! to the autogenerated svd2rust API in [crate::pac].
//!
//! # Example
//!
//! Consider the following code.
//!
//! ```
//! #[entry]
//! fn main() -> ! {
//!     let dp = pac::Peripherals::take().unwrap();
//!     let gpiog = dp.GPIOG.split();
//!     let mut led1 = gpiog.pg13.into_push_pull_output();
//!     led1.set_high().unwrap();
//! }
//! ```
//!
//! Without the prelude we would have to import the following traits:
//!
//! ```
//! use stm32f4xx_hal::gpio::GpioExt; // for the split method.
//! use embedded_hal::digital::v2::OutputPin; // for the set_high() function.
//! // And more use-statements with more complex code.
//! ```
//!
//! These imports are a bit unintuitive, because we can create the objects
//! without the import. But we need these traits to access most of their
//! functions.
//!
//! The prelude module keeps the import section cleaner:
//! ```
//! use stm32f4xx_hal::prelude::*;
//! ```
pub use embedded_hal::adc::OneShot as _embedded_hal_adc_OneShot;
pub use embedded_hal::blocking::delay::DelayMs as _embedded_hal_blocking_delay_DelayMs;
pub use embedded_hal::blocking::delay::DelayUs as _embedded_hal_blocking_delay_DelayUs;
pub use embedded_hal::blocking::i2c::{
    Read as _embedded_hal_blocking_i2c_Read, Write as _embedded_hal_blocking_i2c_Write,
    WriteRead as _embedded_hal_blocking_i2c_WriteRead,
};
pub use embedded_hal::blocking::serial::Write as _embedded_hal_blocking_serial_Write;
pub use embedded_hal::blocking::spi::{
    Transfer as _embedded_hal_blocking_spi_Transfer, Write as _embedded_hal_blocking_spi_Write,
};
pub use embedded_hal::serial::Read as _embedded_hal_serial_Read;
pub use embedded_hal::serial::Write as _embedded_hal_serial_Write;
pub use embedded_hal::spi::FullDuplex as _embedded_hal_spi_FullDuplex;
pub use embedded_hal::timer::CountDown as _embedded_hal_timer_CountDown;
pub use embedded_hal::watchdog::Watchdog as _embedded_hal_watchdog_Watchdog;
pub use embedded_hal::watchdog::WatchdogDisable as _embedded_hal_watchdog_WatchdogDisable;
pub use embedded_hal::watchdog::WatchdogEnable as _embedded_hal_watchdog_WatchdogEnable;
pub use embedded_hal::Capture as _embedded_hal_Capture;
pub use embedded_hal::Pwm as _embedded_hal_Pwm;
pub use embedded_hal::Qei as _embedded_hal_Qei;
pub use fugit::ExtU32 as _fugit_ExtU32;

#[cfg(all(feature = "device-selected", feature = "dac"))]
pub use crate::dac::DacExt as _stm32f4xx_hal_dac_DacExt;
#[cfg(feature = "rtic")]
#[cfg(not(feature = "stm32f410"))]
pub use crate::fugit::MonoTimerExt as _stm32f4xx_hal_fugit_MonoTimerExt;
pub use crate::fugit::PwmExt as _stm32f4xx_hal_fugit_PwmExt;
pub use crate::fugit::SysCounterExt as _stm32f4xx_hal_fugit_SysCounterExt;
pub use crate::fugit::TimerExt as _stm32f4xx_hal_fugit_TimerExt;
pub use crate::gpio::ExtiPin as _stm32f4xx_hal_gpio_ExtiPin;
pub use crate::gpio::GpioExt as _stm32f4xx_hal_gpio_GpioExt;
pub use crate::i2c::Pins as _stm32f4xx_hal_i2c_Pins;
pub use crate::rcc::RccExt as _stm32f4xx_hal_rcc_RccExt;
#[cfg(all(feature = "device-selected", feature = "rng"))]
pub use crate::rng::RngExt as _stm32f4xx_hal_rng_RngExt;
pub use crate::syscfg::SysCfgExt as _stm32f4xx_hal_syscfg_SysCfgExt;
pub use crate::time::U32Ext as _stm32f4xx_hal_time_U32Ext;
