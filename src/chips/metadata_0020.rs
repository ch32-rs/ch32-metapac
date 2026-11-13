
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "PFIC",
        address: 0xe000e000,
        registers: Some(PeripheralRegisters {
            kind: "pfic",
            version: "m030",
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
            version: "m030",
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
                signal: "EXTI7",
                interrupt: "EXTI7_0",
            },
            PeripheralInterrupt {
                signal: "EXTI8",
                interrupt: "EXTI15_8",
            },
            PeripheralInterrupt {
                signal: "EXTI15",
                interrupt: "EXTI15_8",
            },
        ],
    },
    Peripheral {
        name: "RCC",
        address: 0x40021000,
        registers: Some(PeripheralRegisters {
            kind: "rcc",
            version: "m030",
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
            version: "m030",
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
        name: "AFIO",
        address: 0x40010000,
        registers: Some(PeripheralRegisters {
            kind: "afio",
            version: "m030",
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
        name: "PWR",
        address: 0x40007000,
        registers: Some(PeripheralRegisters {
            kind: "pwr",
            version: "m030",
            block: "PWR",
            ir: &pwr::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
            kernel_clock: Clock("PCLK1"),
            enable: Some(PeripheralRccRegister {
                register: "PB1PCENR",
                field: "PWREN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
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
        name: "EXTEND",
        address: 0x40023800,
        registers: Some(PeripheralRegisters {
            kind: "extend",
            version: "m030",
            block: "EXTEN",
            ir: &extend::REGISTERS,
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
            reset: Some(PeripheralRccRegister {
                register: "PB1PRSTR",
                field: "WWDGRST",
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
            version: "m030",
            block: "ADTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK2",
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
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "TIM1_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PC1",
                signal: "ETR",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH2",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH3",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH4",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "BKIN",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1N",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH2N",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH3N",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "ETR",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH2",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH3",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH4",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "BKIN",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1N",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH2N",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH3N",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "ETR",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH2",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH3",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH4",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "BKIN",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1N",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH2N",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH3N",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "ETR",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH1",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CH2",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CH3",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PC5",
                signal: "CH4",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "BKIN",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH1N",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH2N",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH3N",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "ETR",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "CH1",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PB9",
                signal: "CH1",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "CH2",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "CH2",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "CH3",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH3",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CH4",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "BKIN",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "CH1N",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PB8",
                signal: "CH1N",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH2N",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "CH2N",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH3N",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH3N",
                remap: Some(4),
                af: None,
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
                channel: Some("DMA1_CH2"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH3",
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
                signal: "CH5",
                channel: Some("DMA1_CH5"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "UP",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BRK",
                interrupt: "TIM1BRK",
            },
            PeripheralInterrupt {
                signal: "UP",
                interrupt: "TIM1UP",
            },
            PeripheralInterrupt {
                signal: "TRG",
                interrupt: "TIM1TRG",
            },
            PeripheralInterrupt {
                signal: "COM",
                interrupt: "TIM1TRG",
            },
            PeripheralInterrupt {
                signal: "CC",
                interrupt: "TIM1CC",
            },
        ],
    },
    Peripheral {
        name: "TIM2",
        address: 0x40000000,
        registers: Some(PeripheralRegisters {
            kind: "timer",
            version: "m030",
            block: "GPTM",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
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
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "TIM2_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA5",
                signal: "CH1",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "ETR",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CH2",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH3",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CH1N",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH4",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH2N",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CH1",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "ETR",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CH2",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH1N",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH4",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH2N",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "ETR",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH3",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH1N",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH4",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH1",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "ETR",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH2",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH3",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CH1N",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH4",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH2N",
                remap: Some(3),
                af: None,
            },
        ],
        dma_channels: &[
            PeripheralDmaChannel {
                signal: "TRIG",
                channel: Some("DMA1_CH2"),
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
                signal: "CH4",
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
                signal: "CH2",
                channel: Some("DMA1_CH6"),
                dmamux: None,
                dma: None,
                request: None,
            },
            PeripheralDmaChannel {
                signal: "CH1",
                channel: Some("DMA1_CH7"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
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
            version: "m030",
            block: "GPTM3",
            ir: &timer::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "PCLK1",
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
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "TIM3_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PC0",
                signal: "CH1",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "ETR",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH2",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1N",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2N",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "CH1",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "ETR",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH2",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "CH1N",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2N",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CH1",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "ETR",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CH2",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CH1N",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2N",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CH1",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "ETR",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CH2",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CH1N",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CH2N",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "CH1",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "ETR",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "CH2",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "CH1N",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "CH2N",
                remap: Some(4),
                af: None,
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
                channel: Some("DMA1_CH2"),
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
                signal: "TRIG",
                channel: Some("DMA1_CH4"),
                dmamux: None,
                dma: None,
                request: None,
            },
        ],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TIM3",
        }],
    },
    Peripheral {
        name: "ADC1",
        address: 0x40012400,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "m030",
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
                field: "ADC1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "ADC_ETRGIN_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "IN13",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "IN14",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "IN15",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "IN16",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "IN5",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "IN6",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "IN2",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "IN7",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "IN8",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "IN19",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "IN18",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "IN9",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "IN10",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "IN11",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "IN12",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "IN0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "IN1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "IN17",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "IN3",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "IN4",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "EXT_TRG",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "EXT_TRG",
                remap: Some(1),
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
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
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "I2C1_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PB3",
                signal: "SCL",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "SDA",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SCL",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SDA",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "SCL",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "SDA",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "SCL",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "SDA",
                remap: Some(3),
                af: None,
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
        name: "UART1",
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
                field: "UART1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "PB2PRSTR",
                field: "UART1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "UART1_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PC1",
                signal: "TX",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PC0",
                signal: "RX",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CTS",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PC1",
                signal: "TX",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PC2",
                signal: "RX",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "CTS",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RTS",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "TX",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "RX",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "CTS",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "RTS",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "TX",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "RX",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CTS",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RTS",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "TX",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "RX",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CTS",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RTS",
                remap: Some(4),
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TX",
                remap: Some(5),
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "RX",
                remap: Some(5),
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CTS",
                remap: Some(5),
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "RTS",
                remap: Some(5),
                af: None,
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
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART1",
        }],
    },
    Peripheral {
        name: "SPI1",
        address: 0x40013000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "m030",
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
        remap: Some(PeripheralRemapRegister {
            register: "PCFR1",
            field: "SPI1_RM",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "NSS",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SCK",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "MISO",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PC3",
                signal: "MOSI",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "NSS",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "SCK",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "MISO",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "MOSI",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "NSS",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SCK",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PC4",
                signal: "MISO",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PA11",
                signal: "MOSI",
                remap: Some(2),
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "NSS",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "SCK",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "MISO",
                remap: Some(3),
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "MOSI",
                remap: Some(3),
                af: None,
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
        name: "USBFS",
        address: 0x40023400,
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
                pin: "PB0",
                signal: "DP",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DM",
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
                interrupt: "USBFSWAKEUP",
            },
        ],
    },
    Peripheral {
        name: "USBPD0",
        address: 0x40024000,
        registers: Some(PeripheralRegisters {
            kind: "usbpd",
            version: "m030",
            block: "USBPD",
            ir: &usbpd::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "USBPD0EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HBRSTR",
                field: "USBPD0RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "CC1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "CC2",
                remap: None,
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USBPD0",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "USBPD0WAKEUP",
            },
        ],
    },
    Peripheral {
        name: "USBPD1",
        address: 0x40024400,
        registers: Some(PeripheralRegisters {
            kind: "usbpd",
            version: "m030",
            block: "USBPD",
            ir: &usbpd::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "USBPD1EN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HBRSTR",
                field: "USBPD1RST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA2",
                signal: "CC1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "CC2",
                remap: None,
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "GLOBAL",
                interrupt: "USBPD1",
            },
            PeripheralInterrupt {
                signal: "WKUP",
                interrupt: "USBPD1WAKEUP",
            },
        ],
    },
    Peripheral {
        name: "OPA",
        address: 0x40024800,
        registers: Some(PeripheralRegisters {
            kind: "opa",
            version: "m030",
            block: "OPA",
            ir: &opa::REGISTERS,
        }),
        rcc: Some(PeripheralRcc {
            bus_clock: "HCLK",
            kernel_clock: Clock("HCLK"),
            enable: Some(PeripheralRccRegister {
                register: "HBPCENR",
                field: "OPAEN",
            }),
            reset: Some(PeripheralRccRegister {
                register: "HBRSTR",
                field: "OPARST",
            }),
            stop_mode: StopMode::Stop1,
        }),
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA3",
                signal: "CMP3_P0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "CMP3_N0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "CMP3_N1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "CMP3_N2",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "CMP3_OUT0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CMP3_P1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "CMP3_P2",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "CMP3_N3",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "CMP3_P3",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "CMP3_OUT1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CMP3_OUT2",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "CMP2_P0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "CMP2_N0",
                remap: None,
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "OPA",
        }],
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "WWDG",
        number: 16,
    },
    Interrupt {
        name: "OPA",
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
        name: "UART1",
        number: 32,
    },
    Interrupt {
        name: "EXTI15_8",
        number: 33,
    },
    Interrupt {
        name: "TIM1BRK",
        number: 34,
    },
    Interrupt {
        name: "TIM1UP",
        number: 35,
    },
    Interrupt {
        name: "TIM1TRG",
        number: 36,
    },
    Interrupt {
        name: "TIM1CC",
        number: 37,
    },
    Interrupt {
        name: "TIM2",
        number: 38,
    },
    Interrupt {
        name: "USBPD0",
        number: 39,
    },
    Interrupt {
        name: "USBPD0WAKEUP",
        number: 40,
    },
    Interrupt {
        name: "USBPD1",
        number: 41,
    },
    Interrupt {
        name: "USBPD1WAKEUP",
        number: 42,
    },
    Interrupt {
        name: "TIM3",
        number: 43,
    },
    Interrupt {
        name: "SPI1",
        number: 44,
    },
    Interrupt {
        name: "USBFS",
        number: 45,
    },
    Interrupt {
        name: "USBFSWAKEUP",
        number: 46,
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
#[path = "../registers/adc_m030.rs"]
pub mod adc;
#[path = "../registers/afio_m030.rs"]
pub mod afio;
#[path = "../registers/dma_m030.rs"]
pub mod dma;
#[path = "../registers/extend_m030.rs"]
pub mod extend;
#[path = "../registers/exti_common.rs"]
pub mod exti;
#[path = "../registers/flash_m030.rs"]
pub mod flash;
#[path = "../registers/gpio_v3.rs"]
pub mod gpio;
#[path = "../registers/i2c_v0.rs"]
pub mod i2c;
#[path = "../registers/opa_m030.rs"]
pub mod opa;
#[path = "../registers/pfic_m030.rs"]
pub mod pfic;
#[path = "../registers/pwr_m030.rs"]
pub mod pwr;
#[path = "../registers/rcc_m030.rs"]
pub mod rcc;
#[path = "../registers/spi_m030.rs"]
pub mod spi;
#[path = "../registers/systick_rv4.rs"]
pub mod systick;
#[path = "../registers/timer_m030.rs"]
pub mod timer;
#[path = "../registers/usart_common.rs"]
pub mod usart;
#[path = "../registers/usb_v2fs.rs"]
pub mod usb;
#[path = "../registers/usbpd_m030.rs"]
pub mod usbpd;
#[path = "../registers/wwdg_v00x.rs"]
pub mod wwdg;
