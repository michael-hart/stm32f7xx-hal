use core::ptr;

use hal;
pub use hal::spi::{Mode, Phase, Polarity};
use nb;

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use stm32::{RCC, SPI1, SPI2};

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use stm32::SPI3;

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use stm32::SPI4;

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use stm32::SPI5;

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use stm32::SPI6;

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpioa::PA9;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpioa::{PA1, PA11};
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpioa::{PA10, PA12};
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpioa::{PA5, PA6, PA7};

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpiob::PB0;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpiob::PB12;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpiob::PB2;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpiob::PB8;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpiob::{PB10, PB13, PB14, PB15, PB3, PB4, PB5};

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpioc::PC1;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpioc::PC7;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpioc::{PC10, PC11, PC12};
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpioc::{PC2, PC3};

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpiod::PD0;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpiod::{PD3, PD6};

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpioe::{PE12, PE13, PE14, PE2, PE5, PE6};

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpiof::{PF11, PF7, PF8, PF9};

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpiog::PG11;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpiog::PG14;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpiog::{PG12, PG13};

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpioh::{PH6, PH7};

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::gpioi::{PI1, PI2, PI3};

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::AF7;
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
use gpio::{Alternate, AF5, AF6};

use rcc::Clocks;
use time::Hertz;

/// SPI error
#[derive(Debug)]
pub enum Error {
    /// Overrun occurred
    Overrun,
    /// Mode fault occurred
    ModeFault,
    /// CRC error
    Crc,
    #[doc(hidden)]
    _Extensible,
}

pub trait Pins<SPI> {}
pub trait PinSck<SPI> {}
pub trait PinMiso<SPI> {}
pub trait PinMosi<SPI> {}

impl<SPI, SCK, MISO, MOSI> Pins<SPI> for (SCK, MISO, MOSI)
where
    SCK: PinSck<SPI>,
    MISO: PinMiso<SPI>,
    MOSI: PinMosi<SPI>,
{
}

/// A filler type for when the SCK pin is unnecessary
pub struct NoSck;
/// A filler type for when the Miso pin is unnecessary
pub struct NoMiso;
/// A filler type for when the Mosi pin is unnecessary
pub struct NoMosi;

macro_rules! pins {
    ($($SPIX:ty: SCK: [$($SCK:ty),*] MISO: [$($MISO:ty),*] MOSI: [$($MOSI:ty),*])+) => {
        $(
            $(
                impl PinSck<$SPIX> for $SCK {}
            )*
            $(
                impl PinMiso<$SPIX> for $MISO {}
            )*
            $(
                impl PinMosi<$SPIX> for $MOSI {}
            )*
        )+
    }
}

// #[cfg(any(
//     feature = "stm32f7x2",
//     feature = "stm32f7x3",
//     feature = "stm32f7x5",
//     feature = "stm32f7x6",
//     feature = "stm32f7x7",
//     feature = "stm32f7x9",
// ))]
// pins! {
//     SPI1:
//         SCK: [
//             NoSck,
//             PA5<Alternate<AF5>>,
//             PB3<Alternate<AF5>>
//         ]
//         MISO: [
//             NoMiso,
//             PA6<Alternate<AF5>>,
//             PB4<Alternate<AF5>>
//         ]
//         MOSI: [
//             NoMosi,
//             PA7<Alternate<AF5>>,
//             PB5<Alternate<AF5>>
//         ]

//     SPI2:
//         SCK: [
//             NoSck,
//             PB10<Alternate<AF5>>,
//             PB13<Alternate<AF5>>
//         ]
//         MISO: [
//             NoMiso,
//             PB14<Alternate<AF5>>,
//             PC2<Alternate<AF5>>
//         ]
//         MOSI: [
//             NoMosi,
//             PB15<Alternate<AF5>>,
//             PC3<Alternate<AF5>>
//         ]
// }

// #[cfg(any(
//     feature = "stm32f7x2",
//     feature = "stm32f7x3",
//     feature = "stm32f7x5",
//     feature = "stm32f7x6",
//     feature = "stm32f7x7",
//     feature = "stm32f7x9",
// ))]
// pins! {
//     SPI3:
//         SCK: [
//             NoSck,
//             PB3<Alternate<AF6>>,
//             PC10<Alternate<AF6>>
//         ]
//         MISO: [
//             NoMiso,
//             PB4<Alternate<AF6>>,
//             PC11<Alternate<AF6>>
//         ]
//         MOSI: [
//             NoMosi,
//             PB5<Alternate<AF6>>,
//             PC12<Alternate<AF6>>
//         ]
// }

// #[cfg(any(
//     feature = "stm32f7x2",
//     feature = "stm32f7x3",
//     feature = "stm32f7x5",
//     feature = "stm32f7x6",
//     feature = "stm32f7x7",
//     feature = "stm32f7x9",
// ))]
// pins! {
//     SPI2:
//         SCK: [PD3<Alternate<AF5>>]
//         MISO: []
//         MOSI: []
//     SPI3:
//         SCK: []
//         MISO: []
//         MOSI: [PD6<Alternate<AF5>>]
//     SPI4:
//         SCK: [
//             NoSck,
//             PE2<Alternate<AF5>>,
//             PE12<Alternate<AF5>>
//         ]
//         MISO: [
//             NoMiso,
//             PE5<Alternate<AF5>>,
//             PE13<Alternate<AF5>>
//         ]
//         MOSI: [
//             NoMosi,
//             PE6<Alternate<AF5>>,
//             PE14<Alternate<AF5>>
//         ]
// }

