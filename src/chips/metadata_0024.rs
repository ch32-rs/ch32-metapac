
pub(crate) static PERIPHERALS: &[Peripheral] = &[
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
            version: "x0",
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
            PeripheralInterrupt {
                signal: "EXTI16",
                interrupt: "EXTI25_16",
            },
        ],
    },
    Peripheral {
        name: "RCC",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "x0",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[PeripheralPin {
            pin: "PB9",
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
            PeripheralInterrupt {
                signal: "CH8",
                interrupt: "DMA1_CHANNEL8",
            },
        ],
    },
    Peripheral {
        name: "GPIOA",
        address: 0x40010800,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "x0",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
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
            version: "x0",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
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
            version: "x0",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
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
        name: "AFIO",
        address: 0x40010000,
        registers: Some(PeripheralRegisters {
            kind: "afio",
            version: "x0",
            block: "AFIO",
            ir: &afio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
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
                pin: "PB11",
                signal: "RX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "CTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CK",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "RTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "CTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "TX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CK",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "RTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CTS",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CK",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RTS",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "RX",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CTS",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "TX",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CK",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA14",
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
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
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
                pin: "PA3",
                signal: "RX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "RX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "TX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CK",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "RTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "RX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CTS",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "TX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CK",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "RTS",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "RX",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CTS",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "TX",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PB20",
                signal: "CK",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "RTS",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA16",
                signal: "RX",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PA17",
                signal: "CTS",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "TX",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CK",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "RTS",
                remap: Some(4),
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
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
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
                pin: "PB4",
                signal: "RX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "TX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CK",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "RX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "TX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CK",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CTS",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "TX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CK",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "RTS",
                remap: Some(2),
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
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
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
            register: "PCFR1",
            field: "USART4_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PB1",
                signal: "RX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "TX",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CK",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RTS",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "TX",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CK",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "RTS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "RX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "TX",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CK",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RTS",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CTS",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CK",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "RTS",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "RX",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CTS",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "RTS",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "RX",
                remap: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                remap: Some(5),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "TX",
                remap: Some(5),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CK",
                remap: Some(5),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RTS",
                remap: Some(5),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH8"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH1"),
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
        name: "SPI1",
        address: 0x40013000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "v0",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
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
                pin: "PA8",
                signal: "MISO",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "MOSI",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "MISO",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "MOSI",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "NSS",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "SCK",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "NSS",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "NSS",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "SCK",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "MISO",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "MOSI",
                remap: Some(3),
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
        name: "ADC1",
        address: 0x40012400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "x0",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "SYSCLK",
            kernel_clock: Clock("SYSCLK"),
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
            interrupt: "ADC1",
        }],
    },
    Peripheral {
        name: "TIM1",
        address: 0x40012c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "x0",
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
                signal: "CH1",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH2",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH3",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "CH4",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "ETR",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "BKIN",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH2",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH3",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC16",
                signal: "CH4",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC17",
                signal: "ETR",
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
                pin: "PB9",
                signal: "CH1",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH2",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH3",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH4",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "ETR",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "BKIN",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1N",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2N",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3N",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH1",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH2",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH3",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH4",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "ETR",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "BKIN",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CH1N",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH2N",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PC7",
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
            version: "x0",
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
                pin: "PA0",
                signal: "CH1",
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
                pin: "PA19",
                signal: "ETR",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "BKIN",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA21",
                signal: "CH1N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CH2N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CH3N",
                remap: Some(0),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "CH1",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB15",
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
                pin: "PA18",
                signal: "ETR",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "BKIN",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CH1N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CH2N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH3N",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH3",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH4",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA19",
                signal: "ETR",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA20",
                signal: "BKIN",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH1N",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA22",
                signal: "CH2N",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA23",
                signal: "CH3N",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PB21",
                signal: "CH1",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH3",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH4",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA18",
                signal: "ETR",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "BKIN",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CH1N",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CH2N",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH3N",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PB16",
                signal: "CH1",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PB17",
                signal: "CH2",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PB18",
                signal: "CH3",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PB19",
                signal: "CH4",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "ETR",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "BKIN",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH1N",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH2N",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH3N",
                remap: Some(4),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "CH1",
                remap: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CH2",
                remap: Some(5),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CH3",
                remap: Some(5),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH4",
                remap: Some(5),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "ETR",
                remap: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "BKIN",
                remap: Some(5),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "CH1N",
                remap: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH2N",
                remap: Some(5),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH3N",
                remap: Some(5),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "CH1",
                remap: Some(6),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "CH2",
                remap: Some(6),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "CH3",
                remap: Some(6),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH4",
                remap: Some(6),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "ETR",
                remap: Some(6),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "BKIN",
                remap: Some(6),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH1N",
                remap: Some(6),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH2N",
                remap: Some(6),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH3N",
                remap: Some(6),
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
                signal: "COM",
                channel: Some("DMA1_CH8"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH8"),
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
                signal: "BRK",
                interrupt: "TIM2_BRK",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM2_UP",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM2_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM2_TRG_COM",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM2_CC",
            },
        ],
    },
    Peripheral {
        name: "TIM3",
        address: 0x40000400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "x0",
            block: "GPTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
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
                pin: "PB4",
                signal: "CH1",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                remap: Some(1),
            },
            PeripheralPin {
                pin: "PC19",
                signal: "CH1",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PC18",
                signal: "CH2",
                remap: Some(2),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH1",
                remap: Some(3),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH2",
                remap: Some(3),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM3",
            },
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
        name: "OPA",
        address: 0x40026000,
        registers: Some(PeripheralRegisters {
            kind: "opa",
            version: "x0",
            block: "OPA",
            ir: &opa::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PIOC",
        address: 0x40026c00,
        registers: Some(PeripheralRegisters {
            kind: "pioc",
            version: "x0",
            block: "PIOC",
            ir: &pioc::REGISTERS,
        }),
        rcc: None,
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "PIOC_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PC18",
                signal: "IO0",
                remap: None,
            },
            PeripheralPin {
                pin: "PC19",
                signal: "IO1",
                remap: None,
            },
            PeripheralPin {
                pin: "PC7",
                signal: "IO0",
                remap: Some(1),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PIOC",
        }],
    },
    Peripheral {
        name: "USBFS",
        address: 0x40023400,
        registers: Some(PeripheralRegisters {
            kind: "usb",
            version: "x0fs",
            block: "USB",
            ir: &usb::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "AHBPCENR",
                field: "USBFSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "USBFSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PC16",
                signal: "DM",
                remap: None,
            },
            PeripheralPin {
                pin: "PC17",
                signal: "DP",
                remap: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USBFS",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "USBFS_WKUP",
            },
        ],
    },
    Peripheral {
        name: "AWU",
        address: 0x40026400,
        registers: Some(PeripheralRegisters {
            kind: "awu",
            version: "x0",
            block: "AWU",
            ir: &awu::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "AWU",
        }],
    },
    Peripheral {
        name: "USBPD",
        address: 0x40027000,
        registers: Some(PeripheralRegisters {
            kind: "usbpd",
            version: "x0",
            block: "USBPD",
            ir: &usbpd::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "AHBPCENR",
                field: "USBPD",
            }),
            reset: Some(PeripheralRccRegister {
                register: "AHBRSTR",
                field: "USBPDRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PC14",
                signal: "CC1",
                remap: None,
            },
            PeripheralPin {
                pin: "PC15",
                signal: "CC2",
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
        name: "ADC1",
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
        name: "SPI1",
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
        name: "TIM2_UP",
        number: 38,
    },
    Interrupt {
        name: "USART2",
        number: 39,
    },
    Interrupt {
        name: "EXTI15_8",
        number: 40,
    },
    Interrupt {
        name: "EXTI25_16",
        number: 41,
    },
    Interrupt {
        name: "USART3",
        number: 42,
    },
    Interrupt {
        name: "USART4",
        number: 43,
    },
    Interrupt {
        name: "DMA1_CHANNEL8",
        number: 44,
    },
    Interrupt {
        name: "USBFS",
        number: 45,
    },
    Interrupt {
        name: "USBFS_WKUP",
        number: 46,
    },
    Interrupt {
        name: "PIOC",
        number: 47,
    },
    Interrupt {
        name: "OPA",
        number: 48,
    },
    Interrupt {
        name: "USBPD",
        number: 49,
    },
    Interrupt {
        name: "USBPD_WKUP",
        number: 50,
    },
    Interrupt {
        name: "TIM2_CC",
        number: 51,
    },
    Interrupt {
        name: "TIM2_TRG_COM",
        number: 52,
    },
    Interrupt {
        name: "TIM2_BRK",
        number: 53,
    },
    Interrupt {
        name: "TIM3",
        number: 54,
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
        name: "DMA1_CH8",
        dma: "DMA1",
        channel: 7,
        dmamux: None,
        dmamux_channel: None,
    },
];
#[path = "../registers/adc_x0.rs"]
pub mod adc;
#[path = "../registers/afio_x0.rs"]
pub mod afio;
#[path = "../registers/awu_x0.rs"]
pub mod awu;
#[path = "../registers/dma_v1.rs"]
pub mod dma;
#[path = "../registers/exti_common.rs"]
pub mod exti;
#[path = "../registers/flash_x0.rs"]
pub mod flash;
#[path = "../registers/gpio_x0.rs"]
pub mod gpio;
#[path = "../registers/opa_x0.rs"]
pub mod opa;
#[path = "../registers/pfic_rv4.rs"]
pub mod pfic;
#[path = "../registers/pioc_x0.rs"]
pub mod pioc;
#[path = "../registers/rcc_x0.rs"]
pub mod rcc;
#[path = "../registers/spi_v0.rs"]
pub mod spi;
#[path = "../registers/systick_rv4.rs"]
pub mod systick;
#[path = "../registers/timer_x0.rs"]
pub mod timer;
#[path = "../registers/usart_common.rs"]
pub mod usart;
#[path = "../registers/usb_x0fs.rs"]
pub mod usb;
#[path = "../registers/usbpd_x0.rs"]
pub mod usbpd;
