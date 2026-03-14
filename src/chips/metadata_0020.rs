
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "RCC",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "v3_d8c",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[PeripheralPin {
            pin: "PA8",
            signal: "MCO",
            remap: None,
        }],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PFIC",
        address: 0xe000e000,
        registers: Some(PeripheralRegisters {
            kind: "pfic",
            version: "rv4",
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
            version: "rv4",
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
            version: "v3",
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
                interrupt: "EXTI0",
            },
            PeripheralInterrupt {
                signal: "EXTI1",
                interrupt: "EXTI1",
            },
            PeripheralInterrupt {
                signal: "EXTI2",
                interrupt: "EXTI2",
            },
            PeripheralInterrupt {
                signal: "EXTI3",
                interrupt: "EXTI3",
            },
            PeripheralInterrupt {
                signal: "EXTI4",
                interrupt: "EXTI4",
            },
            PeripheralInterrupt {
                signal: "EXTI5",
                interrupt: "EXTI9_5",
            },
            PeripheralInterrupt {
                signal: "EXTI10",
                interrupt: "EXTI15_10",
            },
        ],
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
        name: "DMA2",
        address: 0x40020400,
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
                field: "DMA2EN",
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
                interrupt: "DMA2_CHANNEL1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA2_CHANNEL2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA2_CHANNEL3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA2_CHANNEL4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA2_CHANNEL5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA2_CHANNEL6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA2_CHANNEL7",
            },
            PeripheralInterrupt {
                signal: "CH8",
                interrupt: "DMA2_CHANNEL8",
            },
            PeripheralInterrupt {
                signal: "CH9",
                interrupt: "DMA2_CHANNEL9",
            },
            PeripheralInterrupt {
                signal: "CH10",
                interrupt: "DMA2_CHANNEL10",
            },
            PeripheralInterrupt {
                signal: "CH11",
                interrupt: "DMA2_CHANNEL11",
            },
        ],
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
        name: "GPIOC",
        address: 0x40011000,
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
                field: "IOPCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "IOPCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOD",
        address: 0x40011400,
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
                field: "IOPDEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "IOPDRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIOE",
        address: 0x40011800,
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
                field: "IOPEEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "IOPERST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "AFIO",
        address: 0x40010000,
        registers: Some(PeripheralRegisters {
            kind: "afio",
            version: "v3",
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
        name: "EXTEND",
        address: 0x40023800,
        registers: Some(PeripheralRegisters {
            kind: "extend",
            version: "v3",
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
        name: "RTC",
        address: 0x40002800,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "common",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DAC",
        address: 0x40007400,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v3",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("DAC"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "DACEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "DACRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "OUT0",
                remap: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "OUT1",
                remap: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
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
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
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
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CK",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "TX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CTS",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RTS",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CK",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "TX",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RX",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CTS",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RTS",
                remap: Some(3),
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
        name: "USART2",
        address: 0x40004400,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "common",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "USART2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "USART2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "USART2_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "RTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "TX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "RX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CK",
                remap: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART2",
        }],
    },
    Peripheral {
        name: "USART3",
        address: 0x40004800,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "common",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "USART3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "USART3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "USART3_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CK",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "TX",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "RX",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CK",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CTS",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "RTS",
                remap: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART3",
        }],
    },
    Peripheral {
        name: "USART4",
        address: 0x40004c00,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "common",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "USART4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "USART4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR2",
            field: "USART4_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "TX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "TX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "RX",
                remap: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART4",
        }],
    },
    Peripheral {
        name: "USART5",
        address: 0x40005000,
        registers: Some(PeripheralRegisters {
            kind: "usart",
            version: "common",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "USART5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "USART5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR2",
            field: "USART5_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "TX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "TX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "RX",
                remap: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART5",
        }],
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
            bus_clock: "PCLK2_TIM",
            kernel_clock: Clock("PCLK2"),
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
                pin: "PA12",
                signal: "ETR",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "ETR",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "CH1",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CH2",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "CH3",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CH4",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "BKIN",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "CH1N",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CH2N",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "CH3N",
                remap: Some(3),
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
                signal: "CH4",
                channel: Some("DMA1_CH4"),
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
                signal: "BRK",
                interrupt: "TIM1_BRK",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1_UP",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1_CC",
            },
        ],
    },
    Peripheral {
        name: "TIM2",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "GPTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1_TIM",
            kernel_clock: Clock("PCLK1"),
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
                pin: "PA0",
                signal: "CH1",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
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
                pin: "PA3",
                signal: "CH4",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH3",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH4",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "ETR",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH3",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH4",
                remap: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM2",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM2",
            },
        ],
    },
    Peripheral {
        name: "TIM3",
        address: 0x40000400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "GPTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1_TIM",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "TIM3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "TIM3_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "ETR",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "ETR",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "ETR",
                remap: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM3",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM3",
            },
        ],
    },
    Peripheral {
        name: "TIM4",
        address: 0x40000800,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "GPTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1_TIM",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "TIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "TIM4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "TIM4_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CH1",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CH2",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CH3",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CH4",
                remap: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM4",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM4",
            },
        ],
    },
    Peripheral {
        name: "TIM5",
        address: 0x40000c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "GPTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1_TIM",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "TIM5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "TIM5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                remap: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                remap: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                remap: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                remap: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM5",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM5",
            },
        ],
    },
    Peripheral {
        name: "TIM6",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "BCTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1_TIM",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "TIM6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "TIM6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: Some("DMA2_CH3"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "UP",
            interrupt: "TIM6",
        }],
    },
    Peripheral {
        name: "TIM7",
        address: 0x40001400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "BCTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1_TIM",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "TIM7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "TIM7RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[PeripheralDmaChannel {
            signal: "UP",
            channel: Some("DMA2_CH4"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "UP",
            interrupt: "TIM7",
        }],
    },
    Peripheral {
        name: "TIM8",
        address: 0x40013400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "ADTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2_TIM",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "TIM8EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "TIM8RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR2",
            field: "TIM8_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "CH4",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "BKIN",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CH1N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH2N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH3N",
                remap: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM8_BRK",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM8_UP",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM8_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM8_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM8_CC",
            },
        ],
    },
    Peripheral {
        name: "TIM9",
        address: 0x40014c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "ADTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2_TIM",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "TIM9EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "TIM9RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR2",
            field: "TIM9_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "ETR",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH3",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CH4",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "BKIN",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH1N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH2N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH3N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "ETR",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH2",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH3",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "CH4",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "BKIN",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH1N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH2N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH3N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "ETR",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "CH1",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CH2",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CH3",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CH4",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "BKIN",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "CH1N",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CH2N",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CH3N",
                remap: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH9"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH11"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA2_CH8"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some("DMA2_CH10"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA2_CH10"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM9_BRK",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM9_UP",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM9_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM9_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM9_CC",
            },
        ],
    },
    Peripheral {
        name: "TIM10",
        address: 0x40015000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3",
            block: "ADTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2_TIM",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "TIM10EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "TIM10RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR2",
            field: "TIM10_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PC10",
                signal: "ETR",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH2",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH3",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CH4",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "BKIN",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CH1N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CH2N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH3N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "ETR",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH1",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH2",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH3",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "CH4",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "BKIN",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH2N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH3N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "ETR",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "CH1",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CH2",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "CH3",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CH4",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "BKIN",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "CH1N",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "CH2N",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CH3N",
                remap: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH8"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH10"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH9"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "COM",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH11"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM10_BRK",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM10_UP",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM10_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM10_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM10_CC",
            },
        ],
    },
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "v3",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("ADC"),
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
                pin: "PA0",
                signal: "IN0",
                remap: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                remap: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                remap: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN3",
                remap: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN4",
                remap: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN5",
                remap: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN6",
                remap: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN7",
                remap: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN8",
                remap: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN9",
                remap: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN10",
                remap: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                remap: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                remap: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN13",
                remap: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN14",
                remap: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN15",
                remap: None,
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
        name: "ADC2",
        address: 0x40012800,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "v3",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("ADC"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "ADC2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "ADC2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN0",
                remap: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                remap: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                remap: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN3",
                remap: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN4",
                remap: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN5",
                remap: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN6",
                remap: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN7",
                remap: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN8",
                remap: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN9",
                remap: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN10",
                remap: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                remap: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                remap: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN13",
                remap: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN14",
                remap: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN15",
                remap: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
    },
    Peripheral {
        name: "SPI1",
        address: 0x40013000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v3",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "APB2PCENR",
                field: "SPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB2PRSTR",
                field: "SPI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "SPI1_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                remap: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI1",
        }],
    },
    Peripheral {
        name: "SPI2",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v3",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "SPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "SPI2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                remap: Some(0),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI2",
        }],
    },
    Peripheral {
        name: "SPI3",
        address: 0x40003c00,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v3",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "SPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "SPI3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "SPI3_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SCK",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "MISO",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "MOSI",
                remap: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
    },
    Peripheral {
        name: "I2C1",
        address: 0x40005400,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v3",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
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
                pin: "PB5",
                signal: "SMBA",
                remap: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                remap: Some(1),
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
                signal: "ER",
                interrupt: "I2C1_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C1_EV",
            },
        ],
    },
    Peripheral {
        name: "I2C2",
        address: 0x40005800,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v3",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "I2C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "I2C2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                remap: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SDA",
                remap: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
                remap: None,
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2_ER",
            },
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2_EV",
            },
        ],
    },
    Peripheral {
        name: "RNG",
        address: 0x40023c00,
        registers: Some(PeripheralRegisters {
            kind: "rng",
            version: "v3",
            block: "RNG",
            ir: &rng::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "AHBPCENR",
                field: "TRNG_EN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RNG",
        }],
    },
    Peripheral {
        name: "USBHS",
        address: 0x40023400,
        registers: Some(PeripheralRegisters {
            kind: "usbhs",
            version: "v3",
            block: "USB",
            ir: &usbhs::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("USBHSPLLCLK"),
            enable: Some(PeripheralRccRegister {
                register: "AHBPCENR",
                field: "USBHSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "USBHSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "DM",
                remap: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "DP",
                remap: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USBHS",
            },
            PeripheralInterrupt {
                signal: "WAKEUP",
                interrupt: "USBHS_WKUP",
            },
        ],
    },
    Peripheral {
        name: "DAC1",
        address: 0x40007400,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "v3",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "DACEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "DACRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "OUT1",
                remap: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "OUT2",
                remap: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[],
    },
    Peripheral {
        name: "SDIO",
        address: 0x40018000,
        registers: Some(PeripheralRegisters {
            kind: "sdio",
            version: "v3",
            block: "SDIO",
            ir: &sdio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "AHBPCENR",
                field: "SDIOEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PC8",
                signal: "D0",
                remap: None,
            },
            PeripheralPin {
                pin: "PC9",
                signal: "D1",
                remap: None,
            },
            PeripheralPin {
                pin: "PC10",
                signal: "D2",
                remap: None,
            },
            PeripheralPin {
                pin: "PC11",
                signal: "D3",
                remap: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "D4",
                remap: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "D5",
                remap: None,
            },
            PeripheralPin {
                pin: "PC6",
                signal: "D6",
                remap: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "D7",
                remap: None,
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                remap: None,
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CMD",
                remap: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "D0",
                remap: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "D1",
                remap: None,
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "SDIO",
            channel: Some("DMA2_CH4"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SDIO",
        }],
    },
    Peripheral {
        name: "CAN1",
        address: 0x40006400,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "v3",
            block: "CAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "CAN1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "CAN1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "CAN1_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "RX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                remap: Some(1),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "USB_HP_CAN1_TX",
            },
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "USB_LP_CAN1_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "CAN1_RX1",
            },
            PeripheralInterrupt {
                signal: "SCE",
                interrupt: "CAN1_SCE",
            },
        ],
    },
    Peripheral {
        name: "CAN2",
        address: 0x40006800,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "v3",
            block: "CAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "APB1PCENR",
                field: "CAN2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "APB1PRSTR",
                field: "CAN2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "CAN2_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "RX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                remap: Some(1),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "CAN2_TX",
            },
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "CAN2_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "CAN2_RX1",
            },
            PeripheralInterrupt {
                signal: "SCE",
                interrupt: "CAN2_SCE",
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
        name: "TAMPER",
        number: 18,
    },
    Interrupt {
        name: "RTC",
        number: 19,
    },
    Interrupt {
        name: "FLASH",
        number: 20,
    },
    Interrupt {
        name: "RCC",
        number: 21,
    },
    Interrupt {
        name: "EXTI0",
        number: 22,
    },
    Interrupt {
        name: "EXTI1",
        number: 23,
    },
    Interrupt {
        name: "EXTI2",
        number: 24,
    },
    Interrupt {
        name: "EXTI3",
        number: 25,
    },
    Interrupt {
        name: "EXTI4",
        number: 26,
    },
    Interrupt {
        name: "DMA1_CHANNEL1",
        number: 27,
    },
    Interrupt {
        name: "DMA1_CHANNEL2",
        number: 28,
    },
    Interrupt {
        name: "DMA1_CHANNEL3",
        number: 29,
    },
    Interrupt {
        name: "DMA1_CHANNEL4",
        number: 30,
    },
    Interrupt {
        name: "DMA1_CHANNEL5",
        number: 31,
    },
    Interrupt {
        name: "DMA1_CHANNEL6",
        number: 32,
    },
    Interrupt {
        name: "DMA1_CHANNEL7",
        number: 33,
    },
    Interrupt {
        name: "ADC",
        number: 34,
    },
    Interrupt {
        name: "USB_HP_CAN1_TX",
        number: 35,
    },
    Interrupt {
        name: "USB_LP_CAN1_RX0",
        number: 36,
    },
    Interrupt {
        name: "CAN1_RX1",
        number: 37,
    },
    Interrupt {
        name: "CAN1_SCE",
        number: 38,
    },
    Interrupt {
        name: "EXTI9_5",
        number: 39,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 40,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 41,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
        number: 42,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 43,
    },
    Interrupt {
        name: "TIM2",
        number: 44,
    },
    Interrupt {
        name: "TIM3",
        number: 45,
    },
    Interrupt {
        name: "TIM4",
        number: 46,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 47,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 48,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 49,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 50,
    },
    Interrupt {
        name: "SPI1",
        number: 51,
    },
    Interrupt {
        name: "SPI2",
        number: 52,
    },
    Interrupt {
        name: "USART1",
        number: 53,
    },
    Interrupt {
        name: "USART2",
        number: 54,
    },
    Interrupt {
        name: "USART3",
        number: 55,
    },
    Interrupt {
        name: "EXTI15_10",
        number: 56,
    },
    Interrupt {
        name: "RTCALARM",
        number: 57,
    },
    Interrupt {
        name: "USB_WKUP",
        number: 58,
    },
    Interrupt {
        name: "TIM8_BRK",
        number: 59,
    },
    Interrupt {
        name: "TIM8_UP",
        number: 60,
    },
    Interrupt {
        name: "TIM8_TRG_COM",
        number: 61,
    },
    Interrupt {
        name: "TIM8_CC",
        number: 62,
    },
    Interrupt {
        name: "RNG",
        number: 63,
    },
    Interrupt {
        name: "SDIO",
        number: 65,
    },
    Interrupt {
        name: "TIM5",
        number: 66,
    },
    Interrupt {
        name: "SPI3",
        number: 67,
    },
    Interrupt {
        name: "USART4",
        number: 68,
    },
    Interrupt {
        name: "USART5",
        number: 69,
    },
    Interrupt {
        name: "TIM6",
        number: 70,
    },
    Interrupt {
        name: "TIM7",
        number: 71,
    },
    Interrupt {
        name: "DMA2_CHANNEL1",
        number: 72,
    },
    Interrupt {
        name: "DMA2_CHANNEL2",
        number: 73,
    },
    Interrupt {
        name: "DMA2_CHANNEL3",
        number: 74,
    },
    Interrupt {
        name: "DMA2_CHANNEL4",
        number: 75,
    },
    Interrupt {
        name: "DMA2_CHANNEL5",
        number: 76,
    },
    Interrupt {
        name: "ETH",
        number: 77,
    },
    Interrupt {
        name: "ETH_WKUP",
        number: 78,
    },
    Interrupt {
        name: "CAN2_TX",
        number: 79,
    },
    Interrupt {
        name: "CAN2_RX0",
        number: 80,
    },
    Interrupt {
        name: "CAN2_RX1",
        number: 81,
    },
    Interrupt {
        name: "CAN2_SCE",
        number: 82,
    },
    Interrupt {
        name: "OTG_FS",
        number: 83,
    },
    Interrupt {
        name: "USBHS_WKUP",
        number: 84,
    },
    Interrupt {
        name: "USBHS",
        number: 85,
    },
    Interrupt {
        name: "DVP",
        number: 86,
    },
    Interrupt {
        name: "USART6",
        number: 87,
    },
    Interrupt {
        name: "USART7",
        number: 88,
    },
    Interrupt {
        name: "USART8",
        number: 89,
    },
    Interrupt {
        name: "TIM9_BRK",
        number: 90,
    },
    Interrupt {
        name: "TIM9_UP",
        number: 91,
    },
    Interrupt {
        name: "TIM9_TRG_COM",
        number: 92,
    },
    Interrupt {
        name: "TIM9_CC",
        number: 93,
    },
    Interrupt {
        name: "TIM10_BRK",
        number: 94,
    },
    Interrupt {
        name: "TIM10_UP",
        number: 95,
    },
    Interrupt {
        name: "TIM10_TRG_COM",
        number: 96,
    },
    Interrupt {
        name: "TIM10_CC",
        number: 97,
    },
    Interrupt {
        name: "DMA2_CHANNEL6",
        number: 98,
    },
    Interrupt {
        name: "DMA2_CHANNEL7",
        number: 99,
    },
    Interrupt {
        name: "DMA2_CHANNEL8",
        number: 100,
    },
    Interrupt {
        name: "DMA2_CHANNEL9",
        number: 101,
    },
    Interrupt {
        name: "DMA2_CHANNEL10",
        number: 102,
    },
    Interrupt {
        name: "DMA2_CHANNEL11",
        number: 103,
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
    DmaChannel {
        name: "DMA2_CH1",
        dma: "DMA2",
        channel: 0,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH2",
        dma: "DMA2",
        channel: 1,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH3",
        dma: "DMA2",
        channel: 2,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH4",
        dma: "DMA2",
        channel: 3,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH5",
        dma: "DMA2",
        channel: 4,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH6",
        dma: "DMA2",
        channel: 5,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH7",
        dma: "DMA2",
        channel: 6,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH8",
        dma: "DMA2",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH9",
        dma: "DMA2",
        channel: 8,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH10",
        dma: "DMA2",
        channel: 9,
        dmamux: None,
        dmamux_channel: None,
    },
    DmaChannel {
        name: "DMA2_CH11",
        dma: "DMA2",
        channel: 10,
        dmamux: None,
        dmamux_channel: None,
    },
];
#[path = "../registers/adc_v3.rs"]
pub mod adc;
#[path = "../registers/afio_v3.rs"]
pub mod afio;
#[path = "../registers/can_v3.rs"]
pub mod can;
#[path = "../registers/dac_v3.rs"]
pub mod dac;
#[path = "../registers/dma_v1.rs"]
pub mod dma;
#[path = "../registers/extend_v3.rs"]
pub mod extend;
#[path = "../registers/exti_common.rs"]
pub mod exti;
#[path = "../registers/flash_v3.rs"]
pub mod flash;
#[path = "../registers/gpio_v3.rs"]
pub mod gpio;
#[path = "../registers/i2c_v3.rs"]
pub mod i2c;
#[path = "../registers/pfic_rv4.rs"]
pub mod pfic;
#[path = "../registers/rcc_v3_d8c.rs"]
pub mod rcc;
#[path = "../registers/rng_v3.rs"]
pub mod rng;
#[path = "../registers/rtc_common.rs"]
pub mod rtc;
#[path = "../registers/sdio_v3.rs"]
pub mod sdio;
#[path = "../registers/spi_v3.rs"]
pub mod spi;
#[path = "../registers/systick_rv4.rs"]
pub mod systick;
#[path = "../registers/timer_v3.rs"]
pub mod timer;
#[path = "../registers/usart_common.rs"]
pub mod usart;
#[path = "../registers/usbhs_v3.rs"]
pub mod usbhs;