// #[cfg(any(
//     feature = "stm32f7x2",
//     feature = "stm32f7x3",
//     feature = "stm32f7x5",
//     feature = "stm32f7x6",
//     feature = "stm32f7x7",
//     feature = "stm32f7x9",
// ))]
// pins! {
//     SPI2:
//         SCK: [PI1<Alternate<AF5>>]
//         MISO: [PI2<Alternate<AF5>>]
//         MOSI: [PI3<Alternate<AF5>>]
// }

// #[cfg(any(
//     feature = "stm32f7x2",
//     feature = "stm32f7x3",
//     feature = "stm32f7x5",
//     feature = "stm32f7x6",
//     feature = "stm32f7x7",
//     feature = "stm32f7x9",
// ))]
// pins! {
//     SPI2:
//         SCK: [PC7<Alternate<AF5>>]
//         MISO: []
//         MOSI: []
// }

// #[cfg(any(
//     feature = "stm32f7x2",
//     feature = "stm32f7x3",
//     feature = "stm32f7x5",
//     feature = "stm32f7x6",
//     feature = "stm32f7x7",
//     feature = "stm32f7x9",
// ))]
// pins! {
//     SPI5:
//         SCK: [
//             NoSck,
//             PB0<Alternate<AF6>>
//         ]
//         MISO: [
//             NoMiso,
//             PA12<Alternate<AF6>>
//         ]
//         MOSI: [
//             NoMosi,
//             PA10<Alternate<AF6>>,
//             PB8<Alternate<AF6>>
//         ]
// }

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
pins! {
    SPI3:
        SCK: [PB12<Alternate<AF7>>]
        MISO: []
        MOSI: []
    SPI4:
        SCK: [PB13<Alternate<AF6>>]
        MISO: [PA11<Alternate<AF6>>]
        MOSI: [PA1<Alternate<AF5>>]
    SPI5:
        SCK: [
            PE2<Alternate<AF6>>,
            PE12<Alternate<AF6>>
        ]
        MISO: [
            PE5<Alternate<AF6>>,
            PE13<Alternate<AF6>>
        ]
        MOSI: [
            PE6<Alternate<AF6>>,
            PE14<Alternate<AF6>>
        ]
}

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
pins! {
    SPI2:
        SCK: [PA9<Alternate<AF5>>]
        MISO: [PA12<Alternate<AF5>>]
        MOSI: [PA10<Alternate<AF5>>]
}

// #[cfg(any(
//     feature = "stm32f7x2",
//     feature = "stm32f7x3",
//     feature = "stm32f7x5",
//     feature = "stm32f7x6",
//     feature = "stm32f7x7",
//     feature = "stm32f7x9",
// ))]
// pins! {
//     SPI5:
//         SCK: [
//             NoSck,
//             PF7<Alternate<AF5>>,
//             PH6<Alternate<AF5>>
//         ]
//         MISO: [
//             NoMiso,
//             PF8<Alternate<AF5>>,
//             PH7<Alternate<AF5>>
//         ]
//         MOSI: [
//             NoMosi,
//             PF9<Alternate<AF5>>,
//             PF11<Alternate<AF5>>
//         ]

//     SPI6:
//         SCK: [
//             NoSck,
//             PG13<Alternate<AF5>>
//         ]
//         MISO: [
//             NoMiso,
//             PG12<Alternate<AF5>>
//         ]
//         MOSI: [
//             NoMosi,
//             PG14<Alternate<AF5>>
//         ]
// }

// #[cfg(any(
//     feature = "stm32f7x2",
//     feature = "stm32f7x3",
//     feature = "stm32f7x5",
//     feature = "stm32f7x6",
//     feature = "stm32f7x7",
//     feature = "stm32f7x9",
// ))]
// pins! {
//     SPI2:
//         SCK: [PA9<Alternate<AF5>>]
//         MISO: []
//         MOSI: [PC1<Alternate<AF7>>]

//     SPI3:
//         SCK: []
//         MISO: []
//         MOSI: [
//             PB0<Alternate<AF7>>,
//             PB2<Alternate<AF7>>,
//             PD0<Alternate<AF6>>
//         ]

//     SPI4:
//         SCK: [PG11<Alternate<AF6>>]
//         MISO: [
//             PG12<Alternate<AF6>>,
//             PD0<Alternate<AF5>>
//         ]
//         MOSI: [PG13<Alternate<AF6>>]
// }

// #[cfg(any(
//     feature = "stm32f7x2",
//     feature = "stm32f7x3",
//     feature = "stm32f7x5",
//     feature = "stm32f7x6",
//     feature = "stm32f7x7",
//     feature = "stm32f7x9",
// ))]
// pins! {
//     SPI2:
//         SCK: [PA9<Alternate<AF5>>]
//         MISO: []
//         MOSI: [PC1<Alternate<AF5>>]
// }

