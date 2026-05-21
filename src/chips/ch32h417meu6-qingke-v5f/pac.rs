#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "16 - IPC_CH0"]
    IPC_CH0 = 16,
    #[doc = "17 - IPC_CH1"]
    IPC_CH1 = 17,
    #[doc = "18 - IPC_CH2"]
    IPC_CH2 = 18,
    #[doc = "19 - IPC_CH3"]
    IPC_CH3 = 19,
    #[doc = "28 - HSEM"]
    HSEM = 28,
    #[doc = "32 - WWDG"]
    WWDG = 32,
    #[doc = "33 - EXTI15_8"]
    EXTI15_8 = 33,
    #[doc = "34 - FLASH"]
    FLASH = 34,
    #[doc = "35 - RCC"]
    RCC = 35,
    #[doc = "36 - EXTI7_0"]
    EXTI7_0 = 36,
    #[doc = "37 - SPI1"]
    SPI1 = 37,
    #[doc = "38 - DMA1_CH2"]
    DMA1_CH2 = 38,
    #[doc = "39 - DMA1_CH3"]
    DMA1_CH3 = 39,
    #[doc = "40 - DMA1_CH4"]
    DMA1_CH4 = 40,
    #[doc = "41 - DMA1_CH5"]
    DMA1_CH5 = 41,
    #[doc = "42 - DMA1_CH6"]
    DMA1_CH6 = 42,
    #[doc = "43 - DMA1_CH7"]
    DMA1_CH7 = 43,
    #[doc = "44 - DMA1_CH8"]
    DMA1_CH8 = 44,
    #[doc = "45 - USART2"]
    USART2 = 45,
    #[doc = "46 - I2C1_EV"]
    I2C1_EV = 46,
    #[doc = "47 - I2C1_ER"]
    I2C1_ER = 47,
    #[doc = "48 - USART1"]
    USART1 = 48,
    #[doc = "49 - SPI2"]
    SPI2 = 49,
    #[doc = "50 - SPI3"]
    SPI3 = 50,
    #[doc = "51 - SPI4"]
    SPI4 = 51,
    #[doc = "52 - I2C2_EV"]
    I2C2_EV = 52,
    #[doc = "53 - I2C2_ER"]
    I2C2_ER = 53,
    #[doc = "54 - USBPD"]
    USBPD = 54,
    #[doc = "55 - USBPDWAKEUP"]
    USBPDWAKEUP = 55,
    #[doc = "56 - USBHS"]
    USBHS = 56,
    #[doc = "57 - DMA1_CH1"]
    DMA1_CH1 = 57,
    #[doc = "58 - CAN1_SCE"]
    CAN1_SCE = 58,
    #[doc = "59 - CAN1_TX"]
    CAN1_TX = 59,
    #[doc = "60 - CAN1_RX0"]
    CAN1_RX0 = 60,
    #[doc = "61 - CAN1_RX1"]
    CAN1_RX1 = 61,
    #[doc = "62 - USBSS"]
    USBSS = 62,
    #[doc = "63 - USBSS_LINK"]
    USBSS_LINK = 63,
    #[doc = "64 - USBHSWAKEUP"]
    USBHSWAKEUP = 64,
    #[doc = "65 - USBSSWAKEUP"]
    USBSSWAKEUP = 65,
    #[doc = "66 - RTCALARM"]
    RTCALARM = 66,
    #[doc = "67 - USBFS"]
    USBFS = 67,
    #[doc = "68 - USBFSWAKEUP"]
    USBFSWAKEUP = 68,
    #[doc = "69 - ADC1_2"]
    ADC1_2 = 69,
    #[doc = "70 - TIM1_BRK"]
    TIM1_BRK = 70,
    #[doc = "71 - TIM1_UP"]
    TIM1_UP = 71,
    #[doc = "72 - TIM1_TRG_COM"]
    TIM1_TRG_COM = 72,
    #[doc = "73 - TIM1_CC"]
    TIM1_CC = 73,
    #[doc = "74 - TIM2"]
    TIM2 = 74,
    #[doc = "75 - TIM3"]
    TIM3 = 75,
    #[doc = "76 - TIM4"]
    TIM4 = 76,
    #[doc = "77 - TIM5"]
    TIM5 = 77,
    #[doc = "78 - I2C3_EV"]
    I2C3_EV = 78,
    #[doc = "79 - I2C3_ER"]
    I2C3_ER = 79,
    #[doc = "80 - I2C4_EV"]
    I2C4_EV = 80,
    #[doc = "81 - I2C4_ER"]
    I2C4_ER = 81,
    #[doc = "82 - QSPI1"]
    QSPI1 = 82,
    #[doc = "83 - SERDES"]
    SERDES = 83,
    #[doc = "84 - USART3"]
    USART3 = 84,
    #[doc = "85 - USART4"]
    USART4 = 85,
    #[doc = "86 - TIM8_BRK"]
    TIM8_BRK = 86,
    #[doc = "87 - TIM8_UP"]
    TIM8_UP = 87,
    #[doc = "88 - TIM8_TRG_COM"]
    TIM8_TRG_COM = 88,
    #[doc = "89 - TIM8_CC"]
    TIM8_CC = 89,
    #[doc = "90 - TIM9"]
    TIM9 = 90,
    #[doc = "91 - TIM10"]
    TIM10 = 91,
    #[doc = "92 - TIM11"]
    TIM11 = 92,
    #[doc = "93 - TIM12"]
    TIM12 = 93,
    #[doc = "94 - FMC"]
    FMC = 94,
    #[doc = "95 - SDMMC"]
    SDMMC = 95,
    #[doc = "96 - LPTIM1"]
    LPTIM1 = 96,
    #[doc = "97 - LPTIM2"]
    LPTIM2 = 97,
    #[doc = "98 - USART5"]
    USART5 = 98,
    #[doc = "99 - USART6"]
    USART6 = 99,
    #[doc = "100 - TIM6"]
    TIM6 = 100,
    #[doc = "101 - TIM7"]
    TIM7 = 101,
    #[doc = "102 - DMA2_CH1"]
    DMA2_CH1 = 102,
    #[doc = "103 - DMA2_CH2"]
    DMA2_CH2 = 103,
    #[doc = "104 - DMA2_CH3"]
    DMA2_CH3 = 104,
    #[doc = "105 - DMA2_CH4"]
    DMA2_CH4 = 105,
    #[doc = "106 - DMA2_CH5"]
    DMA2_CH5 = 106,
    #[doc = "107 - DMA2_CH6"]
    DMA2_CH6 = 107,
    #[doc = "108 - DMA2_CH7"]
    DMA2_CH7 = 108,
    #[doc = "109 - DMA2_CH8"]
    DMA2_CH8 = 109,
    #[doc = "110 - ETH"]
    ETH = 110,
    #[doc = "111 - ETH_WKUP"]
    ETH_WKUP = 111,
    #[doc = "112 - CAN2_SCE"]
    CAN2_SCE = 112,
    #[doc = "113 - CAN2_TX"]
    CAN2_TX = 113,
    #[doc = "114 - CAN2_RX0"]
    CAN2_RX0 = 114,
    #[doc = "115 - CAN2_RX1"]
    CAN2_RX1 = 115,
    #[doc = "116 - USART7"]
    USART7 = 116,
    #[doc = "117 - USART8"]
    USART8 = 117,
    #[doc = "118 - I3C_EV"]
    I3C_EV = 118,
    #[doc = "119 - I3C_ER"]
    I3C_ER = 119,
    #[doc = "120 - DVP"]
    DVP = 120,
    #[doc = "121 - ECDC"]
    ECDC = 121,
    #[doc = "122 - PIOC"]
    PIOC = 122,
    #[doc = "123 - SAI"]
    SAI = 123,
    #[doc = "124 - LTDC"]
    LTDC = 124,
    #[doc = "125 - GPHA"]
    GPHA = 125,
    #[doc = "127 - DFSDM0"]
    DFSDM0 = 127,
    #[doc = "128 - DFSDM1"]
    DFSDM1 = 128,
    #[doc = "131 - SWPMI"]
    SWPMI = 131,
    #[doc = "134 - QSPI2"]
    QSPI2 = 134,
    #[doc = "135 - SWPMIWAKEUP"]
    SWPMIWAKEUP = 135,
    #[doc = "136 - CAN3_SCE"]
    CAN3_SCE = 136,
    #[doc = "137 - CAN3_TX"]
    CAN3_TX = 137,
    #[doc = "138 - CAN3_RX0"]
    CAN3_RX0 = 138,
    #[doc = "139 - CAN3_RX1"]
    CAN3_RX1 = 139,
    #[doc = "140 - LPTIM2WAKEUP"]
    LPTIM2WAKEUP = 140,
    #[doc = "141 - LPTIM1WAKEUP"]
    LPTIM1WAKEUP = 141,
    #[doc = "142 - I3CWAKEUP"]
    I3CWAKEUP = 142,
    #[doc = "143 - RTC"]
    RTC = 143,
    #[doc = "144 - HSADC"]
    HSADC = 144,
    #[doc = "145 - UHSIF"]
    UHSIF = 145,
    #[doc = "146 - RNG"]
    RNG = 146,
    #[doc = "147 - SDIO"]
    SDIO = 147,
    #[doc = "148 - USARTWAKEUP"]
    USARTWAKEUP = 148,
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
        fn IPC_CH0();
        fn IPC_CH1();
        fn IPC_CH2();
        fn IPC_CH3();
        fn HSEM();
        fn WWDG();
        fn EXTI15_8();
        fn FLASH();
        fn RCC();
        fn EXTI7_0();
        fn SPI1();
        fn DMA1_CH2();
        fn DMA1_CH3();
        fn DMA1_CH4();
        fn DMA1_CH5();
        fn DMA1_CH6();
        fn DMA1_CH7();
        fn DMA1_CH8();
        fn USART2();
        fn I2C1_EV();
        fn I2C1_ER();
        fn USART1();
        fn SPI2();
        fn SPI3();
        fn SPI4();
        fn I2C2_EV();
        fn I2C2_ER();
        fn USBPD();
        fn USBPDWAKEUP();
        fn USBHS();
        fn DMA1_CH1();
        fn CAN1_SCE();
        fn CAN1_TX();
        fn CAN1_RX0();
        fn CAN1_RX1();
        fn USBSS();
        fn USBSS_LINK();
        fn USBHSWAKEUP();
        fn USBSSWAKEUP();
        fn RTCALARM();
        fn USBFS();
        fn USBFSWAKEUP();
        fn ADC1_2();
        fn TIM1_BRK();
        fn TIM1_UP();
        fn TIM1_TRG_COM();
        fn TIM1_CC();
        fn TIM2();
        fn TIM3();
        fn TIM4();
        fn TIM5();
        fn I2C3_EV();
        fn I2C3_ER();
        fn I2C4_EV();
        fn I2C4_ER();
        fn QSPI1();
        fn SERDES();
        fn USART3();
        fn USART4();
        fn TIM8_BRK();
        fn TIM8_UP();
        fn TIM8_TRG_COM();
        fn TIM8_CC();
        fn TIM9();
        fn TIM10();
        fn TIM11();
        fn TIM12();
        fn FMC();
        fn SDMMC();
        fn LPTIM1();
        fn LPTIM2();
        fn USART5();
        fn USART6();
        fn TIM6();
        fn TIM7();
        fn DMA2_CH1();
        fn DMA2_CH2();
        fn DMA2_CH3();
        fn DMA2_CH4();
        fn DMA2_CH5();
        fn DMA2_CH6();
        fn DMA2_CH7();
        fn DMA2_CH8();
        fn ETH();
        fn ETH_WKUP();
        fn CAN2_SCE();
        fn CAN2_TX();
        fn CAN2_RX0();
        fn CAN2_RX1();
        fn USART7();
        fn USART8();
        fn I3C_EV();
        fn I3C_ER();
        fn DVP();
        fn ECDC();
        fn PIOC();
        fn SAI();
        fn LTDC();
        fn GPHA();
        fn DFSDM0();
        fn DFSDM1();
        fn SWPMI();
        fn QSPI2();
        fn SWPMIWAKEUP();
        fn CAN3_SCE();
        fn CAN3_TX();
        fn CAN3_RX0();
        fn CAN3_RX1();
        fn LPTIM2WAKEUP();
        fn LPTIM1WAKEUP();
        fn I3CWAKEUP();
        fn RTC();
        fn HSADC();
        fn UHSIF();
        fn RNG();
        fn SDIO();
        fn USARTWAKEUP();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.external_interrupts"]
    #[no_mangle]
    pub static __EXTERNAL_INTERRUPTS: [Vector; 133] = [
        Vector { _handler: IPC_CH0 },
        Vector { _handler: IPC_CH1 },
        Vector { _handler: IPC_CH2 },
        Vector { _handler: IPC_CH3 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: HSEM },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: WWDG },
        Vector { _handler: EXTI15_8 },
        Vector { _handler: FLASH },
        Vector { _handler: RCC },
        Vector { _handler: EXTI7_0 },
        Vector { _handler: SPI1 },
        Vector { _handler: DMA1_CH2 },
        Vector { _handler: DMA1_CH3 },
        Vector { _handler: DMA1_CH4 },
        Vector { _handler: DMA1_CH5 },
        Vector { _handler: DMA1_CH6 },
        Vector { _handler: DMA1_CH7 },
        Vector { _handler: DMA1_CH8 },
        Vector { _handler: USART2 },
        Vector { _handler: I2C1_EV },
        Vector { _handler: I2C1_ER },
        Vector { _handler: USART1 },
        Vector { _handler: SPI2 },
        Vector { _handler: SPI3 },
        Vector { _handler: SPI4 },
        Vector { _handler: I2C2_EV },
        Vector { _handler: I2C2_ER },
        Vector { _handler: USBPD },
        Vector {
            _handler: USBPDWAKEUP,
        },
        Vector { _handler: USBHS },
        Vector { _handler: DMA1_CH1 },
        Vector { _handler: CAN1_SCE },
        Vector { _handler: CAN1_TX },
        Vector { _handler: CAN1_RX0 },
        Vector { _handler: CAN1_RX1 },
        Vector { _handler: USBSS },
        Vector {
            _handler: USBSS_LINK,
        },
        Vector {
            _handler: USBHSWAKEUP,
        },
        Vector {
            _handler: USBSSWAKEUP,
        },
        Vector { _handler: RTCALARM },
        Vector { _handler: USBFS },
        Vector {
            _handler: USBFSWAKEUP,
        },
        Vector { _handler: ADC1_2 },
        Vector { _handler: TIM1_BRK },
        Vector { _handler: TIM1_UP },
        Vector {
            _handler: TIM1_TRG_COM,
        },
        Vector { _handler: TIM1_CC },
        Vector { _handler: TIM2 },
        Vector { _handler: TIM3 },
        Vector { _handler: TIM4 },
        Vector { _handler: TIM5 },
        Vector { _handler: I2C3_EV },
        Vector { _handler: I2C3_ER },
        Vector { _handler: I2C4_EV },
        Vector { _handler: I2C4_ER },
        Vector { _handler: QSPI1 },
        Vector { _handler: SERDES },
        Vector { _handler: USART3 },
        Vector { _handler: USART4 },
        Vector { _handler: TIM8_BRK },
        Vector { _handler: TIM8_UP },
        Vector {
            _handler: TIM8_TRG_COM,
        },
        Vector { _handler: TIM8_CC },
        Vector { _handler: TIM9 },
        Vector { _handler: TIM10 },
        Vector { _handler: TIM11 },
        Vector { _handler: TIM12 },
        Vector { _handler: FMC },
        Vector { _handler: SDMMC },
        Vector { _handler: LPTIM1 },
        Vector { _handler: LPTIM2 },
        Vector { _handler: USART5 },
        Vector { _handler: USART6 },
        Vector { _handler: TIM6 },
        Vector { _handler: TIM7 },
        Vector { _handler: DMA2_CH1 },
        Vector { _handler: DMA2_CH2 },
        Vector { _handler: DMA2_CH3 },
        Vector { _handler: DMA2_CH4 },
        Vector { _handler: DMA2_CH5 },
        Vector { _handler: DMA2_CH6 },
        Vector { _handler: DMA2_CH7 },
        Vector { _handler: DMA2_CH8 },
        Vector { _handler: ETH },
        Vector { _handler: ETH_WKUP },
        Vector { _handler: CAN2_SCE },
        Vector { _handler: CAN2_TX },
        Vector { _handler: CAN2_RX0 },
        Vector { _handler: CAN2_RX1 },
        Vector { _handler: USART7 },
        Vector { _handler: USART8 },
        Vector { _handler: I3C_EV },
        Vector { _handler: I3C_ER },
        Vector { _handler: DVP },
        Vector { _handler: ECDC },
        Vector { _handler: PIOC },
        Vector { _handler: SAI },
        Vector { _handler: LTDC },
        Vector { _handler: GPHA },
        Vector { _reserved: 0 },
        Vector { _handler: DFSDM0 },
        Vector { _handler: DFSDM1 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: SWPMI },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: QSPI2 },
        Vector {
            _handler: SWPMIWAKEUP,
        },
        Vector { _handler: CAN3_SCE },
        Vector { _handler: CAN3_TX },
        Vector { _handler: CAN3_RX0 },
        Vector { _handler: CAN3_RX1 },
        Vector {
            _handler: LPTIM2WAKEUP,
        },
        Vector {
            _handler: LPTIM1WAKEUP,
        },
        Vector {
            _handler: I3CWAKEUP,
        },
        Vector { _handler: RTC },
        Vector { _handler: HSADC },
        Vector { _handler: UHSIF },
        Vector { _handler: RNG },
        Vector { _handler: SDIO },
        Vector {
            _handler: USARTWAKEUP,
        },
    ];
}
pub const ESIG: esig::Esig = unsafe { esig::Esig::from_ptr(0x1fff_f7e0usize as _) };
pub const TIM2: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0000usize as _) };
pub const TIM3: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0400usize as _) };
pub const TIM4: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0800usize as _) };
pub const TIM5: timer::Gptm = unsafe { timer::Gptm::from_ptr(0x4000_0c00usize as _) };
pub const TIM6: timer::Bctm = unsafe { timer::Bctm::from_ptr(0x4000_1000usize as _) };
pub const TIM7: timer::Bctm = unsafe { timer::Bctm::from_ptr(0x4000_1400usize as _) };
pub const USART6: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_1800usize as _) };
pub const USART7: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_1c00usize as _) };
pub const USART8: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_2000usize as _) };
pub const LPTIM1: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_2400usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_2800usize as _) };
pub const WWDG: wwdg::Wwdg = unsafe { wwdg::Wwdg::from_ptr(0x4000_2c00usize as _) };
pub const IWDG: iwdg::Iwdg = unsafe { iwdg::Iwdg::from_ptr(0x4000_3000usize as _) };
pub const LPTIM2: lptim::Lptim = unsafe { lptim::Lptim::from_ptr(0x4000_3400usize as _) };
pub const SPI2: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3800usize as _) };
pub const SPI3: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_3c00usize as _) };
pub const SPI4: spi::Spi = unsafe { spi::Spi::from_ptr(0x4000_4000usize as _) };
pub const USART2: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4400usize as _) };
pub const USART3: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4800usize as _) };
pub const USART4: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_4c00usize as _) };
pub const USART5: usart::Usart = unsafe { usart::Usart::from_ptr(0x4000_5000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5400usize as _) };
pub const I2C2: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5800usize as _) };
pub const I2C3: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_5c00usize as _) };
pub const CAN1: can::Can = unsafe { can::Can::from_ptr(0x4000_6400usize as _) };
pub const CAN2: can::Can = unsafe { can::Can::from_ptr(0x4000_6800usize as _) };
pub const PWR: pwr::Pwr = unsafe { pwr::Pwr::from_ptr(0x4000_7000usize as _) };
pub const DAC: dac::Dac = unsafe { dac::Dac::from_ptr(0x4000_7400usize as _) };
pub const CAN3: can::Can = unsafe { can::Can::from_ptr(0x4000_7800usize as _) };
pub const AFIO: afio::Afio = unsafe { afio::Afio::from_ptr(0x4001_0000usize as _) };
pub const EXTI: exti::Exti = unsafe { exti::Exti::from_ptr(0x4001_0400usize as _) };
pub const GPIOA: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0800usize as _) };
pub const GPIOB: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0c00usize as _) };
pub const GPIOC: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1000usize as _) };
pub const GPIOD: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1400usize as _) };
pub const GPIOE: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1800usize as _) };
pub const GPIOF: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_1c00usize as _) };
pub const ADC1: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2400usize as _) };
pub const ADC2: adc::Adc = unsafe { adc::Adc::from_ptr(0x4001_2800usize as _) };
pub const TIM1: timer::Adtm = unsafe { timer::Adtm::from_ptr(0x4001_2c00usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4001_3000usize as _) };
pub const TIM8: timer::Adtm = unsafe { timer::Adtm::from_ptr(0x4001_3400usize as _) };
pub const USART1: usart::Usart = unsafe { usart::Usart::from_ptr(0x4001_3800usize as _) };
pub const TIM12: timer::Gptm32 = unsafe { timer::Gptm32::from_ptr(0x4001_3c00usize as _) };
pub const I2C4: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4001_4000usize as _) };
pub const TIM9: timer::Gptm32 = unsafe { timer::Gptm32::from_ptr(0x4001_4c00usize as _) };
pub const TIM10: timer::Gptm32 = unsafe { timer::Gptm32::from_ptr(0x4001_5000usize as _) };
pub const TIM11: timer::Gptm32 = unsafe { timer::Gptm32::from_ptr(0x4001_5400usize as _) };
pub const DFSDM: dfsdm::Dfsdm = unsafe { dfsdm::Dfsdm::from_ptr(0x4001_7000usize as _) };
pub const HSADC: hsadc::Hsadc = unsafe { hsadc::Hsadc::from_ptr(0x4001_7400usize as _) };
pub const OPA: opa::Opa = unsafe { opa::Opa::from_ptr(0x4001_7800usize as _) };
pub const DMA1: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_0000usize as _) };
pub const DMA2: dma::Dma = unsafe { dma::Dma::from_ptr(0x4002_0400usize as _) };
pub const DMAMUX: dmamux::Dmamux = unsafe { dmamux::Dmamux::from_ptr(0x4002_0800usize as _) };
pub const RCC: rcc::Rcc = unsafe { rcc::Rcc::from_ptr(0x4002_1000usize as _) };
pub const FLASH: flash::Flash = unsafe { flash::Flash::from_ptr(0x4002_2000usize as _) };
pub const CRC: crc::Crc = unsafe { crc::Crc::from_ptr(0x4002_3000usize as _) };
pub const RNG: rng::Rng = unsafe { rng::Rng::from_ptr(0x4002_3c00usize as _) };
pub const USBPD: usbpd::Usbpd = unsafe { usbpd::Usbpd::from_ptr(0x4002_4400usize as _) };
pub const USBHS: usbhs::Usbhs = unsafe { usbhs::Usbhs::from_ptr(0x4003_0000usize as _) };
pub const USBSS: usbss::Usbss = unsafe { usbss::Usbss::from_ptr(0x4003_4000usize as _) };
pub const USBFS: usbfs::UsbOtgFs = unsafe { usbfs::UsbOtgFs::from_ptr(0x5000_0000usize as _) };
pub const HSEM: hsem::Hsem = unsafe { hsem::Hsem::from_ptr(0xe000_c000usize as _) };
pub const IPC: ipc::Ipc = unsafe { ipc::Ipc::from_ptr(0xe000_d000usize as _) };
pub const PFIC: pfic::Pfic = unsafe { pfic::Pfic::from_ptr(0xe000_e000usize as _) };
pub const SYSTICK: systick::Systick = unsafe { systick::Systick::from_ptr(0xe000_f000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub fn GPIO(n: usize) -> gpio::Gpio {
    unsafe { gpio::Gpio::from_ptr((1073809408 + 1024 * n) as _) }
}
#[path = "../../peripherals/adc_h4.rs"]
pub mod adc;
#[path = "../../peripherals/afio_h4.rs"]
pub mod afio;
#[path = "../../peripherals/can_h4.rs"]
pub mod can;
#[path = "../../peripherals/crc_h4.rs"]
pub mod crc;
#[path = "../../peripherals/dac_h4.rs"]
pub mod dac;
#[path = "../../peripherals/dfsdm_h4.rs"]
pub mod dfsdm;
#[path = "../../peripherals/dma_h4.rs"]
pub mod dma;
#[path = "../../peripherals/dmamux_h4.rs"]
pub mod dmamux;
#[path = "../../peripherals/esig_h4.rs"]
pub mod esig;
#[path = "../../peripherals/exti_common.rs"]
pub mod exti;
#[path = "../../peripherals/flash_h4.rs"]
pub mod flash;
#[path = "../../peripherals/gpio_v3.rs"]
pub mod gpio;
#[path = "../../peripherals/hsadc_h4.rs"]
pub mod hsadc;
#[path = "../../peripherals/hsem_h4.rs"]
pub mod hsem;
#[path = "../../peripherals/i2c_v3.rs"]
pub mod i2c;
#[path = "../../peripherals/ipc_h4.rs"]
pub mod ipc;
#[path = "../../peripherals/iwdg_h4.rs"]
pub mod iwdg;
#[path = "../../peripherals/lptim_l1.rs"]
pub mod lptim;
#[path = "../../peripherals/opa_h4.rs"]
pub mod opa;
#[path = "../../peripherals/pfic_h4.rs"]
pub mod pfic;
#[path = "../../peripherals/pwr_h4.rs"]
pub mod pwr;
#[path = "../../peripherals/rcc_h4.rs"]
pub mod rcc;
#[path = "../../peripherals/rng_h4.rs"]
pub mod rng;
#[path = "../../peripherals/rtc_common.rs"]
pub mod rtc;
#[path = "../../peripherals/spi_h4.rs"]
pub mod spi;
#[path = "../../peripherals/systick_v3f_v5f.rs"]
pub mod systick;
#[path = "../../peripherals/timer_h4.rs"]
pub mod timer;
#[path = "../../peripherals/usart_h4.rs"]
pub mod usart;
#[path = "../../peripherals/usbfs_h4.rs"]
pub mod usbfs;
#[path = "../../peripherals/usbhs_h4.rs"]
pub mod usbhs;
#[path = "../../peripherals/usbpd_h4.rs"]
pub mod usbpd;
#[path = "../../peripherals/usbss_h4.rs"]
pub mod usbss;
#[path = "../../peripherals/wwdg_common.rs"]
pub mod wwdg;
pub const CORE_INDEX: usize = 1;
pub const FLASH_BASE: usize = 0;
pub const FLASH_SIZE: usize = 983040;
pub const WRITE_SIZE: usize = 256;
