#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - WWDG"]
    WWDG = 0,
    #[doc = "1 - PVD"]
    PVD = 1,
    #[doc = "2 - TAMPER"]
    TAMPER = 2,
    #[doc = "3 - RTC"]
    RTC = 3,
    #[doc = "4 - FLASH"]
    FLASH = 4,
    #[doc = "5 - RCC"]
    RCC = 5,
    #[doc = "6 - EXTI0"]
    EXTI0 = 6,
    #[doc = "7 - EXTI1"]
    EXTI1 = 7,
    #[doc = "8 - EXTI2"]
    EXTI2 = 8,
    #[doc = "9 - EXTI3"]
    EXTI3 = 9,
    #[doc = "10 - EXTI4"]
    EXTI4 = 10,
    #[doc = "11 - DMA1_CHANNEL1"]
    DMA1_CHANNEL1 = 11,
    #[doc = "12 - DMA1_CHANNEL2"]
    DMA1_CHANNEL2 = 12,
    #[doc = "13 - DMA1_CHANNEL3"]
    DMA1_CHANNEL3 = 13,
    #[doc = "14 - DMA1_CHANNEL4"]
    DMA1_CHANNEL4 = 14,
    #[doc = "15 - DMA1_CHANNEL5"]
    DMA1_CHANNEL5 = 15,
    #[doc = "16 - DMA1_CHANNEL6"]
    DMA1_CHANNEL6 = 16,
    #[doc = "17 - DMA1_CHANNEL7"]
    DMA1_CHANNEL7 = 17,
    #[doc = "18 - ADC"]
    ADC = 18,
    #[doc = "19 - USB_HP_CAN1_TX"]
    USB_HP_CAN1_TX = 19,
    #[doc = "20 - USB_LP_CAN1_RX0"]
    USB_LP_CAN1_RX0 = 20,
    #[doc = "21 - CAN1_RX1"]
    CAN1_RX1 = 21,
    #[doc = "22 - CAN1_SCE"]
    CAN1_SCE = 22,
    #[doc = "23 - EXTI9_5"]
    EXTI9_5 = 23,
    #[doc = "24 - TIM1_BRK"]
    TIM1_BRK = 24,
    #[doc = "25 - TIM1_UP"]
    TIM1_UP = 25,
    #[doc = "26 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 26,
    #[doc = "27 - TIM1_CC"]
    TIM1_CC = 27,
    #[doc = "28 - TIM2"]
    TIM2 = 28,
    #[doc = "29 - TIM3"]
    TIM3 = 29,
    #[doc = "30 - TIM4"]
    TIM4 = 30,
    #[doc = "31 - I2C1_EV"]
    I2C1_EV = 31,
    #[doc = "32 - I2C1_ER"]
    I2C1_ER = 32,
    #[doc = "33 - I2C2_EV"]
    I2C2_EV = 33,
    #[doc = "34 - I2C2_ER"]
    I2C2_ER = 34,
    #[doc = "35 - SPI1"]
    SPI1 = 35,
    #[doc = "36 - SPI2"]
    SPI2 = 36,
    #[doc = "37 - USART1"]
    USART1 = 37,
    #[doc = "38 - USART2"]
    USART2 = 38,
    #[doc = "39 - USART3"]
    USART3 = 39,
    #[doc = "40 - EXTI15_10"]
    EXTI15_10 = 40,
    #[doc = "41 - RTCALARM"]
    RTCALARM = 41,
    #[doc = "42 - USB_WKUP"]
    USB_WKUP = 42,
    #[doc = "43 - TIM8_BRK"]
    TIM8_BRK = 43,
    #[doc = "44 - TIM8_UP"]
    TIM8_UP = 44,
    #[doc = "45 - TIM8_TRG_COM"]
    TIM8_TRG_COM = 45,
    #[doc = "46 - TIM8_CC"]
    TIM8_CC = 46,
    #[doc = "47 - RNG"]
    RNG = 47,
    #[doc = "49 - SDIO"]
    SDIO = 49,
    #[doc = "50 - TIM5"]
    TIM5 = 50,
    #[doc = "51 - SPI3"]
    SPI3 = 51,
    #[doc = "52 - USART4"]
    USART4 = 52,
    #[doc = "53 - USART5"]
    USART5 = 53,
    #[doc = "54 - TIM6"]
    TIM6 = 54,
    #[doc = "55 - TIM7"]
    TIM7 = 55,
    #[doc = "56 - DMA2_CHANNEL1"]
    DMA2_CHANNEL1 = 56,
    #[doc = "57 - DMA2_CHANNEL2"]
    DMA2_CHANNEL2 = 57,
    #[doc = "58 - DMA2_CHANNEL3"]
    DMA2_CHANNEL3 = 58,
    #[doc = "59 - DMA2_CHANNEL4"]
    DMA2_CHANNEL4 = 59,
    #[doc = "60 - DMA2_CHANNEL5"]
    DMA2_CHANNEL5 = 60,
    #[doc = "61 - ETH"]
    ETH = 61,
    #[doc = "62 - ETH_WKUP"]
    ETH_WKUP = 62,
    #[doc = "63 - CAN2_TX"]
    CAN2_TX = 63,
    #[doc = "64 - CAN2_RX0"]
    CAN2_RX0 = 64,
    #[doc = "65 - CAN2_RX1"]
    CAN2_RX1 = 65,
    #[doc = "66 - CAN2_SCE"]
    CAN2_SCE = 66,
    #[doc = "67 - USBFS"]
    USBFS = 67,
    #[doc = "68 - USBHS_WKUP"]
    USBHS_WKUP = 68,
    #[doc = "69 - USBHS"]
    USBHS = 69,
    #[doc = "70 - DVP"]
    DVP = 70,
    #[doc = "71 - USART6"]
    USART6 = 71,
    #[doc = "72 - USART7"]
    USART7 = 72,
    #[doc = "73 - USART8"]
    USART8 = 73,
    #[doc = "74 - TIM9_BRK"]
    TIM9_BRK = 74,
    #[doc = "75 - TIM9_UP"]
    TIM9_UP = 75,
    #[doc = "76 - TIM9_TRG_COM"]
    TIM9_TRG_COM = 76,
    #[doc = "77 - TIM9_CC"]
    TIM9_CC = 77,
    #[doc = "78 - TIM10_BRK"]
    TIM10_BRK = 78,
    #[doc = "79 - TIM10_UP"]
    TIM10_UP = 79,
    #[doc = "80 - TIM10_TRG_COM"]
    TIM10_TRG_COM = 80,
    #[doc = "81 - TIM10_CC"]
    TIM10_CC = 81,
    #[doc = "82 - DMA2_CHANNEL6"]
    DMA2_CHANNEL6 = 82,
    #[doc = "83 - DMA2_CHANNEL7"]
    DMA2_CHANNEL7 = 83,
    #[doc = "84 - DMA2_CHANNEL8"]
    DMA2_CHANNEL8 = 84,
    #[doc = "85 - DMA2_CHANNEL9"]
    DMA2_CHANNEL9 = 85,
    #[doc = "86 - DMA2_CHANNEL10"]
    DMA2_CHANNEL10 = 86,
    #[doc = "87 - DMA2_CHANNEL11"]
    DMA2_CHANNEL11 = 87,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn WWDG();
        fn PVD();
        fn TAMPER();
        fn RTC();
        fn FLASH();
        fn RCC();
        fn EXTI0();
        fn EXTI1();
        fn EXTI2();
        fn EXTI3();
        fn EXTI4();
        fn DMA1_CHANNEL1();
        fn DMA1_CHANNEL2();
        fn DMA1_CHANNEL3();
        fn DMA1_CHANNEL4();
        fn DMA1_CHANNEL5();
        fn DMA1_CHANNEL6();
        fn DMA1_CHANNEL7();
        fn ADC();
        fn USB_HP_CAN1_TX();
        fn USB_LP_CAN1_RX0();
        fn CAN1_RX1();
        fn CAN1_SCE();
        fn EXTI9_5();
        fn TIM1_BRK();
        fn TIM1_UP();
        fn TIM1_TRG_COM();
        fn TIM1_CC();
        fn TIM2();
        fn TIM3();
        fn TIM4();
        fn I2C1_EV();
        fn I2C1_ER();
        fn I2C2_EV();
        fn I2C2_ER();
        fn SPI1();
        fn SPI2();
        fn USART1();
        fn USART2();
        fn USART3();
        fn EXTI15_10();
        fn RTCALARM();
        fn USB_WKUP();
        fn TIM8_BRK();
        fn TIM8_UP();
        fn TIM8_TRG_COM();
        fn TIM8_CC();
        fn RNG();
        fn SDIO();
        fn TIM5();
        fn SPI3();
        fn USART4();
        fn USART5();
        fn TIM6();
        fn TIM7();
        fn DMA2_CHANNEL1();
        fn DMA2_CHANNEL2();
        fn DMA2_CHANNEL3();
        fn DMA2_CHANNEL4();
        fn DMA2_CHANNEL5();
        fn ETH();
        fn ETH_WKUP();
        fn CAN2_TX();
        fn CAN2_RX0();
        fn CAN2_RX1();
        fn CAN2_SCE();
        fn USBFS();
        fn USBHS_WKUP();
        fn USBHS();
        fn DVP();
        fn USART6();
        fn USART7();
        fn USART8();
        fn TIM9_BRK();
        fn TIM9_UP();
        fn TIM9_TRG_COM();
        fn TIM9_CC();
        fn TIM10_BRK();
        fn TIM10_UP();
        fn TIM10_TRG_COM();
        fn TIM10_CC();
        fn DMA2_CHANNEL6();
        fn DMA2_CHANNEL7();
        fn DMA2_CHANNEL8();
        fn DMA2_CHANNEL9();
        fn DMA2_CHANNEL10();
        fn DMA2_CHANNEL11();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.interrupts")]
    #[unsafe(no_mangle)]
    pub static __INTERRUPTS: [Vector; 88] = [
        Vector { _handler: WWDG },
        Vector { _handler: PVD },
        Vector { _handler: TAMPER },
        Vector { _handler: RTC },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
        Vector { _handler: EXTI0 },
        Vector { _handler: EXTI1 },
        Vector { _handler: EXTI2 },
        Vector { _handler: EXTI3 },
        Vector { _handler: EXTI4 },
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
        Vector {
            _handler: USB_HP_CAN1_TX,
        },
        Vector {
            _handler: USB_LP_CAN1_RX0,
        },
        Vector { _handler: CAN1_RX1 },
        Vector { _handler: CAN1_SCE },
        Vector { _handler: EXTI9_5 },
        Vector { _handler: TIM1_BRK },
        Vector { _handler: TIM1_UP },
        Vector {
            _handler: TIM1_TRG_COM,
        },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
        Vector { _handler: TIM3 },
        Vector { _handler: TIM4 },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ER },
        Vector { _handler: SPI1 },
        Vector { _handler: SPI2 },
        Vector { _handler: USART1 },
        Vector { _handler: USART2 },
        Vector { _handler: USART3 },
        Vector {
            _handler: EXTI15_10,
        },
        Vector { _handler: RTCALARM },
        Vector { _handler: USB_WKUP },
        Vector { _handler: TIM8_BRK },
        Vector { _handler: TIM8_UP },
        Vector {
            _handler: TIM8_TRG_COM,
        },
        Vector { _handler: TIM8_CC },
        Vector { _handler: RNG },
        Vector { _reserved: 0 },
        Vector { _handler: SDIO },
        Vector { _handler: TIM5 },
        Vector { _handler: SPI3 },
        Vector { _handler: USART4 },
        Vector { _handler: USART5 },
        Vector { _handler: TIM6 },
        Vector { _handler: TIM7 },
        Vector {
            _handler: DMA2_CHANNEL1,
        },
        Vector {
            _handler: DMA2_CHANNEL2,
        },
        Vector {
            _handler: DMA2_CHANNEL3,
        },
        Vector {
            _handler: DMA2_CHANNEL4,
        },
        Vector {
            _handler: DMA2_CHANNEL5,
        },
        Vector { _handler: ETH },
        Vector { _handler: ETH_WKUP },
        Vector { _handler: CAN2_TX },
        Vector { _handler: CAN2_RX0 },
        Vector { _handler: CAN2_RX1 },
        Vector { _handler: CAN2_SCE },
        Vector { _handler: USBFS },
        Vector {
            _handler: USBHS_WKUP,
        },
        Vector { _handler: USBHS },
        Vector { _handler: DVP },
        Vector { _handler: USART6 },
        Vector { _handler: USART7 },
        Vector { _handler: USART8 },
        Vector { _handler: TIM9_BRK },
        Vector { _handler: TIM9_UP },
        Vector {
            _handler: TIM9_TRG_COM,
        },
        Vector { _handler: TIM9_CC },
        Vector {
            _handler: TIM10_BRK,
        },
        Vector { _handler: TIM10_UP },
        Vector {
            _handler: TIM10_TRG_COM,
        },
        Vector { _handler: TIM10_CC },
        Vector {
            _handler: DMA2_CHANNEL6,
        },
        Vector {
            _handler: DMA2_CHANNEL7,
        },
        Vector {
            _handler: DMA2_CHANNEL8,
        },
        Vector {
            _handler: DMA2_CHANNEL9,
        },
        Vector {
            _handler: DMA2_CHANNEL10,
        },
        Vector {
            _handler: DMA2_CHANNEL11,
        },
    ];
}
pub const TIM2: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0400usize as _) };
pub const TIM4: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0800usize as _) };
pub const TIM5: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0c00usize as _) };
pub const TIM6: timer::Bctm = unsafe { timer::Bctm::from_ptr(0x4000_1000usize as _) };
pub const TIM7: timer::Bctm = unsafe { timer::Bctm::from_ptr(0x4000_1400usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4800usize as _) };
pub const USART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4c00usize as _) };
pub const USART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_5000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const CAN1: can::Can = unsafe { can::Can::from_ptr(0x4000_6400usize as _) };
pub const CAN2: can::Can = unsafe { can::Can::from_ptr(0x4000_6800usize as _) };
pub const DAC: dac::Dac = unsafe { dac::Dac::from_ptr(0x4000_7400usize as _) };
pub const AFIO: afio::Afio = unsafe { afio::Afio::from_ptr(0x4001_0000usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_0400usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0800usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0c00usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1000usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1400usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1800usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2400usize as _) };
pub const ADC2: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2800usize as _) };
pub const TIM1: timer::Adtm = unsafe { timer::Adtm::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const TIM8: timer::Adtm = unsafe { timer::Adtm::from_ptr(0x4001_3400usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const TIM9: timer::Adtm = unsafe { timer::Adtm::from_ptr(0x4001_4c00usize as _) };
pub const TIM10: timer::Adtm = unsafe { timer::Adtm::from_ptr(0x4001_5000usize as _) };
pub const SDIO: sdio::Sdio = unsafe { sdio::Sdio::from_ptr(0x4001_8000usize as _) };
pub const DMA1: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_0000usize as _) };
pub const DMA2: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_0400usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const USBHS: usbhs::Usb = unsafe { usbhs::Usb::from_ptr(0x4002_3400usize as _) };
pub const EXTEND: extend::Extend = unsafe { extend::Extend::from_ptr(0x4002_3800usize as _) };
pub const OPA: opa::Opa = unsafe { opa::Opa::from_ptr(0x4002_3804usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x4002_3c00usize as _) };
pub const USBFS: otg::Usb = unsafe { otg::Usb::from_ptr(0x5000_0000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub fn GPIO(n: usize) -> gpio::Gpio {
    unsafe { gpio::Gpio::from_ptr((1073809408 + 1024 * n) as _) }
}
#[path = "../../peripherals/adc_f2.rs"]
pub mod adc;
#[path = "../../peripherals/afio_v3.rs"]
pub mod afio;
#[path = "../../peripherals/can_v3.rs"]
pub mod can;
#[path = "../../peripherals/crc_h4.rs"]
pub mod crc;
#[path = "../../peripherals/dac_v3.rs"]
pub mod dac;
#[path = "../../peripherals/dma_v1.rs"]
pub mod dma;
#[path = "../../peripherals/extend_f2.rs"]
pub mod extend;
#[path = "../../peripherals/exti_common.rs"]
pub mod exti;
#[path = "../../peripherals/flash_v3.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_v3.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v3.rs"]
pub mod i2c;
#[path = "../../peripherals/iwdg_v00x.rs"]
pub mod iwdg;
#[path = "../../peripherals/opa_v3.rs"]
pub mod opa;
#[path = "../../peripherals/otg_v2.rs"]
pub mod otg;
#[path = "../../peripherals/rcc_v3_d8c.rs"]
pub mod rcc;
#[path = "../../peripherals/rng_v3.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_common.rs"]
pub mod rtc;
#[path = "../../peripherals/sdio_v3.rs"]
pub mod sdio;
#[path = "../../peripherals/spi_v3.rs"]
pub mod spi;
#[path = "../../peripherals/timer_v3.rs"]
pub mod timer;
#[path = "../../peripherals/usart_common.rs"]
pub mod usart;
#[path = "../../peripherals/usbhs_v3.rs"]
pub mod usbhs;
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