#[derive(Debug)]
pub struct Spi<SPI, PINS> {
    spi: SPI,
    pins: PINS,
}

macro_rules! hal {
    ($($SPIX:ident: ($spiX:ident, $apbXenr:ident, $spiXen:ident, $pclkX:ident),)+) => {
        $(
            impl<PINS> Spi<$SPIX, PINS> {
                pub fn $spiX(
                    spi: $SPIX,
                    pins: PINS,
                    mode: Mode,
                    freq: Hertz,
                    clocks: Clocks
                ) -> Self
                where PINS: Pins<$SPIX> {
                    // NOTE(unsafe) This executes only during initialisation
                    let rcc = unsafe { &(*RCC::ptr()) };

                    // Enable clock for SPI
                    rcc.$apbXenr.modify(|_, w| w.$spiXen().set_bit());

                    // disable SS output
                    spi.cr2.write(|w| w.ssoe().clear_bit());

                    let br = match clocks.$pclkX().0 / freq.0 {
                        0 => unreachable!(),
                        1...2 => 0b000,
                        3...5 => 0b001,
                        6...11 => 0b010,
                        12...23 => 0b011,
                        24...47 => 0b100,
                        48...95 => 0b101,
                        96...191 => 0b110,
                        _ => 0b111,
                    };

                    // mstr: master configuration
                    // lsbfirst: MSB first
                    // ssm: enable software slave management (NSS pin free for other uses)
                    // ssi: set nss high = master mode
                    // dff: 8 bit frames
                    // bidimode: 2-line unidirectional
                    // spe: enable the SPI bus
                    spi.cr1.write(|w| {
                        w.cpha()
                            .bit(mode.phase == Phase::CaptureOnSecondTransition)
                            .cpol()
                            .bit(mode.polarity == Polarity::IdleHigh)
                            .mstr()
                            .set_bit()
                            .br()
                            .bits(br)
                            .lsbfirst()
                            .clear_bit()
                            .ssm()
                            .set_bit()
                            .ssi()
                            .set_bit()
                            .rxonly()
                            .clear_bit()
                            .dff()
                            .clear_bit()
                            .bidimode()
                            .clear_bit()
                            .spe()
                            .set_bit()
                    });

                    Spi { spi, pins }
                }

                pub fn free(self) -> ($SPIX, PINS) {
                    (self.spi, self.pins)
                }
            }

            impl<PINS> hal::spi::FullDuplex<u8> for Spi<$SPIX, PINS> {
                type Error = Error;

                fn read(&mut self) -> nb::Result<u8, Error> {
                    let sr = self.spi.sr.read();

                    Err(if sr.ovr().bit_is_set() {
                        nb::Error::Other(Error::Overrun)
                    } else if sr.modf().bit_is_set() {
                        nb::Error::Other(Error::ModeFault)
                    } else if sr.crcerr().bit_is_set() {
                        nb::Error::Other(Error::Crc)
                    } else if sr.rxne().bit_is_set() {
                        // NOTE(read_volatile) read only 1 byte (the svd2rust API only allows
                        // reading a half-word)
                        return Ok(unsafe {
                            ptr::read_volatile(&self.spi.dr as *const _ as *const u8)
                        });
                    } else {
                        nb::Error::WouldBlock
                    })
                }

                fn send(&mut self, byte: u8) -> nb::Result<(), Error> {
                    let sr = self.spi.sr.read();

                    Err(if sr.ovr().bit_is_set() {
                        nb::Error::Other(Error::Overrun)
                    } else if sr.modf().bit_is_set() {
                        nb::Error::Other(Error::ModeFault)
                    } else if sr.crcerr().bit_is_set() {
                        nb::Error::Other(Error::Crc)
                    } else if sr.txe().bit_is_set() {
                        // NOTE(write_volatile) see note above
                        unsafe { ptr::write_volatile(&self.spi.dr as *const _ as *mut u8, byte) }
                        return Ok(());
                    } else {
                        nb::Error::WouldBlock
                    })
                }

            }

            impl<PINS> ::hal::blocking::spi::transfer::Default<u8> for Spi<$SPIX, PINS> {}

            impl<PINS> ::hal::blocking::spi::write::Default<u8> for Spi<$SPIX, PINS> {}
        )+
    }
}

#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
hal! {
    SPI1: (spi1, apb2enr, spi1en, pclk2),
    SPI2: (spi2, apb1enr, spi2en, pclk1),
}
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
hal! {
    SPI3: (spi3, apb1enr, spi3en, pclk1),
}
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
hal! {
    SPI4: (spi4, apb2enr, spi4en, pclk2),
}
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
hal! {
    SPI5: (spi5, apb2enr, spi5en, pclk2),
}
#[cfg(any(
    feature = "stm32f7x2",
    feature = "stm32f7x3",
    feature = "stm32f7x5",
    feature = "stm32f7x6",
    feature = "stm32f7x7",
    feature = "stm32f7x9",
))]
hal! {
    SPI6: (spi6, apb2enr, spi6en, pclk2),
}
