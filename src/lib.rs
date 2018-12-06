#![no_std]
#![allow(non_camel_case_types)]

extern crate bare_metal;
extern crate cast;
extern crate cortex_m;
pub extern crate embedded_hal as hal;
extern crate void;

pub extern crate nb;
pub use nb::block;

pub extern crate stm32f7;

#[cfg(feature = "stm32f756")]
pub use stm32f7::stm32f7x6 as stm32;

// Enable use of interrupt macro
#[cfg(feature = "rt")]
pub use stm32f7::interrupt;

#[cfg(any(feature = "stm32f7561"))]
pub mod delay;

#[cfg(any(feature = "stm32f756"))]
pub mod gpio;

#[cfg(any(feature = "stm32f7561"))]
pub mod i2c;

#[cfg(any(feature = "stm32f7561"))]
pub mod prelude;

#[cfg(any(feature = "stm32f756"))]
pub mod rcc;

#[cfg(any(feature = "stm32f7561"))]
pub mod serial;

#[cfg(any(feature = "stm32f756"))]
pub mod spi;

pub mod time;

#[cfg(any(feature = "stm32f7561"))]
pub mod timer;
