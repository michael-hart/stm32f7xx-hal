(WIP) stm32f7xx-hal
===================

_stm32f7xx-hal_ contains a multi device hardware abstraction on top of the
peripheral access API for the STMicro STM32F7 series microcontrollers. The
selection of the MCU is done by feature gates, typically specified by board
support crates. Currently supported configurations are:

* stm32f756

The idea behind this crate is to gloss over the slight differences in the
various peripherals available on those MCUs so a HAL can be written for all
chips in that same family without having to cut and paste crates for every
single model.

Collaboration on this crate is highly welcome as are pull requests!

This crate relies on Adam Greigs fantastic [stm32f7][] crate to provide
appropriate register definitions and implements a partial set of the
[embedded-hal][] traits.

ALL of this implementation was based on the [stm32f4xx-hal][] crate by Daniel
Egger.

[stm32f7]: https://crates.io/crates/stm32f7
[stm32f7xx-hal]: https://github.com/therealprof/stm32f7xx-hal
[embedded-hal]: https://github.com/japaric/embedded-hal.git

License
-------

[0-clause BSD license](LICENSE-0BSD.txt).
