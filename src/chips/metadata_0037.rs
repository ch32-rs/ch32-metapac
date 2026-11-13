
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "PFIC",
        address: 0xe000e000,
        registers: Some(PeripheralRegisters {
            kind: "pfic",
            version: "v205",
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
            version: "v205",
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
        name: "RCC",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "v205",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[PeripheralPin {
            pin: "PA8",
            signal: "MCO",
            remap: None,
            af: None,
        }],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DMA1",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v205",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "DMAEN",
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
                interrupt: "DMA1_CH1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA1_CH2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA1_CH3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA1_CH4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA1_CH5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA1_CH6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA1_CH7",
            },
            PeripheralInterrupt {
                signal: "CH8",
                interrupt: "DMA1_CH8",
            },
        ],
    },
    Peripheral {
        name: "DMA2",
        address: 0x40020400,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "v205",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "DMAEN",
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
                interrupt: "DMA2_CH1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "DMA2_CH2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "DMA2_CH3",
            },
            PeripheralInterrupt {
                signal: "CH4",
                interrupt: "DMA2_CH4",
            },
            PeripheralInterrupt {
                signal: "CH5",
                interrupt: "DMA2_CH5",
            },
            PeripheralInterrupt {
                signal: "CH6",
                interrupt: "DMA2_CH6",
            },
            PeripheralInterrupt {
                signal: "CH7",
                interrupt: "DMA2_CH7",
            },
            PeripheralInterrupt {
                signal: "CH8",
                interrupt: "DMA2_CH8",
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
                register: "PB2PCENR",
                field: "IOPAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB2PRSTR",
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
                register: "PB2PCENR",
                field: "IOPBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB2PRSTR",
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
                register: "PB2PCENR",
                field: "IOPCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB2PRSTR",
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
                register: "PB2PCENR",
                field: "IOPDEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB2PRSTR",
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
                register: "PB2PCENR",
                field: "IOPEEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB2PRSTR",
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
            version: "v205",
            block: "AFIO",
            ir: &afio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "PB2PCENR",
                field: "AFIOEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB2PRSTR",
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
            version: "v205",
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
        name: "WWDG",
        address: 0x40002c00,
        registers: Some(PeripheralRegisters {
            kind: "wwdg",
            version: "v00x",
            block: "WWDG",
            ir: &wwdg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "PB1PCENR",
                field: "WWDGEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "IWDG",
        address: 0x40003000,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "v00x",
            block: "IWDG",
            ir: &iwdg::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CRC",
        address: 0x40023000,
        registers: Some(PeripheralRegisters {
            kind: "crc",
            version: "h4",
            block: "CRC",
            ir: &crc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "CRCEN",
            }),
            reset: None,
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
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "PB2PCENR",
                field: "USART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB2PRSTR",
                field: "USART1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "CK",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CK",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "RX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "TX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "RTS",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "RTS",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RTS",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CTS",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CTS",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CTS",
                remap: None,
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH5"),
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
                register: "PB1PCENR",
                field: "USART2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
                field: "USART2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CK",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "RTS",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CTS",
                remap: None,
                af: Some(2),
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
                register: "PB1PCENR",
                field: "USART3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
                field: "USART3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "CK",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CK",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "RTS",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CTS",
                remap: None,
                af: Some(2),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH3"),
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
                register: "PB1PCENR",
                field: "USART4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
                field: "USART4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PD0",
                signal: "CK",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "CK",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RX",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RTS",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "RTS",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CTS",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CTS",
                remap: None,
                af: Some(3),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH8"),
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
                register: "PB1PCENR",
                field: "USART5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
                field: "USART5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PC8",
                signal: "CK",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "CK",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "TX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RTS",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CTS",
                remap: None,
                af: Some(3),
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
                channel: Some("DMA2_CH3"),
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
        name: "USART6",
        address: 0x40001800,
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
                register: "PB1PCENR",
                field: "USART6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
                field: "USART6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB15",
                signal: "CK",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CK",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RTS",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "RTS",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CTS",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CTS",
                remap: None,
                af: Some(4),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART6",
        }],
    },
    Peripheral {
        name: "USART7",
        address: 0x40001c00,
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
                register: "PB1PCENR",
                field: "USART7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
                field: "USART7RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "CK",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "CK",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "RX",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "TX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "TX",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "RTS",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CTS",
                remap: None,
                af: Some(4),
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
                channel: Some("DMA2_CH8"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART7",
        }],
    },
    Peripheral {
        name: "USART8",
        address: 0x40002000,
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
                register: "PB1PCENR",
                field: "USART8EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
                field: "USART8RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA15",
                signal: "CK",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "CK",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "RX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "RX",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "TX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "TX",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "RTS",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CTS",
                remap: None,
                af: Some(3),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "RX",
            channel: Some("DMA2_CH3"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART8",
        }],
    },
    Peripheral {
        name: "TIM1",
        address: 0x40012c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "v3b",
            block: "ADTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2_TIM",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "PB2PCENR",
                field: "TIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB2PRSTR",
                field: "TIM1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "ETR",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "ETR",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "CH1",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "CH1",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CH2",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CH2",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CH3",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "CH3",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CH4",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "CH4",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CH1N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH1N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "CH1N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH2N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CH2N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CH2N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH3N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CH3N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "CH3N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "BKIN",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "BKIN",
                remap: None,
                af: Some(0),
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
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH6"),
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
            version: "v3b",
            block: "GPTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1_TIM",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "PB1PCENR",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
                field: "TIM2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CH1_ETR",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CH1_ETR",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "CH3",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH3",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CH4",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH4",
                remap: None,
                af: Some(1),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA1_CH1"),
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
                signal: "CH4",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH7"),
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
            version: "v3b",
            block: "GPTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1_TIM",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "PB1PCENR",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
                field: "TIM3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PD2",
                signal: "ETR",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                remap: None,
                af: Some(1),
            },
        ],
        dma_channels: &[
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
                signal: "UP",
                channel: Some("DMA1_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH6"),
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
                signal: "CH1",
                channel: Some("DMA2_CH3"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH8"),
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
            version: "v3b",
            block: "GPTM32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1_TIM",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "PB1PCENR",
                field: "TIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
                field: "TIM4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PE0",
                signal: "ETR",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CH1",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CH2",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CH3",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CH4",
                remap: None,
                af: Some(1),
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
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA2_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
                channel: Some("DMA2_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH2",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH4",
                channel: Some("DMA2_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA2_CH8"),
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
        name: "ADC1",
        address: 0x40012400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "v205",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("ADC"),
            enable: Some(PeripheralRccRegister {
                register: "PB2PCENR",
                field: "ADCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB2PRSTR",
                field: "ADCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN2",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN3",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN4",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN5",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "IN6",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN7",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN8",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN9",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "IN10",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "IN11",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "IN12",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "IN13",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "IN14",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "IN15",
                remap: None,
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some("DMA1_CH1"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "ADC1",
                channel: Some("DMA2_CH8"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
    },
    Peripheral {
        name: "TKEY",
        address: 0x40012400,
        registers: Some(PeripheralRegisters {
            kind: "tkey",
            version: "v205",
            block: "TKEY",
            ir: &tkey::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "PB2PCENR",
                field: "SPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB2PRSTR",
                field: "SPI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "SCK",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "MOSI",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "MISO",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                remap: None,
                af: Some(5),
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
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA2_CH7"),
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
            version: "v0",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "PB1PCENR",
                field: "SPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
                field: "SPI2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MISO",
                remap: None,
                af: Some(5),
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
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA2_CH4"),
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
            interrupt: "SPI2",
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
                register: "PB1PCENR",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
                field: "I2C1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SCL",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SDA",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
                remap: None,
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH7"),
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
                register: "PB1PCENR",
                field: "I2C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
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
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC13",
                signal: "SCL",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SDA",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC14",
                signal: "SDA",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC15",
                signal: "SMBA",
                remap: None,
                af: Some(7),
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TX",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "RX",
                channel: Some("DMA1_CH5"),
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
        name: "CAN1",
        address: 0x40006400,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "v205",
            block: "CAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "PB1PCENR",
                field: "CANEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
                field: "CANRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "RX",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                remap: None,
                af: Some(6),
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
        name: "USBFS",
        address: 0x50000000,
        registers: Some(PeripheralRegisters {
            kind: "usb",
            version: "v2fs",
            block: "USB",
            ir: &usb::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "USBFSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HBRSTR",
                field: "USBFSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "DM",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "DP",
                remap: None,
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USBFS",
            },
            PeripheralInterrupt {
                signal: "WAKEUP",
                interrupt: "USBFS_WKUP",
            },
        ],
    },
    Peripheral {
        name: "USBHS",
        address: 0x40023400,
        registers: Some(PeripheralRegisters {
            kind: "usbhs",
            version: "v4x7",
            block: "USB",
            ir: &usbhs::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("USBHSPLLCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "USBHSEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "DP",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "DM",
                remap: None,
                af: None,
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
        name: "USBPD",
        address: 0x40027000,
        registers: Some(PeripheralRegisters {
            kind: "usbpd",
            version: "l1",
            block: "USBPD",
            ir: &usbpd::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "USBPDEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HBRSTR",
                field: "USBPDRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CC1",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CC2",
                remap: None,
                af: Some(7),
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
    Peripheral {
        name: "PIOC",
        address: 0x40025c00,
        registers: Some(PeripheralRegisters {
            kind: "pioc",
            version: "v205",
            block: "PIOC",
            ir: &pioc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "PIOCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HBRSTR",
                field: "PIOCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "IO0",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "IO0",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "IO0",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "IO0",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "IO1",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "IO1",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "IO1",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "IO1",
                remap: None,
                af: Some(4),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PIOC",
        }],
    },
    Peripheral {
        name: "QSPI1",
        address: 0x40024c00,
        registers: Some(PeripheralRegisters {
            kind: "qspi",
            version: "v205",
            block: "QSPI",
            ir: &qspi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "QSPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HBRSTR",
                field: "QSPI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "SCK",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SCK",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "SCSN",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCSN",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SIO0",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "SIO0",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "SIO0",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "SIO1",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SIO1",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "SIO1",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "SIO2",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "SIO2",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SIO3",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "SIO3",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "SCSXN",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "SCSXN",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SIOX0",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "SIOX0",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SIOX1",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "SIOX1",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SIOX2",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "SIOX2",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "SIOX3",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "SIOX3",
                remap: None,
                af: Some(5),
            },
        ],
        dma_channels: &[PeripheralDmaChannel {
            signal: "QSPI1",
            channel: Some("DMA2_CH1"),
            dmamux: None,
            dma: None,
            request: None,
        }],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "QSPI1",
        }],
    },
    Peripheral {
        name: "OPA",
        address: 0x40026000,
        registers: Some(PeripheralRegisters {
            kind: "opa",
            version: "v205",
            block: "OPA",
            ir: &opa::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB1",
                signal: "OPA1_OUT1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "OPA1_OUT2",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "OPA1_CHN0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "OPA1_CHN1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "OPA1_CHP0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "OPA1_CHP1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "OPA1_CHP2",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "OPA1_CHNVB",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "OPA2_OUT1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "OPA2_OUT2",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "OPA2_CHN0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "OPA2_CHN1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "OPA2_CHP0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "OPA2_CHP1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "OPA2_CHNVB",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CMP1_CHP0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CMP1_CHP1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CMP1_CHN0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CMP1_CHN1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CMP1_OUT",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CMP2_CHP0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CMP2_CHP1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CMP2_CHN0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CMP2_CHN1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "CMP2_OUT",
                remap: None,
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "OPA",
            },
            PeripheralInterrupt {
                signal: "CMP",
                interrupt: "CMP_WKUP",
            },
        ],
    },
    Peripheral {
        name: "FSMC",
        address: 0xa0000000,
        registers: Some(PeripheralRegisters {
            kind: "fsmc",
            version: "v4x7",
            block: "FSMC",
            ir: &fsmc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "FSMCEN",
            }),
            reset: None,
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PD14",
                signal: "D0",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "D1",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "D2",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "D3",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "D4",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "D5",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "D6",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "D7",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "D8",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "D9",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "D10",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "D11",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "D12",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "D13",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "D14",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "D15",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "A16",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "A17",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "A18",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "A19",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "A20",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "A21",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "A22",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "A23",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "NADV",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "NADV",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CLK",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "NOE",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "NWE",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "NWAIT",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "NE1",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "NCE2",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "NBL0",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "NBL1",
                remap: None,
                af: Some(7),
            },
        ],
        dma_channels: &[],
        interrupts: &[],
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
        name: "DMA1_CH1",
        number: 27,
    },
    Interrupt {
        name: "DMA1_CH2",
        number: 28,
    },
    Interrupt {
        name: "DMA1_CH3",
        number: 29,
    },
    Interrupt {
        name: "DMA1_CH4",
        number: 30,
    },
    Interrupt {
        name: "DMA1_CH5",
        number: 31,
    },
    Interrupt {
        name: "DMA1_CH6",
        number: 32,
    },
    Interrupt {
        name: "DMA1_CH7",
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
        name: "QSPI1",
        number: 58,
    },
    Interrupt {
        name: "USBFS",
        number: 59,
    },
    Interrupt {
        name: "USBFS_WKUP",
        number: 60,
    },
    Interrupt {
        name: "USART4",
        number: 61,
    },
    Interrupt {
        name: "DMA1_CH8",
        number: 62,
    },
    Interrupt {
        name: "PIOC",
        number: 63,
    },
    Interrupt {
        name: "OPA",
        number: 64,
    },
    Interrupt {
        name: "USBPD",
        number: 65,
    },
    Interrupt {
        name: "USBPD_WKUP",
        number: 67,
    },
    Interrupt {
        name: "CMP_WKUP",
        number: 68,
    },
    Interrupt {
        name: "USBHS",
        number: 69,
    },
    Interrupt {
        name: "LVDS",
        number: 70,
    },
    Interrupt {
        name: "USBHS_WKUP",
        number: 71,
    },
    Interrupt {
        name: "LVDS_WKUP",
        number: 72,
    },
    Interrupt {
        name: "USART5",
        number: 73,
    },
    Interrupt {
        name: "USART6",
        number: 74,
    },
    Interrupt {
        name: "USART7",
        number: 75,
    },
    Interrupt {
        name: "USART8",
        number: 76,
    },
    Interrupt {
        name: "DMA2_CH1",
        number: 77,
    },
    Interrupt {
        name: "DMA2_CH2",
        number: 78,
    },
    Interrupt {
        name: "DMA2_CH3",
        number: 79,
    },
    Interrupt {
        name: "DMA2_CH4",
        number: 80,
    },
    Interrupt {
        name: "DMA2_CH5",
        number: 81,
    },
    Interrupt {
        name: "DMA2_CH6",
        number: 82,
    },
    Interrupt {
        name: "DMA2_CH7",
        number: 83,
    },
    Interrupt {
        name: "DMA2_CH8",
        number: 84,
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
];
#[path = "../registers/adc_v205.rs"]
pub mod adc;
#[path = "../registers/afio_v205.rs"]
pub mod afio;
#[path = "../registers/can_v205.rs"]
pub mod can;
#[path = "../registers/crc_h4.rs"]
pub mod crc;
#[path = "../registers/dma_v205.rs"]
pub mod dma;
#[path = "../registers/extend_v205.rs"]
pub mod extend;
#[path = "../registers/exti_common.rs"]
pub mod exti;
#[path = "../registers/flash_v205.rs"]
pub mod flash;
#[path = "../registers/fsmc_v4x7.rs"]
pub mod fsmc;
#[path = "../registers/gpio_v3.rs"]
pub mod gpio;
#[path = "../registers/i2c_v3.rs"]
pub mod i2c;
#[path = "../registers/iwdg_v00x.rs"]
pub mod iwdg;
#[path = "../registers/opa_v205.rs"]
pub mod opa;
#[path = "../registers/pfic_v205.rs"]
pub mod pfic;
#[path = "../registers/pioc_v205.rs"]
pub mod pioc;
#[path = "../registers/qspi_v205.rs"]
pub mod qspi;
#[path = "../registers/rcc_v205.rs"]
pub mod rcc;
#[path = "../registers/rtc_common.rs"]
pub mod rtc;
#[path = "../registers/spi_v0.rs"]
pub mod spi;
#[path = "../registers/systick_rv4.rs"]
pub mod systick;
#[path = "../registers/timer_v3b.rs"]
pub mod timer;
#[path = "../registers/tkey_v205.rs"]
pub mod tkey;
#[path = "../registers/usart_common.rs"]
pub mod usart;
#[path = "../registers/usb_v2fs.rs"]
pub mod usb;
#[path = "../registers/usbhs_v4x7.rs"]
pub mod usbhs;
#[path = "../registers/usbpd_l1.rs"]
pub mod usbpd;
#[path = "../registers/wwdg_v00x.rs"]
pub mod wwdg;
