#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "16 - WWDG"]
    WWDG = 16,
    #[doc = "17 - PVD"]
    PVD = 17,
    #[doc = "18 - FLASH"]
    FLASH = 18,
    #[doc = "19 - RCC"]
    RCC = 19,
    #[doc = "20 - EXTI7_0"]
    EXTI7_0 = 20,
    #[doc = "21 - AWU"]
    AWU = 21,
    #[doc = "22 - DMA1_CHANNEL1"]
    DMA1_CHANNEL1 = 22,
    #[doc = "23 - DMA1_CHANNEL2"]
    DMA1_CHANNEL2 = 23,
    #[doc = "24 - DMA1_CHANNEL3"]
    DMA1_CHANNEL3 = 24,
    #[doc = "25 - DMA1_CHANNEL4"]
    DMA1_CHANNEL4 = 25,
    #[doc = "26 - DMA1_CHANNEL5"]
    DMA1_CHANNEL5 = 26,
    #[doc = "27 - DMA1_CHANNEL6"]
    DMA1_CHANNEL6 = 27,
    #[doc = "28 - DMA1_CHANNEL7"]
    DMA1_CHANNEL7 = 28,
    #[doc = "29 - ADC"]
    ADC = 29,
    #[doc = "30 - I2C1_EV"]
    I2C1_EV = 30,
    #[doc = "31 - I2C1_ER"]
    I2C1_ER = 31,
    #[doc = "32 - USART1"]
    USART1 = 32,
    #[doc = "33 - SPI1"]
    SPI1 = 33,
    #[doc = "34 - TIM1_BRK"]
    TIM1_BRK = 34,
    #[doc = "35 - TIM1_UP"]
    TIM1_UP = 35,
    #[doc = "36 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 36,
    #[doc = "37 - TIM1_CC"]
    TIM1_CC = 37,
    #[doc = "38 - TIM2"]
    TIM2 = 38,
}
unsafe impl crate::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    extern "C" {
        fn WWDG();
        fn PVD();
        fn FLASH();
        fn RCC();
        fn EXTI7_0();
        fn AWU();
        fn DMA1_CHANNEL1();
        fn DMA1_CHANNEL2();
        fn DMA1_CHANNEL3();
        fn DMA1_CHANNEL4();
        fn DMA1_CHANNEL5();
        fn DMA1_CHANNEL6();
        fn DMA1_CHANNEL7();
        fn ADC();
        fn I2C1_EV();
        fn I2C1_ER();
        fn USART1();
        fn SPI1();
        fn TIM1_BRK();
        fn TIM1_UP();
        fn TIM1_TRG_COM();
        fn TIM1_CC();
        fn TIM2();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.external_interrupts"]
    #[no_mangle]
    pub static __EXTERNAL_INTERRUPTS: [Vector; 23] = [
        Vector { _handler: WWDG },
        Vector { _handler: PVD },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
        Vector { _handler: EXTI7_0 },
        Vector { _handler: AWU },
        Vector {
            _handler: DMA1_CHANNEL1,
        },
        Vector {
            _handler: DMA1_CHANNEL2,
        },
        Vector {
            _handler: DMA1_CHANNEL3,
        },
        Vector {
            _handler: DMA1_CHANNEL4,
        },
        Vector {
            _handler: DMA1_CHANNEL5,
        },
        Vector {
            _handler: DMA1_CHANNEL6,
        },
        Vector {
            _handler: DMA1_CHANNEL7,
        },
        Vector { _handler: ADC },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: USART1 },
        Vector { _handler: SPI1 },
        Vector { _handler: TIM1_BRK },
        Vector { _handler: TIM1_UP },
        Vector {
            _handler: TIM1_TRG_COM,
        },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
    ];
}
pub const TIM2: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const AFIO: afio::Afio = unsafe { afio::Afio::from_ptr(0x4001_0000usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_0400usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0800usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1000usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1400usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2400usize as _) };
pub const TIM1: timer::Adtm = unsafe { timer::Adtm::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const DMA1: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_0000usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const OPA: extend::Extend = unsafe { extend::Extend::from_ptr(0x4002_3800usize as _) };
pub const PFIC: pfic::Pfic = unsafe { pfic::Pfic::from_ptr(0xe000_e000usize as _) };
pub const SYSTICK: systick::Systick = unsafe { systick::Systick::from_ptr(0xe000_f000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub fn GPIO(n: usize) -> gpio::Gpio {
    unsafe { gpio::Gpio::from_ptr((1073809408 + 1024 * n) as _) }
}
#[path = "../../peripherals/adc_v003.rs"]
pub mod adc;
#[path = "../../peripherals/afio_v003.rs"]
pub mod afio;
#[path = "../../peripherals/dma_v1.rs"]
pub mod dma;
#[path = "../../peripherals/extend_v0.rs"]
pub mod extend;
#[path = "../../peripherals/exti_common.rs"]
pub mod exti;
#[path = "../../peripherals/flash_v0.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_v0.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v0.rs"]
pub mod i2c;
#[path = "../../peripherals/pfic_rv2.rs"]
pub mod pfic;
#[path = "../../peripherals/rcc_v003.rs"]
pub mod rcc;
#[path = "../../peripherals/spi_v0.rs"]
pub mod spi;
#[path = "../../peripherals/systick_rv2.rs"]
pub mod systick;
#[path = "../../peripherals/timer_v3.rs"]
pub mod timer;
#[path = "../../peripherals/usart_common.rs"]
pub mod usart;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 0;
pub const FLASH_SIZE: usize = 16384;
pub const WRITE_SIZE: usize = 64;
