#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "16 - WWDG"]
    WWDG = 16,
    #[doc = "17 - PVD"]
    PVD = 17,
    #[doc = "18 - FLASH"]
    FLASH = 18,
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
    #[doc = "29 - ADC1"]
    ADC1 = 29,
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
    #[doc = "38 - TIM2_UP"]
    TIM2_UP = 38,
    #[doc = "39 - USART2"]
    USART2 = 39,
    #[doc = "40 - EXTI15_8"]
    EXTI15_8 = 40,
    #[doc = "41 - EXTI25_16"]
    EXTI25_16 = 41,
    #[doc = "42 - USART3"]
    USART3 = 42,
    #[doc = "43 - USART4"]
    USART4 = 43,
    #[doc = "44 - DMA1_CHANNEL8"]
    DMA1_CHANNEL8 = 44,
    #[doc = "45 - USBFS"]
    USBFS = 45,
    #[doc = "46 - USBFS_WKUP"]
    USBFS_WKUP = 46,
    #[doc = "47 - PIOC"]
    PIOC = 47,
    #[doc = "48 - OPA"]
    OPA = 48,
    #[doc = "49 - USBPD"]
    USBPD = 49,
    #[doc = "50 - USBPD_WKUP"]
    USBPD_WKUP = 50,
    #[doc = "51 - TIM2_CC"]
    TIM2_CC = 51,
    #[doc = "52 - TIM2_TRG_COM"]
    TIM2_TRG_COM = 52,
    #[doc = "53 - TIM2_BRK"]
    TIM2_BRK = 53,
    #[doc = "54 - TIM3"]
    TIM3 = 54,
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
        fn EXTI7_0();
        fn AWU();
        fn DMA1_CHANNEL1();
        fn DMA1_CHANNEL2();
        fn DMA1_CHANNEL3();
        fn DMA1_CHANNEL4();
        fn DMA1_CHANNEL5();
        fn DMA1_CHANNEL6();
        fn DMA1_CHANNEL7();
        fn ADC1();
        fn I2C1_EV();
        fn I2C1_ER();
        fn USART1();
        fn SPI1();
        fn TIM1_BRK();
        fn TIM1_UP();
        fn TIM1_TRG_COM();
        fn TIM1_CC();
        fn TIM2_UP();
        fn USART2();
        fn EXTI15_8();
        fn EXTI25_16();
        fn USART3();
        fn USART4();
        fn DMA1_CHANNEL8();
        fn USBFS();
        fn USBFS_WKUP();
        fn PIOC();
        fn OPA();
        fn USBPD();
        fn USBPD_WKUP();
        fn TIM2_CC();
        fn TIM2_TRG_COM();
        fn TIM2_BRK();
        fn TIM3();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.external_interrupts"]
    #[no_mangle]
    pub static __EXTERNAL_INTERRUPTS: [Vector; 39] = [
        Vector { _handler: WWDG },
        Vector { _handler: PVD },
        Vector { _handler: FLASH },
        Vector { _reserved: 0 },
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
        Vector { _handler: ADC1 },
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
        Vector { _handler: TIM2_UP },
        Vector { _handler: USART2 },
        Vector { _handler: EXTI15_8 },
        Vector {
            _handler: EXTI25_16,
        },
        Vector { _handler: USART3 },
        Vector { _handler: USART4 },
        Vector {
            _handler: DMA1_CHANNEL8,
        },
        Vector { _handler: USBFS },
        Vector {
            _handler: USBFS_WKUP,
        },
        Vector { _handler: PIOC },
        Vector { _handler: OPA },
        Vector { _handler: USBPD },
        Vector {
            _handler: USBPD_WKUP,
        },
        Vector { _handler: TIM2_CC },
        Vector {
            _handler: TIM2_TRG_COM,
        },
        Vector { _handler: TIM2_BRK },
        Vector { _handler: TIM3 },
    ];
}
pub const TIM2: timer::Adtm = unsafe { timer::Adtm::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0400usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4800usize as _) };
pub const USART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4c00usize as _) };
pub const AFIO: afio::Afio = unsafe { afio::Afio::from_ptr(0x4001_0000usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_0400usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0800usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0c00usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1000usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2400usize as _) };
pub const TIM1: timer::Adtm = unsafe { timer::Adtm::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const DMA1: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_0000usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const USBFS: usb::Usb = unsafe { usb::Usb::from_ptr(0x4002_3400usize as _) };
pub const OPA: opa::Opa = unsafe { opa::Opa::from_ptr(0x4002_6000usize as _) };
pub const AWU: awu::Awu = unsafe { awu::Awu::from_ptr(0x4002_6400usize as _) };
pub const PIOC: pioc::Pioc = unsafe { pioc::Pioc::from_ptr(0x4002_6c00usize as _) };
pub const PFIC: pfic::Pfic = unsafe { pfic::Pfic::from_ptr(0xe000_e000usize as _) };
pub const SYSTICK: systick::Systick = unsafe { systick::Systick::from_ptr(0xe000_f000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub fn GPIO(n: usize) -> gpio::Gpio {
    unsafe { gpio::Gpio::from_ptr((1073809408 + 1024 * n) as _) }
}
#[path = "../../peripherals/adc_x0.rs"]
pub mod adc;
#[path = "../../peripherals/afio_x0.rs"]
pub mod afio;
#[path = "../../peripherals/awu_x0.rs"]
pub mod awu;
#[path = "../../peripherals/dma_v1.rs"]
pub mod dma;
#[path = "../../peripherals/exti_common.rs"]
pub mod exti;
#[path = "../../peripherals/flash_x0.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_x0.rs"]
pub mod gpio;
#[path = "../../peripherals/opa_x0.rs"]
pub mod opa;
#[path = "../../peripherals/pfic_rv4.rs"]
pub mod pfic;
#[path = "../../peripherals/pioc_x0.rs"]
pub mod pioc;
#[path = "../../peripherals/rcc_x0.rs"]
pub mod rcc;
#[path = "../../peripherals/spi_v0.rs"]
pub mod spi;
#[path = "../../peripherals/systick_rv4.rs"]
pub mod systick;
#[path = "../../peripherals/timer_x0.rs"]
pub mod timer;
#[path = "../../peripherals/usart_common.rs"]
pub mod usart;
#[path = "../../peripherals/usb_x0fs.rs"]
pub mod usb;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 0;
pub const FLASH_SIZE: usize = 63488;
pub const WRITE_SIZE: usize = 256;
