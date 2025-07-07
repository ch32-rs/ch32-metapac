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
    #[doc = "57 - RTC_ALARM"]
    RTC_ALARM = 57,
    #[doc = "58 - USB_WKUP"]
    USB_WKUP = 58,
    #[doc = "59 - USBHD"]
    USBHD = 59,
    #[doc = "60 - USBHD_WKUP"]
    USBHD_WKUP = 60,
    #[doc = "61 - ETH"]
    ETH = 61,
    #[doc = "62 - ETH_WKUP"]
    ETH_WKUP = 62,
    #[doc = "63 - BB"]
    BB = 63,
    #[doc = "64 - LLE"]
    LLE = 64,
    #[doc = "65 - TIM5"]
    TIM5 = 65,
    #[doc = "66 - UART4"]
    UART4 = 66,
    #[doc = "67 - DMA1_CHANNEL8"]
    DMA1_CHANNEL8 = 67,
    #[doc = "68 - OSC32KCAL"]
    OSC32KCAL = 68,
    #[doc = "69 - OSC_WKUP"]
    OSC_WKUP = 69,
    #[doc = "83 - OTG_FS"]
    OTG_FS = 83,
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
        fn RTC_ALARM();
        fn USB_WKUP();
        fn USBHD();
        fn USBHD_WKUP();
        fn ETH();
        fn ETH_WKUP();
        fn BB();
        fn LLE();
        fn TIM5();
        fn UART4();
        fn DMA1_CHANNEL8();
        fn OSC32KCAL();
        fn OSC_WKUP();
        fn OTG_FS();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.external_interrupts"]
    #[no_mangle]
    pub static __EXTERNAL_INTERRUPTS: [Vector; 68] = [
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
        Vector {
            _handler: RTC_ALARM,
        },
        Vector { _handler: USB_WKUP },
        Vector { _handler: USBHD },
        Vector {
            _handler: USBHD_WKUP,
        },
        Vector { _handler: ETH },
        Vector { _handler: ETH_WKUP },
        Vector { _handler: BB },
        Vector { _handler: LLE },
        Vector { _handler: TIM5 },
        Vector { _handler: UART4 },
        Vector {
            _handler: DMA1_CHANNEL8,
        },
        Vector {
            _handler: OSC32KCAL,
        },
        Vector { _handler: OSC_WKUP },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: OTG_FS },
    ];
}
pub const TIM2: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0400usize as _) };
pub const TIM4: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0800usize as _) };
pub const TIM5: timer::Gptm32 = unsafe { timer::Gptm32::from_ptr(0x4000_0c00usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_2800usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const USBD: usbd::Usb = unsafe { usbd::Usb::from_ptr(0x4000_5c00usize as _) };
pub const USBRAM: usbram::Usbram = unsafe { usbram::Usbram::from_ptr(0x4000_6000usize as _) };
pub const CAN1: can::Can = unsafe { can::Can::from_ptr(0x4000_6400usize as _) };
pub const AFIO: afio::Afio = unsafe { afio::Afio::from_ptr(0x4001_0000usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_0400usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0800usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0c00usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1000usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1400usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2400usize as _) };
pub const TIM1: timer::Adtm = unsafe { timer::Adtm::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const DMA1: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_0000usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const EXTEND: extend::Extend = unsafe { extend::Extend::from_ptr(0x4002_3800usize as _) };
pub const ETH: eth::Eth = unsafe { eth::Eth::from_ptr(0x4002_8000usize as _) };
pub const USBFS: usb::Usb = unsafe { usb::Usb::from_ptr(0x5000_0000usize as _) };
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
#[path = "../../peripherals/dma_v1.rs"]
pub mod dma;
#[path = "../../peripherals/eth_10m.rs"]
pub mod eth;
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
#[path = "../../peripherals/rcc_v3.rs"]
pub mod rcc;
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
#[path = "../../peripherals/usb_v2fs.rs"]
pub mod usb;
#[path = "../../peripherals/usbd_v2.rs"]
pub mod usbd;
#[path = "../../peripherals/usbram_16x1_512.rs"]
pub mod usbram;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 0;
pub const FLASH_SIZE: usize = 131072;
pub const WRITE_SIZE: usize = 256;
