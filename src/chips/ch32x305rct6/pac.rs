#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "16 - WWDG"]
    WWDG = 16,
    #[doc = "17 - EXTI15_8"]
    EXTI15_8 = 17,
    #[doc = "18 - FLASH"]
    FLASH = 18,
    #[doc = "19 - RCC"]
    RCC = 19,
    #[doc = "20 - EXTI7_0"]
    EXTI7_0 = 20,
    #[doc = "21 - SPI2"]
    SPI2 = 21,
    #[doc = "22 - DMA1_CH1"]
    DMA1_CH1 = 22,
    #[doc = "23 - DMA1_CH2"]
    DMA1_CH2 = 23,
    #[doc = "24 - DMA1_CH3"]
    DMA1_CH3 = 24,
    #[doc = "25 - DMA1_CH4"]
    DMA1_CH4 = 25,
    #[doc = "26 - DMA1_CH5"]
    DMA1_CH5 = 26,
    #[doc = "27 - DMA1_CH6"]
    DMA1_CH6 = 27,
    #[doc = "28 - DMA1_CH7"]
    DMA1_CH7 = 28,
    #[doc = "29 - DMA1_CH8"]
    DMA1_CH8 = 29,
    #[doc = "30 - DMA1_CH9"]
    DMA1_CH9 = 30,
    #[doc = "31 - DMA1_CH10"]
    DMA1_CH10 = 31,
    #[doc = "32 - DMA1_CH11"]
    DMA1_CH11 = 32,
    #[doc = "33 - USART2"]
    USART2 = 33,
    #[doc = "34 - I2C1_EV"]
    I2C1_EV = 34,
    #[doc = "35 - I2C1_ER"]
    I2C1_ER = 35,
    #[doc = "36 - USART1"]
    USART1 = 36,
    #[doc = "37 - SPI1"]
    SPI1 = 37,
    #[doc = "38 - TIM1_UP"]
    TIM1_UP = 38,
    #[doc = "39 - TIM1_CC"]
    TIM1_CC = 39,
    #[doc = "40 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 40,
    #[doc = "41 - TIM1_BRK"]
    TIM1_BRK = 41,
    #[doc = "42 - TIM2"]
    TIM2 = 42,
    #[doc = "43 - I2C2_EV"]
    I2C2_EV = 43,
    #[doc = "44 - I2C2_ER"]
    I2C2_ER = 44,
    #[doc = "45 - USBPD"]
    USBPD = 45,
    #[doc = "46 - USBPD_WKUP"]
    USBPD_WKUP = 46,
    #[doc = "47 - ARGB"]
    ARGB = 47,
    #[doc = "48 - TIM3"]
    TIM3 = 48,
    #[doc = "49 - TIM4"]
    TIM4 = 49,
    #[doc = "50 - SPI3"]
    SPI3 = 50,
    #[doc = "51 - USART3"]
    USART3 = 51,
    #[doc = "52 - USART4"]
    USART4 = 52,
    #[doc = "53 - USBSS"]
    USBSS = 53,
    #[doc = "54 - USBSS_LINK"]
    USBSS_LINK = 54,
    #[doc = "55 - USBHS"]
    USBHS = 55,
    #[doc = "56 - USBHS_WKUP"]
    USBHS_WKUP = 56,
    #[doc = "57 - USBSS_WKUP"]
    USBSS_WKUP = 57,
    #[doc = "58 - PVD"]
    PVD = 58,
    #[doc = "59 - ADC1"]
    ADC1 = 59,
    #[doc = "60 - ADC2"]
    ADC2 = 60,
    #[doc = "61 - ADC3"]
    ADC3 = 61,
    #[doc = "62 - RTCALARM"]
    RTCALARM = 62,
    #[doc = "63 - RTC"]
    RTC = 63,
    #[doc = "64 - USART_WKUP"]
    USART_WKUP = 64,
    #[doc = "65 - ADC4"]
    ADC4 = 65,
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
        fn EXTI15_8();
        fn FLASH();
        fn RCC();
        fn EXTI7_0();
        fn SPI2();
        fn DMA1_CH1();
        fn DMA1_CH2();
        fn DMA1_CH3();
        fn DMA1_CH4();
        fn DMA1_CH5();
        fn DMA1_CH6();
        fn DMA1_CH7();
        fn DMA1_CH8();
        fn DMA1_CH9();
        fn DMA1_CH10();
        fn DMA1_CH11();
        fn USART2();
        fn I2C1_EV();
        fn I2C1_ER();
        fn USART1();
        fn SPI1();
        fn TIM1_UP();
        fn TIM1_CC();
        fn TIM1_TRG_COM();
        fn TIM1_BRK();
        fn TIM2();
        fn I2C2_EV();
        fn I2C2_ER();
        fn USBPD();
        fn USBPD_WKUP();
        fn ARGB();
        fn TIM3();
        fn TIM4();
        fn SPI3();
        fn USART3();
        fn USART4();
        fn USBSS();
        fn USBSS_LINK();
        fn USBHS();
        fn USBHS_WKUP();
        fn USBSS_WKUP();
        fn PVD();
        fn ADC1();
        fn ADC2();
        fn ADC3();
        fn RTCALARM();
        fn RTC();
        fn USART_WKUP();
        fn ADC4();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.external_interrupts")]
    #[unsafe(no_mangle)]
    pub static __EXTERNAL_INTERRUPTS: [Vector; 50] = [
        Vector { _handler: WWDG },
        Vector { _handler: EXTI15_8 },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
        Vector { _handler: EXTI7_0 },
        Vector { _handler: SPI2 },
        Vector { _handler: DMA1_CH1 },
        Vector { _handler: DMA1_CH2 },
        Vector { _handler: DMA1_CH3 },
        Vector { _handler: DMA1_CH4 },
        Vector { _handler: DMA1_CH5 },
        Vector { _handler: DMA1_CH6 },
        Vector { _handler: DMA1_CH7 },
        Vector { _handler: DMA1_CH8 },
        Vector { _handler: DMA1_CH9 },
        Vector {
            _handler: DMA1_CH10,
        },
        Vector {
            _handler: DMA1_CH11,
        },
        Vector { _handler: USART2 },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: USART1 },
        Vector { _handler: SPI1 },
        Vector { _handler: TIM1_UP },
        Vector { _handler: TIM1_CC },
        Vector {
            _handler: TIM1_TRG_COM,
        },
        Vector { _handler: TIM1_BRK },
        Vector { _handler: TIM2 },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ER },
        Vector { _handler: USBPD },
        Vector {
            _handler: USBPD_WKUP,
        },
        Vector { _handler: ARGB },
        Vector { _handler: TIM3 },
        Vector { _handler: TIM4 },
        Vector { _handler: SPI3 },
        Vector { _handler: USART3 },
        Vector { _handler: USART4 },
        Vector { _handler: USBSS },
        Vector {
            _handler: USBSS_LINK,
        },
        Vector { _handler: USBHS },
        Vector {
            _handler: USBHS_WKUP,
        },
        Vector {
            _handler: USBSS_WKUP,
        },
        Vector { _handler: PVD },
        Vector { _handler: ADC1 },
        Vector { _handler: ADC2 },
        Vector { _handler: ADC3 },
        Vector { _handler: RTCALARM },
        Vector { _handler: RTC },
        Vector {
            _handler: USART_WKUP,
        },
        Vector { _handler: ADC4 },
    ];
}
pub const TIM2: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0400usize as _) };
pub const TIM4: timer::Gptm32 = unsafe { timer::Gptm32::from_ptr(0x4000_0800usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4800usize as _) };
pub const USART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4c00usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
pub const AFIO: afio::Afio = unsafe { afio::Afio::from_ptr(0x4001_0000usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_0400usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0800usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0c00usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1000usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1400usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2400usize as _) };
pub const ADC2: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2800usize as _) };
pub const TIM1: timer::Adtm = unsafe { timer::Adtm::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const ADC3: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_3c00usize as _) };
pub const ADC4: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_4000usize as _) };
pub const DMA1: dma::Dma1 = unsafe { dma::Dma1::from_ptr(0x4002_0000usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const EXTEND: extend::Extend = unsafe { extend::Extend::from_ptr(0x4002_20c0usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const USBPD: usbpd::Usbpd = unsafe { usbpd::Usbpd::from_ptr(0x4002_4400usize as _) };
pub const ARGB: argb::Argb = unsafe { argb::Argb::from_ptr(0x4002_5000usize as _) };
pub const USBHS: usbhs::Usb = unsafe { usbhs::Usb::from_ptr(0x4003_0000usize as _) };
pub const PFIC: pfic::Pfic = unsafe { pfic::Pfic::from_ptr(0xe000_e000usize as _) };
pub const SYSTICK: systick::Systick = unsafe { systick::Systick::from_ptr(0xe000_f000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub fn GPIO(n: usize) -> gpio::Gpio {
    unsafe { gpio::Gpio::from_ptr((1073809408 + 1024 * n) as _) }
}
#[path = "../../peripherals/adc_x315.rs"]
pub mod adc;
#[path = "../../peripherals/afio_x315.rs"]
pub mod afio;
#[path = "../../peripherals/argb_x315.rs"]
pub mod argb;
#[path = "../../peripherals/crc_h4.rs"]
pub mod crc;
#[path = "../../peripherals/dma_x315.rs"]
pub mod dma;
#[path = "../../peripherals/extend_x315.rs"]
pub mod extend;
#[path = "../../peripherals/exti_common.rs"]
pub mod exti;
#[path = "../../peripherals/flash_x315.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_h4.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v0.rs"]
pub mod i2c;
#[path = "../../peripherals/iwdg_v00x.rs"]
pub mod iwdg;
#[path = "../../peripherals/pfic_v4x7.rs"]
pub mod pfic;
#[path = "../../peripherals/pwr_x315.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_x315.rs"]
pub mod rcc;
#[path = "../../peripherals/rtc_common.rs"]
pub mod rtc;
#[path = "../../peripherals/spi_v0.rs"]
pub mod spi;
#[path = "../../peripherals/systick_rv4.rs"]
pub mod systick;
#[path = "../../peripherals/timer_x315.rs"]
pub mod timer;
#[path = "../../peripherals/usart_h4.rs"]
pub mod usart;
#[path = "../../peripherals/usbhs_x315.rs"]
pub mod usbhs;
#[path = "../../peripherals/usbpd_x315.rs"]
pub mod usbpd;
#[path = "../../peripherals/wwdg_v00x.rs"]
pub mod wwdg;
pub const CORE_INDEX: usize = 0;
#[deprecated(note = "use ch32_metapac::MEMORY_LAYOUT instead")]
pub const FLASH_BASE: usize = 0;
#[deprecated(note = "use ch32_metapac::MEMORY_LAYOUT instead")]
pub const FLASH_SIZE: usize = 491520;
#[deprecated(note = "use ch32_metapac::MEMORY_LAYOUT instead")]
pub const WRITE_SIZE: usize = 256;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
