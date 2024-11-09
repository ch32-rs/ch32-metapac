#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "16 - WWDG"]
    WWDG = 16,
    #[doc = "17 - PVD"]
    PVD = 17,
    #[doc = "18 - TAMPER"]
    TAMPER = 18,
    #[doc = "19 - RTC"]
    RTC = 19,
    #[doc = "20 - FLASH"]
    FLASH = 20,
    #[doc = "21 - RCC"]
    RCC = 21,
    #[doc = "22 - EXTI0"]
    EXTI0 = 22,
    #[doc = "23 - EXTI1"]
    EXTI1 = 23,
    #[doc = "24 - EXTI2"]
    EXTI2 = 24,
    #[doc = "25 - EXTI3"]
    EXTI3 = 25,
    #[doc = "26 - EXTI4"]
    EXTI4 = 26,
    #[doc = "27 - DMA1_CHANNEL1"]
    DMA1_CHANNEL1 = 27,
    #[doc = "28 - DMA1_CHANNEL2"]
    DMA1_CHANNEL2 = 28,
    #[doc = "29 - DMA1_CHANNEL3"]
    DMA1_CHANNEL3 = 29,
    #[doc = "30 - DMA1_CHANNEL4"]
    DMA1_CHANNEL4 = 30,
    #[doc = "31 - DMA1_CHANNEL5"]
    DMA1_CHANNEL5 = 31,
    #[doc = "32 - DMA1_CHANNEL6"]
    DMA1_CHANNEL6 = 32,
    #[doc = "33 - DMA1_CHANNEL7"]
    DMA1_CHANNEL7 = 33,
    #[doc = "34 - ADC"]
    ADC = 34,
    #[doc = "35 - USB_HP_CAN1_TX"]
    USB_HP_CAN1_TX = 35,
    #[doc = "36 - USB_LP_CAN1_RX0"]
    USB_LP_CAN1_RX0 = 36,
    #[doc = "37 - CAN1_RX1"]
    CAN1_RX1 = 37,
    #[doc = "38 - CAN1_SCE"]
    CAN1_SCE = 38,
    #[doc = "39 - EXTI9_5"]
    EXTI9_5 = 39,
    #[doc = "40 - TIM1_BRK"]
    TIM1_BRK = 40,
    #[doc = "41 - TIM1_UP"]
    TIM1_UP = 41,
    #[doc = "42 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 42,
    #[doc = "43 - TIM1_CC"]
    TIM1_CC = 43,
    #[doc = "44 - TIM2"]
    TIM2 = 44,
    #[doc = "45 - TIM3"]
    TIM3 = 45,
    #[doc = "46 - TIM4"]
    TIM4 = 46,
    #[doc = "47 - I2C1_EV"]
    I2C1_EV = 47,
    #[doc = "48 - I2C1_ER"]
    I2C1_ER = 48,
    #[doc = "49 - I2C2_EV"]
    I2C2_EV = 49,
    #[doc = "50 - I2C2_ER"]
    I2C2_ER = 50,
    #[doc = "51 - SPI1"]
    SPI1 = 51,
    #[doc = "52 - SPI2"]
    SPI2 = 52,
    #[doc = "53 - USART1"]
    USART1 = 53,
    #[doc = "54 - USART2"]
    USART2 = 54,
    #[doc = "55 - USART3"]
    USART3 = 55,
    #[doc = "56 - EXTI15_10"]
    EXTI15_10 = 56,
    #[doc = "57 - RTCALARM"]
    RTCALARM = 57,
    #[doc = "58 - USB_WKUP"]
    USB_WKUP = 58,
    #[doc = "59 - TIM8_BRK"]
    TIM8_BRK = 59,
    #[doc = "60 - TIM8_UP"]
    TIM8_UP = 60,
    #[doc = "61 - TIM8_TRG_COM"]
    TIM8_TRG_COM = 61,
    #[doc = "62 - TIM8_CC"]
    TIM8_CC = 62,
    #[doc = "63 - RNG"]
    RNG = 63,
    #[doc = "65 - SDIO"]
    SDIO = 65,
    #[doc = "66 - TIM5"]
    TIM5 = 66,
    #[doc = "67 - SPI3"]
    SPI3 = 67,
    #[doc = "68 - USART4"]
    USART4 = 68,
    #[doc = "69 - USART5"]
    USART5 = 69,
    #[doc = "70 - TIM6"]
    TIM6 = 70,
    #[doc = "71 - TIM7"]
    TIM7 = 71,
    #[doc = "72 - DMA2_CHANNEL1"]
    DMA2_CHANNEL1 = 72,
    #[doc = "73 - DMA2_CHANNEL2"]
    DMA2_CHANNEL2 = 73,
    #[doc = "74 - DMA2_CHANNEL3"]
    DMA2_CHANNEL3 = 74,
    #[doc = "75 - DMA2_CHANNEL4"]
    DMA2_CHANNEL4 = 75,
    #[doc = "76 - DMA2_CHANNEL5"]
    DMA2_CHANNEL5 = 76,
    #[doc = "77 - ETH"]
    ETH = 77,
    #[doc = "78 - ETH_WKUP"]
    ETH_WKUP = 78,
    #[doc = "79 - CAN2_TX"]
    CAN2_TX = 79,
    #[doc = "80 - CAN2_RX0"]
    CAN2_RX0 = 80,
    #[doc = "81 - CAN2_RX1"]
    CAN2_RX1 = 81,
    #[doc = "82 - CAN2_SCE"]
    CAN2_SCE = 82,
    #[doc = "83 - OTG_FS"]
    OTG_FS = 83,
    #[doc = "84 - USBHS_WKUP"]
    USBHS_WKUP = 84,
    #[doc = "85 - USBHS"]
    USBHS = 85,
    #[doc = "86 - DVP"]
    DVP = 86,
    #[doc = "87 - USART6"]
    USART6 = 87,
    #[doc = "88 - USART7"]
    USART7 = 88,
    #[doc = "89 - USART8"]
    USART8 = 89,
    #[doc = "90 - TIM9_BRK"]
    TIM9_BRK = 90,
    #[doc = "91 - TIM9_UP"]
    TIM9_UP = 91,
    #[doc = "92 - TIM9_TRG_COM"]
    TIM9_TRG_COM = 92,
    #[doc = "93 - TIM9_CC"]
    TIM9_CC = 93,
    #[doc = "94 - TIM10_BRK"]
    TIM10_BRK = 94,
    #[doc = "95 - TIM10_UP"]
    TIM10_UP = 95,
    #[doc = "96 - TIM10_TRG_COM"]
    TIM10_TRG_COM = 96,
    #[doc = "97 - TIM10_CC"]
    TIM10_CC = 97,
    #[doc = "98 - DMA2_CHANNEL6"]
    DMA2_CHANNEL6 = 98,
    #[doc = "99 - DMA2_CHANNEL7"]
    DMA2_CHANNEL7 = 99,
    #[doc = "100 - DMA2_CHANNEL8"]
    DMA2_CHANNEL8 = 100,
    #[doc = "101 - DMA2_CHANNEL9"]
    DMA2_CHANNEL9 = 101,
    #[doc = "102 - DMA2_CHANNEL10"]
    DMA2_CHANNEL10 = 102,
    #[doc = "103 - DMA2_CHANNEL11"]
    DMA2_CHANNEL11 = 103,
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
        fn OTG_FS();
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
    #[link_section = ".vector_table.external_interrupts"]
    #[no_mangle]
    pub static __EXTERNAL_INTERRUPTS: [Vector; 88] = [
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
        Vector { _handler: OTG_FS },
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
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4800usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const CAN2: can::Can = unsafe { can::Can::from_ptr(0x4000_6800usize as _) };
pub const DAC: dac::Dac = unsafe { dac::Dac::from_ptr(0x4000_7400usize as _) };
pub const DAC1: dac::Dac = unsafe { dac::Dac::from_ptr(0x4000_7400usize as _) };
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
pub const TIM8: timer::Adtm = unsafe { timer::Adtm::from_ptr(0x4001_3400usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const TIM9: timer::Adtm = unsafe { timer::Adtm::from_ptr(0x4001_4c00usize as _) };
pub const TIM10: timer::Adtm = unsafe { timer::Adtm::from_ptr(0x4001_5000usize as _) };
pub const DMA1: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_0000usize as _) };
pub const DMA2: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_0400usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const USBHS: usbhs::Usb = unsafe { usbhs::Usb::from_ptr(0x4002_3400usize as _) };
pub const EXTEND: extend::Extend = unsafe { extend::Extend::from_ptr(0x4002_3800usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x4002_3c00usize as _) };
pub const PFIC: pfic::Pfic = unsafe { pfic::Pfic::from_ptr(0xe000_e000usize as _) };
pub const SYSTICK: systick::Systick = unsafe { systick::Systick::from_ptr(0xe000_f000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub fn GPIO(n: usize) -> gpio::Gpio {
    unsafe { gpio::Gpio::from_ptr((1073809408 + 1024 * n) as _) }
}
#[path = "../../peripherals/adc_v3.rs"]
pub mod adc;
#[path = "../../peripherals/afio_v3.rs"]
pub mod afio;
#[path = "../../peripherals/can_v3.rs"]
pub mod can;
#[path = "../../peripherals/dac_v3.rs"]
pub mod dac;
#[path = "../../peripherals/dma_v1.rs"]
pub mod dma;
#[path = "../../peripherals/extend_v3.rs"]
pub mod extend;
#[path = "../../peripherals/exti_common.rs"]
pub mod exti;
#[path = "../../peripherals/flash_v3.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_v3.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_v3.rs"]
pub mod i2c;
#[path = "../../peripherals/pfic_rv4.rs"]
pub mod pfic;
#[path = "../../peripherals/rcc_v3_d8c.rs"]
pub mod rcc;
#[path = "../../peripherals/rng_v3.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_common.rs"]
pub mod rtc;
#[path = "../../peripherals/spi_v3.rs"]
pub mod spi;
#[path = "../../peripherals/systick_rv4.rs"]
pub mod systick;
#[path = "../../peripherals/timer_v3.rs"]
pub mod timer;
#[path = "../../peripherals/usart_common.rs"]
pub mod usart;
#[path = "../../peripherals/usbhs_v3.rs"]
pub mod usbhs;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 0;
pub const FLASH_SIZE: usize = 131072;
pub const WRITE_SIZE: usize = 256;
