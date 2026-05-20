
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "PFIC",
        address: 0xe000e000,
        registers: Some(PeripheralRegisters {
            kind: "pfic",
            version: "rv2",
            block: "PFIC",
            ir: &pfic::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SYSTICK",
        address: 0xe000f000,
        registers: Some(PeripheralRegisters {
            kind: "systick",
            version: "rv2",
            block: "SYSTICK",
            ir: &systick::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "FLASH",
        address: 0x40022000,
        registers: Some(PeripheralRegisters {
            kind: "flash",
            version: "v0",
            block: "FLASH",
            ir: &flash::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EXTI",
        address: 0x40010400,
        registers: Some(PeripheralRegisters {
            kind: "exti",
            version: "common",
            block: "EXTI",
            ir: &exti::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EXTI0",
                interrupt: "EXTI7_0",
            },
            PeripheralInterrupt {
                signal: "EXTI8",
                interrupt: "EXTI15_8",
            },
        ],
    },
    Peripheral {
        name: "EXTEND",
        address: 0x40023800,
        registers: Some(PeripheralRegisters {
            kind: "extend",
            version: "ch641",
            block: "EXTEND",
            ir: &extend::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RCC",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "ch641",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[PeripheralPin {
            pin: "PB7",
            signal: "MCO",
            remap: None,
        }],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DMA1",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v1",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "AHBPCENR",
                field: "DMA1EN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "DMA1_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_CHANNEL7",
            },
        ],
    },
    Peripheral {
        name: "AFIO",
        address: 0x40010000,
        registers: Some(PeripheralRegisters {
            kind: "afio",
            version: "ch641",
            block: "AFIO",
            ir: &afio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "AFIOEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "AFIORST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOA",
        address: 0x40010800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v3",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "IOPAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "IOPARST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOB",
        address: 0x40010c00,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "v3",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "IOPBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "IOPBRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TIM1",
        address: 0x40012c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "ADTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "TIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "TIM1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "TIM1_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PB9",
                signal: "ETR",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH3",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "BKIN",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH3N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "ETR",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "BKIN",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH3N",
                remap: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1_BRK",
            },
        ],
    },
    Peripheral {
        name: "TIM2",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "ADTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "TIM2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "TIM2_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH2N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH2",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH2N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH1N",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2N",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH2",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH1N",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2N",
                remap: Some(3),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM2",
            },
        ],
    },
    Peripheral {
        name: "USART1",
        address: 0x40013800,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "common",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "USART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "USART1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "USART1_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "TX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "TX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "RX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CTS",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RTS",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "TX",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "RX",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CTS",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RTS",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CTS",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RTS",
                remap: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART1",
        }],
    },
    Peripheral {
        name: "I2C1",
        address: 0x40005400,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v0",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "I2C1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "I2C1_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "SCL",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SDA",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SCL",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "SDA",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "SCL",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SDA",
                remap: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C1_EV",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C1_ER",
            },
        ],
    },
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ch641",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "ADC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "ADC1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "IN0",
                remap: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN1",
                remap: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "IN2",
                remap: None,
            },
            PeripheralPin {
                pin: "PA10",
                signal: "IN3",
                remap: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "IN4",
                remap: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "IN5",
                remap: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "IN6",
                remap: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "IN7",
                remap: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "IN9",
                remap: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN10",
                remap: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN11",
                remap: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "IN12",
                remap: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN13",
                remap: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "IN14",
                remap: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "ETR",
                remap: Some(0),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "ADC1",
            channel: Some("DMA1_CH1"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
    },
    Peripheral {
        name: "USBPD",
        address: 0x40027000,
        registers: Some(PeripheralRegisters {
            kind: "usbpd",
            version: "ch641",
            block: "USBPD",
            ir: &usbpd::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "AHBPCENR",
                field: "USBPDEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "CC1",
                remap: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CC2",
                remap: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CC3",
                remap: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USBPD",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "USBPD_WKUP",
            },
        ],
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 16,
    },
    Interrupt {
        name: "PVD",
        number: 17,
    },
    Interrupt {
        name: "FLASH",
        number: 18,
    },
    Interrupt {
        name: "RCC",
        number: 19,
    },
    Interrupt {
        name: "EXTI7_0",
        number: 20,
    },
    Interrupt {
        name: "AWU",
        number: 21,
    },
    Interrupt {
        name: "DMA1_CHANNEL1",
        number: 22,
    },
    Interrupt {
        name: "DMA1_CHANNEL2",
        number: 23,
    },
    Interrupt {
        name: "DMA1_CHANNEL3",
        number: 24,
    },
    Interrupt {
        name: "DMA1_CHANNEL4",
        number: 25,
    },
    Interrupt {
        name: "DMA1_CHANNEL5",
        number: 26,
    },
    Interrupt {
        name: "DMA1_CHANNEL6",
        number: 27,
    },
    Interrupt {
        name: "DMA1_CHANNEL7",
        number: 28,
    },
    Interrupt {
        name: "ADC",
        number: 29,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 30,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 31,
    },
    Interrupt {
        name: "USART1",
        number: 32,
    },
    Interrupt {
        name: "EXTI15_8",
        number: 33,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 34,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 35,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
        number: 36,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 37,
    },
    Interrupt {
        name: "TIM2",
        number: 38,
    },
    Interrupt {
        name: "USBPD",
        number: 39,
    },
    Interrupt {
        name: "USBPD_WKUP",
        number: 40,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[
    DmaChannel {
        name: "DMA1_CH1",
        dma: "DMA1",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH2",
        dma: "DMA1",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH3",
        dma: "DMA1",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH4",
        dma: "DMA1",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH5",
        dma: "DMA1",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH6",
        dma: "DMA1",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA1_CH7",
        dma: "DMA1",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
];
#[path = "../registers/adc_ch641.rs"]
pub mod adc;
#[path = "../registers/afio_ch641.rs"]
pub mod afio;
#[path = "../registers/dma_v1.rs"]
pub mod dma;
#[path = "../registers/extend_ch641.rs"]
pub mod extend;
#[path = "../registers/exti_common.rs"]
pub mod exti;
#[path = "../registers/flash_v0.rs"]
pub mod flash;
#[path = "../registers/gpio_v3.rs"]
pub mod gpio;
#[path = "../registers/i2c_v0.rs"]
pub mod i2c;
#[path = "../registers/pfic_rv2.rs"]
pub mod pfic;
#[path = "../registers/rcc_ch641.rs"]
pub mod rcc;
#[path = "../registers/systick_rv2.rs"]
pub mod systick;
#[path = "../registers/timer_v3.rs"]
pub mod timer;
#[path = "../registers/usart_common.rs"]
pub mod usart;
#[path = "../registers/usbpd_ch641.rs"]
pub mod usbpd;
