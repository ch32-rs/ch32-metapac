#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "16 - WWDG"]
    WWDG = 16,
    #[doc = "17 - OPA"]
    OPA = 17,
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
    #[doc = "32 - UART1"]
    UART1 = 32,
    #[doc = "33 - EXTI15_8"]
    EXTI15_8 = 33,
    #[doc = "34 - TIM1BRK"]
    TIM1BRK = 34,
    #[doc = "35 - TIM1UP"]
    TIM1UP = 35,
    #[doc = "36 - TIM1TRG"]
    TIM1TRG = 36,
    #[doc = "37 - TIM1CC"]
    TIM1CC = 37,
    #[doc = "38 - TIM2"]
    TIM2 = 38,
    #[doc = "39 - USBPD0"]
    USBPD0 = 39,
    #[doc = "40 - USBPD0WAKEUP"]
    USBPD0WAKEUP = 40,
    #[doc = "41 - USBPD1"]
    USBPD1 = 41,
    #[doc = "42 - USBPD1WAKEUP"]
    USBPD1WAKEUP = 42,
    #[doc = "43 - TIM3"]
    TIM3 = 43,
    #[doc = "44 - SPI1"]
    SPI1 = 44,
    #[doc = "45 - USBFS"]
    USBFS = 45,
    #[doc = "46 - USBFSWAKEUP"]
    USBFSWAKEUP = 46,
}
unsafe impl crate::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn WWDG();
        fn OPA();
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
        fn UART1();
        fn EXTI15_8();
        fn TIM1BRK();
        fn TIM1UP();
        fn TIM1TRG();
        fn TIM1CC();
        fn TIM2();
        fn USBPD0();
        fn USBPD0WAKEUP();
        fn USBPD1();
        fn USBPD1WAKEUP();
        fn TIM3();
        fn SPI1();
        fn USBFS();
        fn USBFSWAKEUP();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.external_interrupts")]
    #[unsafe(no_mangle)]
    pub static __EXTERNAL_INTERRUPTS: [Vector; 31] = [
        Vector { _handler: WWDG },
        Vector { _handler: OPA },
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
        Vector { _handler: UART1 },
        Vector { _handler: EXTI15_8 },
        Vector { _handler: TIM1BRK },
        Vector { _handler: TIM1UP },
        Vector { _handler: TIM1TRG },
        Vector { _handler: TIM1CC },
        Vector { _handler: TIM2 },
        Vector { _handler: USBPD0 },
        Vector {
            _handler: USBPD0WAKEUP,
        },
        Vector { _handler: USBPD1 },
        Vector {
            _handler: USBPD1WAKEUP,
        },
        Vector { _handler: TIM3 },
        Vector { _handler: SPI1 },
        Vector { _handler: USBFS },
        Vector {
            _handler: USBFSWAKEUP,
        },
    ];
}
pub const TIM2: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::Gptm3 = unsafe { timer::Gptm3::from_ptr(0x4000_0400usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
pub const AFIO: afio::Afio = unsafe { afio::Afio::from_ptr(0x4001_0000usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_0400usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0800usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0c00usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1000usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2400usize as _) };
pub const TIM1: timer::Adtm = unsafe { timer::Adtm::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const UART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const DMA1: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_0000usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const USBFS: usb::Usb = unsafe { usb::Usb::from_ptr(0x4002_3400usize as _) };
pub const EXTEND: extend::Exten = unsafe { extend::Exten::from_ptr(0x4002_3800usize as _) };
pub const USBPD0: usbpd::Usbpd = unsafe { usbpd::Usbpd::from_ptr(0x4002_4000usize as _) };
pub const USBPD1: usbpd::Usbpd = unsafe { usbpd::Usbpd::from_ptr(0x4002_4400usize as _) };
pub const OPA: opa::Opa = unsafe { opa::Opa::from_ptr(0x4002_4800usize as _) };
pub const PFIC: pfic::Pfic = unsafe { pfic::Pfic::from_ptr(0xe000_e000usize as _) };
pub const SYSTICK: systick::Systick = unsafe { systick::Systick::from_ptr(0xe000_f000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub fn GPIO(n: usize) -> gpio::Gpio {
    unsafe { gpio::Gpio::from_ptr((1073809408 + 1024 * n) as _) }
}
#[path = "../../peripherals/adc_m030.rs"]
pub mod adc;
#[path = "../../peripherals/afio_m030.rs"]
pub mod afio;
#[path = "../../peripherals/dma_m030.rs"]
pub mod dma;
#[path = "../../peripherals/extend_m030.rs"]
pub mod extend;
#[path = "../../peripherals/exti_common.rs"]
pub mod exti;
#[path = "../../peripherals/flash_m030.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_v3.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v0.rs"]
pub mod i2c;
#[path = "../../peripherals/opa_m030.rs"]
pub mod opa;
#[path = "../../peripherals/pfic_m030.rs"]
pub mod pfic;
#[path = "../../peripherals/pwr_m030.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_m030.rs"]
pub mod rcc;
#[path = "../../peripherals/spi_m030.rs"]
pub mod spi;
#[path = "../../peripherals/systick_rv4.rs"]
pub mod systick;
#[path = "../../peripherals/timer_m030.rs"]
pub mod timer;
#[path = "../../peripherals/usart_common.rs"]
pub mod usart;
#[path = "../../peripherals/usb_v2fs.rs"]
pub mod usb;
#[path = "../../peripherals/usbpd_m030.rs"]
pub mod usbpd;
#[path = "../../peripherals/wwdg_v00x.rs"]
pub mod wwdg;
pub const CORE_INDEX: usize = 0;
#[deprecated(note = "use ch32_metapac::MEMORY_LAYOUT instead")]
pub const FLASH_BASE: usize = 0;
#[deprecated(note = "use ch32_metapac::MEMORY_LAYOUT instead")]
pub const FLASH_SIZE: usize = 65536;
#[deprecated(note = "use ch32_metapac::MEMORY_LAYOUT instead")]
pub const WRITE_SIZE: usize = 128;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
