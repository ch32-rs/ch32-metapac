
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "PFIC",
        address: 0xe000e000,
        registers: Some(PeripheralRegisters {
            kind: "pfic",
            version: "h4",
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
        name: "AFIO",
        address: 0x40010000,
        registers: Some(PeripheralRegisters {
            kind: "afio",
            version: "h4",
            block: "AFIO",
            ir: &afio::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "AFIOEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
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
                register: "HB2PCENR",
                field: "IOPAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
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
                register: "HB2PCENR",
                field: "IOPBEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
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
                register: "HB2PCENR",
                field: "IOPCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
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
                register: "HB2PCENR",
                field: "IOPDEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
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
                register: "HB2PCENR",
                field: "IOPEEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
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
        name: "GPIOF",
        address: 0x40011c00,
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
                register: "HB2PCENR",
                field: "IOPFEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "IOPFRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
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
            version: "h4",
            block: "RCC",
            ir: &rcc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RCC",
        }],
    },
    Peripheral {
        name: "SYSTICK",
        address: 0xe000f000,
        registers: Some(PeripheralRegisters {
            kind: "systick",
            version: "v3f_v5f",
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
        name: "HSEM",
        address: 0xe000c000,
        registers: Some(PeripheralRegisters {
            kind: "hsem",
            version: "h4",
            block: "HSEM",
            ir: &hsem::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HSEM",
        }],
    },
    Peripheral {
        name: "IPC",
        address: 0xe000d000,
        registers: Some(PeripheralRegisters {
            kind: "ipc",
            version: "h4",
            block: "IPC",
            ir: &ipc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "CH0",
                interrupt: "IPC_CH0",
            },
            PeripheralInterrupt {
                signal: "CH1",
                interrupt: "IPC_CH1",
            },
            PeripheralInterrupt {
                signal: "CH2",
                interrupt: "IPC_CH2",
            },
            PeripheralInterrupt {
                signal: "CH3",
                interrupt: "IPC_CH3",
            },
        ],
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
        name: "WWDG",
        address: 0x40002c00,
        registers: Some(PeripheralRegisters {
            kind: "wwdg",
            version: "common",
            block: "WWDG",
            ir: &wwdg::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "WWDGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "WWDGRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "WWDG",
        }],
    },
    Peripheral {
        name: "IWDG",
        address: 0x40003000,
        registers: Some(PeripheralRegisters {
            kind: "iwdg",
            version: "h4",
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
        name: "PWR",
        address: 0x40007000,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "h4",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "PWREN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "PWRRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
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
            version: "h4",
            block: "FLASH",
            ir: &flash::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "FLASH",
        }],
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
        name: "ESIG",
        address: 0x1ffff7e0,
        registers: Some(PeripheralRegisters {
            kind: "esig",
            version: "h4",
            block: "ESIG",
            ir: &esig::REGISTERS,
        }),
        rcc: None,
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
            version: "h4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "USART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "USART1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA9",
                signal: "TX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "TX",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "TX",
                remap: None,
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "RX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "RX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "RX",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "RX",
                remap: None,
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CK",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CK",
                remap: None,
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CTS",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CTS",
                remap: None,
                af: Some(14),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "RTS",
                remap: None,
                af: Some(14),
            },
        ],
        dma_channels: &[],
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
            version: "h4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "USART2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "USART2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "TX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "RX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CK",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CK",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CTS",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CTS",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RTS",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "RTS",
                remap: None,
                af: Some(7),
            },
        ],
        dma_channels: &[],
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
            version: "h4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "USART3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "USART3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "TX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD8",
                signal: "TX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "RX",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "RX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD9",
                signal: "RX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CK",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CK",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CK",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "CK",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "CTS",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CTS",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CTS",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "RTS",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "RTS",
                remap: None,
                af: Some(7),
            },
        ],
        dma_channels: &[],
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
            version: "h4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "USART4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "USART4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PC6",
                signal: "TX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "TX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "RX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "RX",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CK",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF1",
                signal: "CK",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "CTS",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF0",
                signal: "CTS",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "RTS",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "RTS",
                remap: None,
                af: Some(7),
            },
        ],
        dma_channels: &[],
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
            version: "h4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "USART5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "USART5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PE0",
                signal: "TX",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "TX",
                remap: None,
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "RX",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "RX",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CK",
                remap: None,
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "CK",
                remap: None,
                af: Some(11),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "CTS",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "RTS",
                remap: None,
                af: Some(4),
            },
        ],
        dma_channels: &[],
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
            version: "h4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "USART6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "USART6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "TX",
                remap: None,
                af: Some(8),
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
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "TX",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "RX",
                remap: None,
                af: Some(8),
            },
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
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "RX",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "RX",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CK",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CK",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CK",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CK",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PE2",
                signal: "CK",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CTS",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CTS",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "RTS",
                remap: None,
                af: Some(8),
            },
        ],
        dma_channels: &[],
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
            version: "h4",
            block: "USART",
            ir: &usart::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "USART7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "USART7RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                remap: None,
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                remap: None,
                af: Some(14),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "TX",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                remap: None,
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RX",
                remap: None,
                af: Some(14),
            },
            PeripheralPin {
                pin: "PD2",
                signal: "RX",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CK",
                remap: None,
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CK",
                remap: None,
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "CK",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CTS",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "RTS",
                remap: None,
                af: Some(8),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USART7",
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
                register: "HB1PCENR",
                field: "I2C1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "I2C1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SDA",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
                remap: None,
                af: Some(4),
            },
        ],
        dma_channels: &[],
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
                register: "HB1PCENR",
                field: "I2C2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "I2C2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB11",
                signal: "SDA",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "SDA",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCL",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "SCL",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SMBA",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "SMBA",
                remap: None,
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C2_EV",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C2_ER",
            },
        ],
    },
    Peripheral {
        name: "I2C3",
        address: 0x40005c00,
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
                register: "HB1PCENR",
                field: "I2C3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "I2C3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA13",
                signal: "SDA",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "SDA",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "SCL",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SCL",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "SMBA",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SMBA",
                remap: None,
                af: Some(7),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C3_EV",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C3_ER",
            },
        ],
    },
    Peripheral {
        name: "I2C4",
        address: 0x40014000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "v3",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "I2C4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "I2C4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB7",
                signal: "SDA",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "SDA",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "SDA",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCL",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "SCL",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "SCL",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "SCL",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "SMBA",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "SMBA",
                remap: None,
                af: Some(11),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "SMBA",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PF11",
                signal: "SMBA",
                remap: None,
                af: Some(2),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "EV",
                interrupt: "I2C4_EV",
            },
            PeripheralInterrupt {
                signal: "ER",
                interrupt: "I2C4_ER",
            },
        ],
    },
    Peripheral {
        name: "LPTIM1",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "l1",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "LPTIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "LPTIM1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PD12",
                signal: "CH1",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "CH1",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CH2",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF5",
                signal: "CH2",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "ETR",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "ETR",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "OC",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "OC",
                remap: None,
                af: Some(1),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "LPTIM1",
            },
            PeripheralInterrupt {
                signal: "WAKEUP",
                interrupt: "LPTIM1WAKEUP",
            },
        ],
    },
    Peripheral {
        name: "LPTIM2",
        address: 0x40003400,
        registers: Some(PeripheralRegisters {
            kind: "lptim",
            version: "l1",
            block: "LPTIM",
            ir: &lptim::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "LPTIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "LPTIM2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB10",
                signal: "CH1",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CH1",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "CH2",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "ETR",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PD10",
                signal: "ETR",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "OC",
                remap: None,
                af: Some(3),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "LPTIM2",
            },
            PeripheralInterrupt {
                signal: "WAKEUP",
                interrupt: "LPTIM2WAKEUP",
            },
        ],
    },
    Peripheral {
        name: "RNG",
        address: 0x40023c00,
        registers: Some(PeripheralRegisters {
            kind: "rng",
            version: "h4",
            block: "RNG",
            ir: &rng::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "RNGEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HBRSTR",
                field: "RNGRST",
            }),
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
        name: "TIM1",
        address: 0x40012c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "h4",
            block: "ADTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "TIM1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
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
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE7",
                signal: "ETR",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "CH1",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH1",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE9",
                signal: "CH1",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA9",
                signal: "CH2",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE11",
                signal: "CH2",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA10",
                signal: "CH3",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "CH3",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "CH4",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CH4",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "BKIN",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "BKIN",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "BKIN2",
                remap: None,
                af: Some(12),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "BKIN2",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1N",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE8",
                signal: "CH1N",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE10",
                signal: "CH2N",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PE12",
                signal: "CH3N",
                remap: None,
                af: Some(1),
            },
        ],
        dma_channels: &[],
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
                signal: "TRG_COM",
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
            version: "h4",
            block: "GPTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "TIM2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "TIM2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
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
                pin: "PB11",
                signal: "CH4",
                remap: None,
                af: Some(1),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM2",
        }],
    },
    Peripheral {
        name: "TIM3",
        address: 0x40000400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "h4",
            block: "GPTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "TIM3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
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
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CH1",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "CH2",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH3",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "CH3",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH4",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CH4",
                remap: None,
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM3",
        }],
    },
    Peripheral {
        name: "TIM4",
        address: 0x40000800,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "h4",
            block: "GPTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "TIM4EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "TIM4RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB4",
                signal: "ETR",
                remap: None,
                af: Some(10),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CH1",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "CH1",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CH2",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "CH2",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CH3",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CH3",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CH4",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "CH4",
                remap: None,
                af: Some(2),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM4",
        }],
    },
    Peripheral {
        name: "TIM5",
        address: 0x40000c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "h4",
            block: "GPTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "TIM5EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "TIM5RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "ETR",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD11",
                signal: "ETR",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "CH1",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH2",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "CH2",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH3",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD14",
                signal: "CH3",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH4",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH4",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PD15",
                signal: "CH4",
                remap: None,
                af: Some(6),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM5",
        }],
    },
    Peripheral {
        name: "TIM6",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "h4",
            block: "BCTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "TIM6EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "TIM6RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM6",
        }],
    },
    Peripheral {
        name: "TIM7",
        address: 0x40001400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "h4",
            block: "BCTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "TIM7EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "TIM7RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM7",
        }],
    },
    Peripheral {
        name: "TIM8",
        address: 0x40013400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "h4",
            block: "ADTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "TIM8EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "TIM8RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "ETR",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PF2",
                signal: "ETR",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC6",
                signal: "CH1",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "CH1",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC7",
                signal: "CH2",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "CH2",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "CH3",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CH3",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH4",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "CH4",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "BKIN",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "BKIN",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC0",
                signal: "BKIN",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PA8",
                signal: "BKIN2",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "BKIN2",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CH1N",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH1N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2N",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH2N",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3N",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3N",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH3N",
                remap: None,
                af: Some(0),
            },
        ],
        dma_channels: &[],
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
                signal: "TRG_COM",
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
            version: "h4",
            block: "GPTM32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "TIM9EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "TIM9RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "ETR",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "ETR",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PC8",
                signal: "ETR",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CH1",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH1",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "CH1",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH2",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH2",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "CH2",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH3",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH3",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "CH3",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CH3",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH4",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "CH4",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "CH4",
                remap: None,
                af: Some(4),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM9",
        }],
    },
    Peripheral {
        name: "TIM10",
        address: 0x40015000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "h4",
            block: "GPTM32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "TIM10EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "TIM10RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "ETR",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PF10",
                signal: "ETR",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH1",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH1",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "CH1",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH2",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "CH2",
                remap: None,
                af: Some(0),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CH2",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH3",
                remap: None,
                af: Some(8),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH3",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH3",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH4",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH4",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "CH4",
                remap: None,
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM10",
        }],
    },
    Peripheral {
        name: "TIM11",
        address: 0x40015400,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "h4",
            block: "GPTM32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "TIM11EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "TIM11RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB2",
                signal: "ETR",
                remap: None,
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "ETR",
                remap: None,
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD3",
                signal: "CH1",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE0",
                signal: "CH1",
                remap: None,
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "CH1",
                remap: None,
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD4",
                signal: "CH2",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE1",
                signal: "CH2",
                remap: None,
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "CH2",
                remap: None,
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD5",
                signal: "CH3",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PD7",
                signal: "CH3",
                remap: None,
                af: Some(13),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "CH3",
                remap: None,
                af: Some(13),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "CH4",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "CH4",
                remap: None,
                af: Some(13),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM11",
        }],
    },
    Peripheral {
        name: "TIM12",
        address: 0x40013c00,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "h4",
            block: "GPTM32",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "TIM12EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "TIM12RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB0",
                signal: "ETR",
                remap: None,
                af: Some(13),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "ETR",
                remap: None,
                af: Some(14),
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH1",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE3",
                signal: "CH1",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH2",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PE4",
                signal: "CH2",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE13",
                signal: "CH2",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PE5",
                signal: "CH3",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE14",
                signal: "CH3",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF12",
                signal: "CH3",
                remap: None,
                af: Some(13),
            },
            PeripheralPin {
                pin: "PE6",
                signal: "CH4",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PE15",
                signal: "CH4",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF13",
                signal: "CH4",
                remap: None,
                af: Some(13),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM12",
        }],
    },
    Peripheral {
        name: "SPI1",
        address: 0x40013000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "h4",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "SPI1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "SPI1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
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
                pin: "PF5",
                signal: "SCK",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "SCK",
                remap: None,
                af: Some(3),
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
                pin: "PD7",
                signal: "MOSI",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF8",
                signal: "MOSI",
                remap: None,
                af: Some(3),
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
            PeripheralPin {
                pin: "PF3",
                signal: "MISO",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF9",
                signal: "MISO",
                remap: None,
                af: Some(3),
            },
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
                pin: "PF4",
                signal: "NSS",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "NSS",
                remap: None,
                af: Some(3),
            },
        ],
        dma_channels: &[],
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
            version: "h4",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "SPI2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "SPI2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA9",
                signal: "SCK",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "SCK",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB10",
                signal: "SCK",
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
                pin: "PD3",
                signal: "SCK",
                remap: None,
                af: Some(4),
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MOSI",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC1",
                signal: "MOSI",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC3",
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
            PeripheralPin {
                pin: "PC2",
                signal: "MISO",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PA11",
                signal: "NSS",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "NSS",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "NSS",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "NSS",
                remap: None,
                af: Some(5),
            },
        ],
        dma_channels: &[],
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
            version: "h4",
            block: "SPI",
            ir: &spi::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "SPI3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "SPI3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA14",
                signal: "SCK",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB3",
                signal: "SCK",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC10",
                signal: "SCK",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA13",
                signal: "MOSI",
                remap: None,
                af: Some(1),
            },
            PeripheralPin {
                pin: "PB2",
                signal: "MOSI",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                remap: None,
                af: Some(7),
            },
            PeripheralPin {
                pin: "PC12",
                signal: "MOSI",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD6",
                signal: "MOSI",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PB4",
                signal: "MISO",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PC9",
                signal: "MISO",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PC11",
                signal: "MISO",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA4",
                signal: "NSS",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PA15",
                signal: "NSS",
                remap: None,
                af: Some(6),
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI3",
        }],
    },
    Peripheral {
        name: "DMA1",
        address: 0x40020000,
        registers: Some(PeripheralRegisters {
            kind: "dma",
            version: "h4",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "DMA1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HBRSTR",
                field: "DMA1RST",
            }),
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
            version: "h4",
            block: "DMA",
            ir: &dma::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "DMA2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HBRSTR",
                field: "DMA2RST",
            }),
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
        name: "DMAMUX",
        address: 0x40020800,
        registers: Some(PeripheralRegisters {
            kind: "dmamux",
            version: "h4",
            block: "DMAMUX",
            ir: &dmamux::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "h4",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("ADC"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "ADC1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
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
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1_2",
        }],
    },
    Peripheral {
        name: "ADC2",
        address: 0x40012800,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "h4",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("ADC"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "ADC2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
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
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC1_2",
        }],
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
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "RTC",
            },
            PeripheralInterrupt {
                signal: "ALARM",
                interrupt: "RTCALARM",
            },
        ],
    },
    Peripheral {
        name: "DAC",
        address: 0x40007400,
        registers: Some(PeripheralRegisters {
            kind: "dac",
            version: "h4",
            block: "DAC",
            ir: &dac::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "DACEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
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
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "OUT2",
                remap: None,
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "HSADC",
        address: 0x40017400,
        registers: Some(PeripheralRegisters {
            kind: "hsadc",
            version: "h4",
            block: "HSADC",
            ir: &hsadc::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("HSADC"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "HSADCEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "HSADCRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "HSADC",
        }],
    },
    Peripheral {
        name: "OPA",
        address: 0x40017800,
        registers: Some(PeripheralRegisters {
            kind: "opa",
            version: "h4",
            block: "OPA",
            ir: &opa::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "OPCMEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "OPCMRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DFSDM",
        address: 0x40017000,
        registers: Some(PeripheralRegisters {
            kind: "dfsdm",
            version: "h4",
            block: "DFSDM",
            ir: &dfsdm::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
            kernel_clock: Clock("PCLK2"),
            enable: Some(PeripheralRccRegister {
                register: "HB2PCENR",
                field: "DFSDMEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB2PRSTR",
                field: "DFSDMRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "FLT0",
                interrupt: "DFSDM0",
            },
            PeripheralInterrupt {
                signal: "FLT1",
                interrupt: "DFSDM1",
            },
        ],
    },
    Peripheral {
        name: "USBHS",
        address: 0x40030000,
        registers: Some(PeripheralRegisters {
            kind: "usbhs",
            version: "h4",
            block: "USBHS",
            ir: &usbhs::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "USBHSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HBRSTR",
                field: "USBHSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB8",
                signal: "DP",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
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
                interrupt: "USBHSWAKEUP",
            },
        ],
    },
    Peripheral {
        name: "CAN1",
        address: 0x40006400,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "h4",
            block: "CAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "CAN1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "CAN1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "RX",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "RX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB8",
                signal: "RX",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD0",
                signal: "RX",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PA12",
                signal: "TX",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB7",
                signal: "TX",
                remap: None,
                af: Some(3),
            },
            PeripheralPin {
                pin: "PB9",
                signal: "TX",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PD1",
                signal: "TX",
                remap: None,
                af: Some(9),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "CAN1_TX",
            },
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "CAN1_RX0",
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
            version: "h4",
            block: "CAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "CAN2EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "CAN2RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RX",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                remap: None,
                af: Some(9),
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TX",
                remap: None,
                af: Some(9),
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
    Peripheral {
        name: "CAN3",
        address: 0x40007800,
        registers: Some(PeripheralRegisters {
            kind: "can",
            version: "h4",
            block: "CAN",
            ir: &can::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "HB1PCENR",
                field: "CAN3EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HB1PRSTR",
                field: "CAN3RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PC4",
                signal: "RX",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD12",
                signal: "RX",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF4",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF6",
                signal: "RX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PC5",
                signal: "TX",
                remap: None,
                af: Some(6),
            },
            PeripheralPin {
                pin: "PD13",
                signal: "TX",
                remap: None,
                af: Some(5),
            },
            PeripheralPin {
                pin: "PF3",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
            PeripheralPin {
                pin: "PF7",
                signal: "TX",
                remap: None,
                af: Some(2),
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "TX",
                interrupt: "CAN3_TX",
            },
            PeripheralInterrupt {
                signal: "RX0",
                interrupt: "CAN3_RX0",
            },
            PeripheralInterrupt {
                signal: "RX1",
                interrupt: "CAN3_RX1",
            },
            PeripheralInterrupt {
                signal: "SCE",
                interrupt: "CAN3_SCE",
            },
        ],
    },
    Peripheral {
        name: "USBSS",
        address: 0x40034000,
        registers: Some(PeripheralRegisters {
            kind: "usbss",
            version: "h4",
            block: "USBSS",
            ir: &usbss::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "USBSSEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HBRSTR",
                field: "USBSSRST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USBSS",
            },
            PeripheralInterrupt {
                signal: "LINK",
                interrupt: "USBSS_LINK",
            },
            PeripheralInterrupt {
                signal: "WAKEUP",
                interrupt: "USBSSWAKEUP",
            },
        ],
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "IPC_CH0",
        number: 16,
    },
    Interrupt {
        name: "IPC_CH1",
        number: 17,
    },
    Interrupt {
        name: "IPC_CH2",
        number: 18,
    },
    Interrupt {
        name: "IPC_CH3",
        number: 19,
    },
    Interrupt {
        name: "HSEM",
        number: 28,
    },
    Interrupt {
        name: "WWDG",
        number: 32,
    },
    Interrupt {
        name: "EXTI15_8",
        number: 33,
    },
    Interrupt {
        name: "FLASH",
        number: 34,
    },
    Interrupt {
        name: "RCC",
        number: 35,
    },
    Interrupt {
        name: "EXTI7_0",
        number: 36,
    },
    Interrupt {
        name: "SPI1",
        number: 37,
    },
    Interrupt {
        name: "DMA1_CH2",
        number: 38,
    },
    Interrupt {
        name: "DMA1_CH3",
        number: 39,
    },
    Interrupt {
        name: "DMA1_CH4",
        number: 40,
    },
    Interrupt {
        name: "DMA1_CH5",
        number: 41,
    },
    Interrupt {
        name: "DMA1_CH6",
        number: 42,
    },
    Interrupt {
        name: "DMA1_CH7",
        number: 43,
    },
    Interrupt {
        name: "DMA1_CH8",
        number: 44,
    },
    Interrupt {
        name: "USART2",
        number: 45,
    },
    Interrupt {
        name: "I2C1_EV",
        number: 46,
    },
    Interrupt {
        name: "I2C1_ER",
        number: 47,
    },
    Interrupt {
        name: "USART1",
        number: 48,
    },
    Interrupt {
        name: "SPI2",
        number: 49,
    },
    Interrupt {
        name: "SPI3",
        number: 50,
    },
    Interrupt {
        name: "SPI4",
        number: 51,
    },
    Interrupt {
        name: "I2C2_EV",
        number: 52,
    },
    Interrupt {
        name: "I2C2_ER",
        number: 53,
    },
    Interrupt {
        name: "USBPD",
        number: 54,
    },
    Interrupt {
        name: "USBPDWAKEUP",
        number: 55,
    },
    Interrupt {
        name: "USBHS",
        number: 56,
    },
    Interrupt {
        name: "DMA1_CH1",
        number: 57,
    },
    Interrupt {
        name: "CAN1_SCE",
        number: 58,
    },
    Interrupt {
        name: "CAN1_TX",
        number: 59,
    },
    Interrupt {
        name: "CAN1_RX0",
        number: 60,
    },
    Interrupt {
        name: "CAN1_RX1",
        number: 61,
    },
    Interrupt {
        name: "USBSS",
        number: 62,
    },
    Interrupt {
        name: "USBSS_LINK",
        number: 63,
    },
    Interrupt {
        name: "USBHSWAKEUP",
        number: 64,
    },
    Interrupt {
        name: "USBSSWAKEUP",
        number: 65,
    },
    Interrupt {
        name: "RTCALARM",
        number: 66,
    },
    Interrupt {
        name: "USBFS",
        number: 67,
    },
    Interrupt {
        name: "USBFSWAKEUP",
        number: 68,
    },
    Interrupt {
        name: "ADC1_2",
        number: 69,
    },
    Interrupt {
        name: "TIM1_BRK",
        number: 70,
    },
    Interrupt {
        name: "TIM1_UP",
        number: 71,
    },
    Interrupt {
        name: "TIM1_TRG_COM",
        number: 72,
    },
    Interrupt {
        name: "TIM1_CC",
        number: 73,
    },
    Interrupt {
        name: "TIM2",
        number: 74,
    },
    Interrupt {
        name: "TIM3",
        number: 75,
    },
    Interrupt {
        name: "TIM4",
        number: 76,
    },
    Interrupt {
        name: "TIM5",
        number: 77,
    },
    Interrupt {
        name: "I2C3_EV",
        number: 78,
    },
    Interrupt {
        name: "I2C3_ER",
        number: 79,
    },
    Interrupt {
        name: "I2C4_EV",
        number: 80,
    },
    Interrupt {
        name: "I2C4_ER",
        number: 81,
    },
    Interrupt {
        name: "QSPI1",
        number: 82,
    },
    Interrupt {
        name: "SERDES",
        number: 83,
    },
    Interrupt {
        name: "USART3",
        number: 84,
    },
    Interrupt {
        name: "USART4",
        number: 85,
    },
    Interrupt {
        name: "TIM8_BRK",
        number: 86,
    },
    Interrupt {
        name: "TIM8_UP",
        number: 87,
    },
    Interrupt {
        name: "TIM8_TRG_COM",
        number: 88,
    },
    Interrupt {
        name: "TIM8_CC",
        number: 89,
    },
    Interrupt {
        name: "TIM9",
        number: 90,
    },
    Interrupt {
        name: "TIM10",
        number: 91,
    },
    Interrupt {
        name: "TIM11",
        number: 92,
    },
    Interrupt {
        name: "TIM12",
        number: 93,
    },
    Interrupt {
        name: "FMC",
        number: 94,
    },
    Interrupt {
        name: "SDMMC",
        number: 95,
    },
    Interrupt {
        name: "LPTIM1",
        number: 96,
    },
    Interrupt {
        name: "LPTIM2",
        number: 97,
    },
    Interrupt {
        name: "USART5",
        number: 98,
    },
    Interrupt {
        name: "USART6",
        number: 99,
    },
    Interrupt {
        name: "TIM6",
        number: 100,
    },
    Interrupt {
        name: "TIM7",
        number: 101,
    },
    Interrupt {
        name: "DMA2_CH1",
        number: 102,
    },
    Interrupt {
        name: "DMA2_CH2",
        number: 103,
    },
    Interrupt {
        name: "DMA2_CH3",
        number: 104,
    },
    Interrupt {
        name: "DMA2_CH4",
        number: 105,
    },
    Interrupt {
        name: "DMA2_CH5",
        number: 106,
    },
    Interrupt {
        name: "DMA2_CH6",
        number: 107,
    },
    Interrupt {
        name: "DMA2_CH7",
        number: 108,
    },
    Interrupt {
        name: "DMA2_CH8",
        number: 109,
    },
    Interrupt {
        name: "ETH",
        number: 110,
    },
    Interrupt {
        name: "ETH_WKUP",
        number: 111,
    },
    Interrupt {
        name: "CAN2_SCE",
        number: 112,
    },
    Interrupt {
        name: "CAN2_TX",
        number: 113,
    },
    Interrupt {
        name: "CAN2_RX0",
        number: 114,
    },
    Interrupt {
        name: "CAN2_RX1",
        number: 115,
    },
    Interrupt {
        name: "USART7",
        number: 116,
    },
    Interrupt {
        name: "USART8",
        number: 117,
    },
    Interrupt {
        name: "I3C_EV",
        number: 118,
    },
    Interrupt {
        name: "I3C_ER",
        number: 119,
    },
    Interrupt {
        name: "DVP",
        number: 120,
    },
    Interrupt {
        name: "ECDC",
        number: 121,
    },
    Interrupt {
        name: "PIOC",
        number: 122,
    },
    Interrupt {
        name: "SAI",
        number: 123,
    },
    Interrupt {
        name: "LTDC",
        number: 124,
    },
    Interrupt {
        name: "GPHA",
        number: 125,
    },
    Interrupt {
        name: "DFSDM0",
        number: 127,
    },
    Interrupt {
        name: "DFSDM1",
        number: 128,
    },
    Interrupt {
        name: "SWPMI",
        number: 131,
    },
    Interrupt {
        name: "QSPI2",
        number: 134,
    },
    Interrupt {
        name: "SWPMIWAKEUP",
        number: 135,
    },
    Interrupt {
        name: "CAN3_SCE",
        number: 136,
    },
    Interrupt {
        name: "CAN3_TX",
        number: 137,
    },
    Interrupt {
        name: "CAN3_RX0",
        number: 138,
    },
    Interrupt {
        name: "CAN3_RX1",
        number: 139,
    },
    Interrupt {
        name: "LPTIM2WAKEUP",
        number: 140,
    },
    Interrupt {
        name: "LPTIM1WAKEUP",
        number: 141,
    },
    Interrupt {
        name: "I3CWAKEUP",
        number: 142,
    },
    Interrupt {
        name: "RTC",
        number: 143,
    },
    Interrupt {
        name: "HSADC",
        number: 144,
    },
    Interrupt {
        name: "UHSIF",
        number: 145,
    },
    Interrupt {
        name: "RNG",
        number: 146,
    },
    Interrupt {
        name: "SDIO",
        number: 147,
    },
    Interrupt {
        name: "USARTWAKEUP",
        number: 148,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
#[path = "../registers/adc_h4.rs"]
pub mod adc;
#[path = "../registers/afio_h4.rs"]
pub mod afio;
#[path = "../registers/can_h4.rs"]
pub mod can;
#[path = "../registers/crc_h4.rs"]
pub mod crc;
#[path = "../registers/dac_h4.rs"]
pub mod dac;
#[path = "../registers/dfsdm_h4.rs"]
pub mod dfsdm;
#[path = "../registers/dma_h4.rs"]
pub mod dma;
#[path = "../registers/dmamux_h4.rs"]
pub mod dmamux;
#[path = "../registers/esig_h4.rs"]
pub mod esig;
#[path = "../registers/exti_common.rs"]
pub mod exti;
#[path = "../registers/flash_h4.rs"]
pub mod flash;
#[path = "../registers/gpio_v3.rs"]
pub mod gpio;
#[path = "../registers/hsadc_h4.rs"]
pub mod hsadc;
#[path = "../registers/hsem_h4.rs"]
pub mod hsem;
#[path = "../registers/i2c_v3.rs"]
pub mod i2c;
#[path = "../registers/ipc_h4.rs"]
pub mod ipc;
#[path = "../registers/iwdg_h4.rs"]
pub mod iwdg;
#[path = "../registers/lptim_l1.rs"]
pub mod lptim;
#[path = "../registers/opa_h4.rs"]
pub mod opa;
#[path = "../registers/pfic_h4.rs"]
pub mod pfic;
#[path = "../registers/pwr_h4.rs"]
pub mod pwr;
#[path = "../registers/rcc_h4.rs"]
pub mod rcc;
#[path = "../registers/rng_h4.rs"]
pub mod rng;
#[path = "../registers/rtc_common.rs"]
pub mod rtc;
#[path = "../registers/spi_h4.rs"]
pub mod spi;
#[path = "../registers/systick_v3f_v5f.rs"]
pub mod systick;
#[path = "../registers/timer_h4.rs"]
pub mod timer;
#[path = "../registers/usart_h4.rs"]
pub mod usart;
#[path = "../registers/usbhs_h4.rs"]
pub mod usbhs;
#[path = "../registers/usbss_h4.rs"]
pub mod usbss;
#[path = "../registers/wwdg_common.rs"]
pub mod wwdg;
